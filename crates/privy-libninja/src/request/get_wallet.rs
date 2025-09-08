use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::get_wallet`].

On request success, this will return a [`Wallet`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWalletRequest {
    pub privy_app_id: String,
    pub wallet_id: String,
}
impl FluentRequest<'_, GetWalletRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetWalletRequest> {
    type Output = httpclient::InMemoryResult<crate::model::Wallet>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/wallets/{wallet_id}", wallet_id = self.params.wallet_id
            );
            let mut r = self.client.client.get(url);
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Get wallet

Get a wallet by wallet ID.*/
    pub fn get_wallet(
        &self,
        privy_app_id: &str,
        wallet_id: &str,
    ) -> FluentRequest<'_, GetWalletRequest> {
        FluentRequest {
            client: self,
            params: GetWalletRequest {
                privy_app_id: privy_app_id.to_owned(),
                wallet_id: wallet_id.to_owned(),
            },
        }
    }
}
