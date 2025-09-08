use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::get_wallet_balance`].

On request success, this will return a [`GetWalletBalanceResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWalletBalanceRequest {
    pub asset: serde_json::Value,
    pub chain: serde_json::Value,
    pub include_currency: Option<String>,
    pub privy_app_id: String,
    pub wallet_id: String,
}
pub struct GetWalletBalanceRequired<'a> {
    pub asset: serde_json::Value,
    pub chain: serde_json::Value,
    pub privy_app_id: &'a str,
    pub wallet_id: &'a str,
}
impl FluentRequest<'_, GetWalletBalanceRequest> {
    ///Set the value of the include_currency field.
    pub fn include_currency(mut self, include_currency: &str) -> Self {
        self.params.include_currency = Some(include_currency.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetWalletBalanceRequest> {
    type Output = httpclient::InMemoryResult<crate::model::GetWalletBalanceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/wallets/{wallet_id}/balance", wallet_id = self.params.wallet_id
            );
            let mut r = self.client.client.get(url);
            r = r.query("asset", &self.params.asset.to_string());
            r = r.query("chain", &self.params.chain.to_string());
            if let Some(ref unwrapped) = self.params.include_currency {
                r = r.query("include_currency", &unwrapped.to_string());
            }
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Get balance

Get the balance of a wallet by wallet ID.*/
    pub fn get_wallet_balance(
        &self,
        args: GetWalletBalanceRequired,
    ) -> FluentRequest<'_, GetWalletBalanceRequest> {
        FluentRequest {
            client: self,
            params: GetWalletBalanceRequest {
                asset: args.asset,
                chain: args.chain,
                include_currency: None,
                privy_app_id: args.privy_app_id.to_owned(),
                wallet_id: args.wallet_id.to_owned(),
            },
        }
    }
}
