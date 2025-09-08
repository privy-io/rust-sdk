use base64::{Engine, engine::general_purpose::STANDARD};
use futures::TryStreamExt;
use progenitor_client::{ClientHooks, OperationInfo};

use crate::{
    AuthorizationContext, Method, PRIVY_AUTHORIZATION_HEADER, build_canonical_request,
    generated::Client,
};

#[derive(Debug, Clone)]
pub struct MiddlewareState {
    pub app_id: String,
    pub ctx: AuthorizationContext,
}

impl ClientHooks<MiddlewareState> for Client {
    async fn exec(
        &self,
        mut request: reqwest::Request,
        i: &OperationInfo,
    ) -> reqwest::Result<reqwest::Response> {
        match (
            // only attach the sig for supported methods
            Method::try_from(request.method()),
            // grab the body if it exists and is valid json
            request
                .body()
                .as_ref()
                .and_then(|b| b.as_bytes())
                .and_then(|b| serde_json::from_slice::<serde_json::Value>(b).ok()),
            i.operation_id,
        ) {
            (_, _, "authenticate") => {} // ignore, recursion
            (Ok(method), Some(data), _) => {
                let url = request.url().to_string();
                let app_id = self.inner.app_id.clone();
                let ctx = self.inner.ctx.clone();

                let canonical = build_canonical_request(app_id, method, url, data, None)
                    .expect("request is valid json");

                tracing::info!("canonical request data: {}", canonical);

                let signature = ctx
                    .sign(canonical.as_bytes())
                    .map_ok(|s| {
                        let der_bytes = s.to_der();
                        STANDARD.encode(&der_bytes)
                    })
                    .try_collect::<Vec<_>>()
                    .await
                    .map_err(|e| {
                        tracing::error!("failed to sign request: {}", e);
                        todo!()
                    })?
                    .join(",");

                request.headers_mut().insert(
                    PRIVY_AUTHORIZATION_HEADER,
                    reqwest::header::HeaderValue::from_str(&signature)
                        .expect("base64 is inside the visible ascii range"),
                );
            }
            _ => {}
        };

        tracing::debug!("sending request: {:?}", request);

        self.client.execute(request).await
    }
}
