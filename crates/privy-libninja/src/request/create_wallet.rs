use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::WalletAdditionalSigner;
/**You should use this struct via [`PrivyLibninjaClient::create_wallet`].

On request success, this will return a [`Wallet`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWalletRequest {
    pub body: Vec<WalletAdditionalSigner>,
    pub privy_app_id: String,
    pub privy_idempotency_key: Option<String>,
}
impl FluentRequest<'_, CreateWalletRequest> {
    ///Set the value of the privy_idempotency_key field.
    pub fn privy_idempotency_key(mut self, privy_idempotency_key: &str) -> Self {
        self.params.privy_idempotency_key = Some(privy_idempotency_key.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreateWalletRequest> {
    type Output = httpclient::InMemoryResult<crate::model::Wallet>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/wallets";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "body" : self.params.body }));
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            if let Some(ref unwrapped) = self.params.privy_idempotency_key {
                r = r.header("privy-idempotency-key", &unwrapped.to_string());
            }
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Create wallet

Create a new wallet.*/
    pub fn create_wallet(
        &self,
        body: Vec<WalletAdditionalSigner>,
        privy_app_id: &str,
    ) -> FluentRequest<'_, CreateWalletRequest> {
        FluentRequest {
            client: self,
            params: CreateWalletRequest {
                body,
                privy_app_id: privy_app_id.to_owned(),
                privy_idempotency_key: None,
            },
        }
    }
}
