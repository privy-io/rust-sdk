use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PrivyLibninjaClient::wallet_import_submit`].

On request success, this will return a [`Wallet`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletImportSubmitRequest {
    pub additional_signers: Option<Vec<serde_json::Value>>,
    pub owner: Option<serde_json::Value>,
    pub owner_id: Option<String>,
    pub policy_ids: Option<Vec<String>>,
    pub privy_app_id: String,
    pub wallet: serde_json::Value,
}
impl FluentRequest<'_, WalletImportSubmitRequest> {
    ///Set the value of the additional_signers field.
    pub fn additional_signers(
        mut self,
        additional_signers: Vec<serde_json::Value>,
    ) -> Self {
        self.params.additional_signers = Some(additional_signers);
        self
    }
    ///Set the value of the owner field.
    pub fn owner(mut self, owner: serde_json::Value) -> Self {
        self.params.owner = Some(owner);
        self
    }
    ///Set the value of the owner_id field.
    pub fn owner_id(mut self, owner_id: &str) -> Self {
        self.params.owner_id = Some(owner_id.to_owned());
        self
    }
    ///Set the value of the policy_ids field.
    pub fn policy_ids(
        mut self,
        policy_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .policy_ids = Some(
            policy_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, WalletImportSubmitRequest> {
    type Output = httpclient::InMemoryResult<crate::model::Wallet>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/wallets/import/submit";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.additional_signers {
                r = r.json(serde_json::json!({ "additional_signers" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.owner {
                r = r.json(serde_json::json!({ "owner" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.owner_id {
                r = r.json(serde_json::json!({ "owner_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.policy_ids {
                r = r.json(serde_json::json!({ "policy_ids" : unwrapped }));
            }
            r = r.header("privy-app-id", &self.params.privy_app_id.to_string());
            r = r.json(serde_json::json!({ "wallet" : self.params.wallet }));
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Submit import

Submit a wallet import request.*/
    pub fn wallet_import_submit(
        &self,
        privy_app_id: &str,
        wallet: serde_json::Value,
    ) -> FluentRequest<'_, WalletImportSubmitRequest> {
        FluentRequest {
            client: self,
            params: WalletImportSubmitRequest {
                additional_signers: None,
                owner: None,
                owner_id: None,
                policy_ids: None,
                privy_app_id: privy_app_id.to_owned(),
                wallet,
            },
        }
    }
}
