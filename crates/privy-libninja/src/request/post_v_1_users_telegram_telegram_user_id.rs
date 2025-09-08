use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::post_v1_users_telegram_telegram_user_id`].

On request success, this will return a [`User`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostV1UsersTelegramTelegramUserIdRequest {
    pub privy_app_id: String,
    pub telegram_user_id: String,
}
impl FluentRequest<'_, PostV1UsersTelegramTelegramUserIdRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostV1UsersTelegramTelegramUserIdRequest> {
    type Output = httpclient::InMemoryResult<crate::model::User>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/users/telegram/telegram_user_id";
            let mut r = self.client.client.post(url);
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = r
                .json(
                    serde_json::json!(
                        { "telegram_user_id" : self.params.telegram_user_id }
                    ),
                );
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Lookup User by Telegram User ID

Looks up a user by their Telegram user ID.*/
    pub fn post_v1_users_telegram_telegram_user_id(
        &self,
        privy_app_id: &str,
        telegram_user_id: &str,
    ) -> FluentRequest<'_, PostV1UsersTelegramTelegramUserIdRequest> {
        FluentRequest {
            client: self,
            params: PostV1UsersTelegramTelegramUserIdRequest {
                privy_app_id: privy_app_id.to_owned(),
                telegram_user_id: telegram_user_id.to_owned(),
            },
        }
    }
}
