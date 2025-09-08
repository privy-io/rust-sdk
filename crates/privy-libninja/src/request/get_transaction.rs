use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::get_transaction`].

On request success, this will return a [`Transaction`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTransactionRequest {
    pub privy_app_id: String,
    pub transaction_id: String,
}
impl FluentRequest<'_, GetTransactionRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetTransactionRequest> {
    type Output = httpclient::InMemoryResult<crate::model::Transaction>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/transactions/{transaction_id}", transaction_id = self.params
                .transaction_id
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
    /**Get Transaction

Get a transaction by transaction ID.*/
    pub fn get_transaction(
        &self,
        privy_app_id: &str,
        transaction_id: &str,
    ) -> FluentRequest<'_, GetTransactionRequest> {
        FluentRequest {
            client: self,
            params: GetTransactionRequest {
                privy_app_id: privy_app_id.to_owned(),
                transaction_id: transaction_id.to_owned(),
            },
        }
    }
}
