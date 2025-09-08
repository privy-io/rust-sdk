use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::HpkeEncryption;
/**You should use this struct via [`PrivyLibninjaClient::wallet_export`].

On request success, this will return a [`WalletExportResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletExportRequest {
    pub encryption_type: HpkeEncryption,
    pub privy_app_id: String,
    pub privy_authorization_signature: Option<String>,
    pub recipient_public_key: String,
    pub wallet_id: String,
}
pub struct WalletExportRequired<'a> {
    pub encryption_type: HpkeEncryption,
    pub privy_app_id: &'a str,
    pub recipient_public_key: &'a str,
    pub wallet_id: &'a str,
}
impl FluentRequest<'_, WalletExportRequest> {
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
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, WalletExportRequest> {
    type Output = httpclient::InMemoryResult<crate::model::WalletExportResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/wallets/{wallet_id}/export", wallet_id = self.params.wallet_id
            );
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "encryption_type" : self.params.encryption_type }
                    ),
                );
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            if let Some(ref unwrapped) = self.params.privy_authorization_signature {
                r = r.header("privy-authorization-signature", &unwrapped.to_string());
            }
            r = r
                .json(
                    serde_json::json!(
                        { "recipient_public_key" : self.params.recipient_public_key }
                    ),
                );
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Export wallet

Export a wallet's private key*/
    pub fn wallet_export(
        &self,
        args: WalletExportRequired,
    ) -> FluentRequest<'_, WalletExportRequest> {
        FluentRequest {
            client: self,
            params: WalletExportRequest {
                encryption_type: args.encryption_type,
                privy_app_id: args.privy_app_id.to_owned(),
                privy_authorization_signature: None,
                recipient_public_key: args.recipient_public_key.to_owned(),
                wallet_id: args.wallet_id.to_owned(),
            },
        }
    }
}
