use progenitor_client::{ClientHooks, OperationInfo};

use crate::{
    AuthorizationContext, Method, PRIVY_AUTHORIZATION_HEADER, generated::Client,
    sign_canonical_request,
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

                let signature = sign_canonical_request(&ctx, &app_id, method, url, data, None)
                    .await
                    .expect("request is valid json");

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
