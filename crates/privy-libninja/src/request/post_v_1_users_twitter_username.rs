use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::post_v1_users_twitter_username`].

On request success, this will return a [`User`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostV1UsersTwitterUsernameRequest {
    pub privy_app_id: String,
    pub username: String,
}
impl FluentRequest<'_, PostV1UsersTwitterUsernameRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostV1UsersTwitterUsernameRequest> {
    type Output = httpclient::InMemoryResult<crate::model::User>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/users/twitter/username";
            let mut r = self.client.client.post(url);
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = r.json(serde_json::json!({ "username" : self.params.username }));
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Lookup User by Twitter Username

Looks up a user by their Twitter username.*/
    pub fn post_v1_users_twitter_username(
        &self,
        privy_app_id: &str,
        username: &str,
    ) -> FluentRequest<'_, PostV1UsersTwitterUsernameRequest> {
        FluentRequest {
            client: self,
            params: PostV1UsersTwitterUsernameRequest {
                privy_app_id: privy_app_id.to_owned(),
                username: username.to_owned(),
            },
        }
    }
}
