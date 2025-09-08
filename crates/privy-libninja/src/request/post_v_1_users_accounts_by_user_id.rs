use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::post_v1_users_accounts_by_user_id`].

On request success, this will return a [`User`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostV1UsersAccountsByUserIdRequest {
    pub body: serde_json::Value,
    pub privy_app_id: String,
    pub user_id: String,
}
impl FluentRequest<'_, PostV1UsersAccountsByUserIdRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostV1UsersAccountsByUserIdRequest> {
    type Output = httpclient::InMemoryResult<crate::model::User>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/users/{user_id}/accounts", user_id = self.params.user_id
            );
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "body" : self.params.body }));
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Add or update a user linked account

Adds or updates a linked account for a user. This endpoint is not yet available to all users.*/
    pub fn post_v1_users_accounts_by_user_id(
        &self,
        body: serde_json::Value,
        privy_app_id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, PostV1UsersAccountsByUserIdRequest> {
        FluentRequest {
            client: self,
            params: PostV1UsersAccountsByUserIdRequest {
                body,
                privy_app_id: privy_app_id.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
}
