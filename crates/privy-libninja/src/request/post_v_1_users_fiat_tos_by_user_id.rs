use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::post_v1_users_fiat_tos_by_user_id`].

On request success, this will return a [`PostV1UsersFiatTosByUserIdResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostV1UsersFiatTosByUserIdRequest {
    pub privy_app_id: String,
    pub provider: String,
    pub user_id: String,
}
impl FluentRequest<'_, PostV1UsersFiatTosByUserIdRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostV1UsersFiatTosByUserIdRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::PostV1UsersFiatTosByUserIdResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/users/{user_id}/fiat/tos", user_id = self.params.user_id
            );
            let mut r = self.client.client.post(url);
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = r.json(serde_json::json!({ "provider" : self.params.provider }));
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Create a terms of service agreement for a user

Creates a terms of service agreement for a user*/
    pub fn post_v1_users_fiat_tos_by_user_id(
        &self,
        privy_app_id: &str,
        provider: &str,
        user_id: &str,
    ) -> FluentRequest<'_, PostV1UsersFiatTosByUserIdRequest> {
        FluentRequest {
            client: self,
            params: PostV1UsersFiatTosByUserIdRequest {
                privy_app_id: privy_app_id.to_owned(),
                provider: provider.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
}
