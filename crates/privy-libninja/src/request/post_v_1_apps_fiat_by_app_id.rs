use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::post_v1_apps_fiat_by_app_id`].

On request success, this will return a [`PostV1AppsFiatByAppIdResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostV1AppsFiatByAppIdRequest {
    pub api_key: String,
    pub app_id: String,
    pub privy_app_id: String,
    pub provider: String,
}
pub struct PostV1AppsFiatByAppIdRequired<'a> {
    pub api_key: &'a str,
    pub app_id: &'a str,
    pub privy_app_id: &'a str,
    pub provider: &'a str,
}
impl FluentRequest<'_, PostV1AppsFiatByAppIdRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostV1AppsFiatByAppIdRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::PostV1AppsFiatByAppIdResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/apps/{app_id}/fiat", app_id = self.params.app_id);
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "api_key" : self.params.api_key }));
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = r.json(serde_json::json!({ "provider" : self.params.provider }));
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Configure app for fiat onramping and offramping.

Updates the app configuration for the specified onramp provider. This is used to set up the app for fiat onramping and offramping.*/
    pub fn post_v1_apps_fiat_by_app_id(
        &self,
        args: PostV1AppsFiatByAppIdRequired,
    ) -> FluentRequest<'_, PostV1AppsFiatByAppIdRequest> {
        FluentRequest {
            client: self,
            params: PostV1AppsFiatByAppIdRequest {
                api_key: args.api_key.to_owned(),
                app_id: args.app_id.to_owned(),
                privy_app_id: args.privy_app_id.to_owned(),
                provider: args.provider.to_owned(),
            },
        }
    }
}
