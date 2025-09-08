use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::wallet_import_init`].

On request success, this will return a [`WalletImportInitResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletImportInitRequest {
    pub body: serde_json::Value,
    pub privy_app_id: String,
}
impl FluentRequest<'_, WalletImportInitRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, WalletImportInitRequest> {
    type Output = httpclient::InMemoryResult<crate::model::WalletImportInitResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/wallets/import/init";
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
    /**Initialize import

Initialize a wallet import. Complete by submitting the import.*/
    pub fn wallet_import_init(
        &self,
        body: serde_json::Value,
        privy_app_id: &str,
    ) -> FluentRequest<'_, WalletImportInitRequest> {
        FluentRequest {
            client: self,
            params: WalletImportInitRequest {
                body,
                privy_app_id: privy_app_id.to_owned(),
            },
        }
    }
}
