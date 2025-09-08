use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::get_wallets`].

On request success, this will return a [`GetWalletsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWalletsRequest {
    pub chain_type: Option<serde_json::Value>,
    pub cursor: Option<String>,
    pub limit: Option<f64>,
    pub privy_app_id: String,
    pub user_id: Option<String>,
}
impl FluentRequest<'_, GetWalletsRequest> {
    ///Set the value of the chain_type field.
    pub fn chain_type(mut self, chain_type: serde_json::Value) -> Self {
        self.params.chain_type = Some(chain_type);
        self
    }
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
    ///Set the value of the user_id field.
    pub fn user_id(mut self, user_id: &str) -> Self {
        self.params.user_id = Some(user_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetWalletsRequest> {
    type Output = httpclient::InMemoryResult<crate::model::GetWalletsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/wallets";
            let mut r = self.client.client.get(url);
            if let Some(ref unwrapped) = self.params.chain_type {
                r = r.query("chain_type", &unwrapped.to_string());
            }
            if let Some(ref unwrapped) = self.params.cursor {
                r = r.query("cursor", &unwrapped.to_string());
            }
            if let Some(ref unwrapped) = self.params.limit {
                r = r.query("limit", &unwrapped.to_string());
            }
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            if let Some(ref unwrapped) = self.params.user_id {
                r = r.query("user_id", &unwrapped.to_string());
            }
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Get all wallets

Get all wallets in your app.*/
    pub fn get_wallets(
        &self,
        privy_app_id: &str,
    ) -> FluentRequest<'_, GetWalletsRequest> {
        FluentRequest {
            client: self,
            params: GetWalletsRequest {
                chain_type: None,
                cursor: None,
                limit: None,
                privy_app_id: privy_app_id.to_owned(),
                user_id: None,
            },
        }
    }
}
