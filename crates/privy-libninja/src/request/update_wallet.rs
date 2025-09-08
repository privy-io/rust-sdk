use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{WalletAdditionalSigner, OwnerInput};
/**You should use this struct via [`PrivyLibninjaClient::update_wallet`].

On request success, this will return a [`Wallet`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWalletRequest {
    pub additional_signers: Option<WalletAdditionalSigner>,
    pub owner: Option<OwnerInput>,
    pub owner_id: Option<serde_json::Value>,
    pub policy_ids: Option<Vec<String>>,
    pub privy_app_id: String,
    pub privy_authorization_signature: Option<String>,
    pub wallet_id: String,
}
impl FluentRequest<'_, UpdateWalletRequest> {
    ///Set the value of the additional_signers field.
    pub fn additional_signers(
        mut self,
        additional_signers: WalletAdditionalSigner,
    ) -> Self {
        self.params.additional_signers = Some(additional_signers);
        self
    }
    ///Set the value of the owner field.
    pub fn owner(mut self, owner: OwnerInput) -> Self {
        self.params.owner = Some(owner);
        self
    }
    ///Set the value of the owner_id field.
    pub fn owner_id(mut self, owner_id: serde_json::Value) -> Self {
        self.params.owner_id = Some(owner_id);
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
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, UpdateWalletRequest> {
    type Output = httpclient::InMemoryResult<crate::model::Wallet>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/wallets/{wallet_id}", wallet_id = self.params.wallet_id
            );
            let mut r = self.client.client.patch(url);
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
            if let Some(ref unwrapped) = self.params.privy_authorization_signature {
                r = r.header("privy-authorization-signature", &unwrapped.to_string());
            }
            r = self.client._authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PrivyLibninjaClient {
    /**Update wallet

Update a wallet's policies or authorization key configuration.*/
    pub fn update_wallet(
        &self,
        privy_app_id: &str,
        wallet_id: &str,
    ) -> FluentRequest<'_, UpdateWalletRequest> {
        FluentRequest {
            client: self,
            params: UpdateWalletRequest {
                additional_signers: None,
                owner: None,
                owner_id: None,
                policy_ids: None,
                privy_app_id: privy_app_id.to_owned(),
                privy_authorization_signature: None,
                wallet_id: wallet_id.to_owned(),
            },
        }
    }
}
