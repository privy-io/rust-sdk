use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::post_v1_users_custom_auth_id`].

On request success, this will return a [`User`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostV1UsersCustomAuthIdRequest {
    pub custom_user_id: String,
    pub privy_app_id: String,
}
impl FluentRequest<'_, PostV1UsersCustomAuthIdRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostV1UsersCustomAuthIdRequest> {
    type Output = httpclient::InMemoryResult<crate::model::User>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/users/custom_auth/id";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!({ "custom_user_id" : self.params.custom_user_id }),
                );
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Lookup User by Custom Auth ID

Looks up a user by their custom auth ID.*/
    pub fn post_v1_users_custom_auth_id(
        &self,
        custom_user_id: &str,
        privy_app_id: &str,
    ) -> FluentRequest<'_, PostV1UsersCustomAuthIdRequest> {
        FluentRequest {
            client: self,
            params: PostV1UsersCustomAuthIdRequest {
                custom_user_id: custom_user_id.to_owned(),
                privy_app_id: privy_app_id.to_owned(),
            },
        }
    }
}
