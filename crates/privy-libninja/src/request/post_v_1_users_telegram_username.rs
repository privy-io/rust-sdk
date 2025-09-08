use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::post_v1_users_telegram_username`].

On request success, this will return a [`User`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostV1UsersTelegramUsernameRequest {
    pub privy_app_id: String,
    pub username: String,
}
impl FluentRequest<'_, PostV1UsersTelegramUsernameRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostV1UsersTelegramUsernameRequest> {
    type Output = httpclient::InMemoryResult<crate::model::User>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/users/telegram/username";
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
    /**Lookup User by Telegram Username

Looks up a user by their Telegram username.*/
    pub fn post_v1_users_telegram_username(
        &self,
        privy_app_id: &str,
        username: &str,
    ) -> FluentRequest<'_, PostV1UsersTelegramUsernameRequest> {
        FluentRequest {
            client: self,
            params: PostV1UsersTelegramUsernameRequest {
                privy_app_id: privy_app_id.to_owned(),
                username: username.to_owned(),
            },
        }
    }
}
