use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::wallet_transactions`].

On request success, this will return a [`WalletTransactionsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransactionsRequest {
    pub asset: serde_json::Value,
    pub chain: String,
    pub cursor: Option<String>,
    pub limit: Option<f64>,
    pub privy_app_id: String,
    pub wallet_id: String,
}
pub struct WalletTransactionsRequired<'a> {
    pub asset: serde_json::Value,
    pub chain: &'a str,
    pub privy_app_id: &'a str,
    pub wallet_id: &'a str,
}
impl FluentRequest<'_, WalletTransactionsRequest> {
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
    ///Set the value of the limit field.
    pub fn limit(mut self, limit: f64) -> Self {
        self.params.limit = Some(limit);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, WalletTransactionsRequest> {
    type Output = httpclient::InMemoryResult<crate::model::WalletTransactionsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/wallets/{wallet_id}/transactions", wallet_id = self.params.wallet_id
            );
            let mut r = self.client.client.get(url);
            r = r.query("asset", &self.params.asset.to_string());
            r = r.query("chain", &self.params.chain.to_string());
            if let Some(ref unwrapped) = self.params.cursor {
                r = r.query("cursor", &unwrapped.to_string());
            }
            if let Some(ref unwrapped) = self.params.limit {
                r = r.query("limit", &unwrapped.to_string());
            }
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Get transactions

Get incoming and outgoing transactions of a wallet by wallet ID.*/
    pub fn wallet_transactions(
        &self,
        args: WalletTransactionsRequired,
    ) -> FluentRequest<'_, WalletTransactionsRequest> {
        FluentRequest {
            client: self,
            params: WalletTransactionsRequest {
                asset: args.asset,
                chain: args.chain.to_owned(),
                cursor: None,
                limit: None,
                privy_app_id: args.privy_app_id.to_owned(),
                wallet_id: args.wallet_id.to_owned(),
            },
        }
    }
}
