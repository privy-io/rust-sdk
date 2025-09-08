use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::raw_sign`].

On request success, this will return a [`RawSignResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawSignRequest {
    pub params: serde_json::Value,
    pub privy_app_id: String,
    pub privy_authorization_signature: Option<String>,
    pub privy_idempotency_key: Option<String>,
    pub wallet_id: String,
}
impl FluentRequest<'_, RawSignRequest> {
    ///Set the value of the privy_authorization_signature field.
    pub fn privy_authorization_signature(
        mut self,
        privy_authorization_signature: &str,
    ) -> Self {
        self
            .params
            .privy_authorization_signature = Some(
            privy_authorization_signature.to_owned(),
        );
        self
    }
    ///Set the value of the privy_idempotency_key field.
    pub fn privy_idempotency_key(mut self, privy_idempotency_key: &str) -> Self {
        self.params.privy_idempotency_key = Some(privy_idempotency_key.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, RawSignRequest> {
    type Output = httpclient::InMemoryResult<crate::model::RawSignResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/wallets/{wallet_id}/raw_sign", wallet_id = self.params.wallet_id
            );
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "params" : self.params.params }));
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            if let Some(ref unwrapped) = self.params.privy_authorization_signature {
                r = r.header("privy-authorization-signature", &unwrapped.to_string());
            }
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
    /**Raw sign

Sign a message with a wallet by wallet ID.*/
    pub fn raw_sign(
        &self,
        params: serde_json::Value,
        privy_app_id: &str,
        wallet_id: &str,
    ) -> FluentRequest<'_, RawSignRequest> {
        FluentRequest {
            client: self,
            params: RawSignRequest {
                params,
                privy_app_id: privy_app_id.to_owned(),
                privy_authorization_signature: None,
                privy_idempotency_key: None,
                wallet_id: wallet_id.to_owned(),
            },
        }
    }
}
