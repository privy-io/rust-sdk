use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::create_user_wallet`].

On request success, this will return a [`User`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserWalletRequest {
    pub privy_app_id: String,
    pub user_id: String,
    pub wallets: Vec<serde_json::Value>,
}
impl FluentRequest<'_, CreateUserWalletRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreateUserWalletRequest> {
    type Output = httpclient::InMemoryResult<crate::model::User>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/users/{user_id}/wallets", user_id = self.params.user_id
            );
            let mut r = self.client.client.post(url);
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = r.json(serde_json::json!({ "wallets" : self.params.wallets }));
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Create Embedded Wallet

Creates an embedded wallet for an existing user.*/
    pub fn create_user_wallet(
        &self,
        privy_app_id: &str,
        user_id: &str,
        wallets: Vec<serde_json::Value>,
    ) -> FluentRequest<'_, CreateUserWalletRequest> {
        FluentRequest {
            client: self,
            params: CreateUserWalletRequest {
                privy_app_id: privy_app_id.to_owned(),
                user_id: user_id.to_owned(),
                wallets,
            },
        }
    }
}
