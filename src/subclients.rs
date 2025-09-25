//! Subclients for the Privy API.
//!
//! This module includes the generated subclients implementations,
//! as well as some manual overrides for things that need the authctx,
//! following the stainless spec.

use p256::elliptic_curve::SecretKey;

use crate::{
    AuthorizationContext, PrivyApiError, PrivyExportError, PrivyHpke, PrivySignedApiError,
    ethereum::EthereumService,
    generate_authorization_signatures,
    generated::types::{
        HpkeEncryption, Policy, PrivateKeyInitInput, UpdatePolicyBody, UpdatePolicyPolicyId,
        Wallet, WalletExportRequestBody, WalletImportSubmissionRequestAdditionalSignersItem,
        WalletImportSubmissionRequestOwner, WalletImportSupportedChains,
    },
    import::WalletImport,
    solana::SolanaService,
};

include!(concat!(env!("OUT_DIR"), "/subclients.rs"));

impl PoliciesClient {
    /// Update a policy
    ///
    /// # Errors
    ///
    /// Can fail either if the authorization signature could not be generated,
    /// or if the api call fails whether than be due to network issues, auth problems,
    /// or the Privy API returning an error.
    pub async fn update<'a>(
        &'a self,
        policy_id: &'a UpdatePolicyPolicyId,
        ctx: &'a AuthorizationContext,
        body: &'a UpdatePolicyBody,
    ) -> Result<ResponseValue<Policy>, PrivySignedApiError> {
        let sig = generate_authorization_signatures(
            ctx,
            &self.app_id,
            crate::Method::PATCH,
            format!("{}/v1/policies/{}", self.base_url, policy_id.as_str()),
            body,
            None,
        )
        .await?;

        Ok(self._update(policy_id, Some(&sig), body).await?)
    }

    /// Delete a policy
    ///
    /// # Errors
    ///
    /// Can fail either if the authorization signature could not be generated,
    /// or if the api call fails whether than be due to network issues, auth problems,
    /// or the Privy API returning an error.
    pub async fn delete<'a>(
        &'a self,
        policy_id: &'a crate::generated::types::DeletePolicyPolicyId,
        ctx: &'a AuthorizationContext,
    ) -> Result<ResponseValue<crate::generated::types::DeletePolicyResponse>, PrivySignedApiError>
    {
        let sig = generate_authorization_signatures(
            ctx,
            &self.app_id,
            crate::Method::DELETE,
            format!("{}/v1/policies/{}", self.base_url, policy_id.as_str()),
            &serde_json::json!({}),
            None,
        )
        .await?;

        Ok(self._delete(policy_id, Some(&sig)).await?)
    }

    /// Create a rule for a policy
    ///
    /// # Errors
    ///
    /// Can fail either if the authorization signature could not be generated,
    /// or if the api call fails whether than be due to network issues, auth problems,
    /// or the Privy API returning an error.
    pub async fn create_rule<'a>(
        &'a self,
        policy_id: &'a crate::generated::types::CreatePolicyRulePolicyId,
        ctx: &'a AuthorizationContext,
        body: &'a crate::generated::types::PolicyRuleRequestBody,
    ) -> Result<ResponseValue<crate::generated::types::PolicyRuleResponse>, PrivySignedApiError>
    {
        let sig = generate_authorization_signatures(
            ctx,
            &self.app_id,
            crate::Method::POST,
            format!("{}/v1/policies/{}/rules", self.base_url, policy_id.as_str()),
            body,
            None,
        )
        .await?;

        Ok(self._create_rule(policy_id, Some(&sig), body).await?)
    }

    /// Update a rule for a policy
    ///
    /// # Errors
    ///
    /// Can fail either if the authorization signature could not be generated,
    /// or if the api call fails whether than be due to network issues, auth problems,
    /// or the Privy API returning an error.
    pub async fn update_rule<'a>(
        &'a self,
        policy_id: &'a crate::generated::types::UpdatePolicyRulePolicyId,
        rule_id: &'a crate::generated::types::UpdatePolicyRuleRuleId,
        ctx: &'a AuthorizationContext,
        body: &'a crate::generated::types::PolicyRuleRequestBody,
    ) -> Result<ResponseValue<crate::generated::types::UpdatePolicyRuleResponse>, PrivySignedApiError>
    {
        let sig = generate_authorization_signatures(
            ctx,
            &self.app_id,
            crate::Method::PATCH,
            format!(
                "{}/v1/policies/{}/rules/{}",
                self.base_url,
                policy_id.as_str(),
                rule_id.as_str()
            ),
            body,
            None,
        )
        .await?;

        Ok(self
            ._update_rule(policy_id, rule_id, Some(&sig), body)
            .await?)
    }

    /// Delete a rule for a policy
    ///
    /// # Errors
    ///
    /// Can fail either if the authorization signature could not be generated,
    /// or if the api call fails whether than be due to network issues, auth problems,
    /// or the Privy API returning an error.
    pub async fn delete_rule<'a>(
        &'a self,
        policy_id: &'a crate::generated::types::DeletePolicyRulePolicyId,
        rule_id: &'a crate::generated::types::DeletePolicyRuleRuleId,
        ctx: &'a AuthorizationContext,
    ) -> Result<ResponseValue<crate::generated::types::DeletePolicyRuleResponse>, PrivySignedApiError>
    {
        let sig = generate_authorization_signatures(
            ctx,
            &self.app_id,
            crate::Method::DELETE,
            format!(
                "{}/v1/policies/{}/rules/{}",
                self.base_url,
                policy_id.as_str(),
                rule_id.as_str()
            ),
            &serde_json::json!({}),
            None,
        )
        .await?;

        Ok(self._delete_rule(policy_id, rule_id, Some(&sig)).await?)
    }
}

impl KeyQuorumsClient {
    /// Update a key quorum
    ///
    /// # Errors
    ///
    /// Can fail either if the authorization signature could not be generated,
    /// or if the api call fails whether than be due to network issues, auth problems,
    /// or the Privy API returning an error.
    pub async fn update<'a>(
        &'a self,
        key_quorum_id: &'a str,
        ctx: &'a AuthorizationContext,
        body: &'a crate::generated::types::UpdateKeyQuorumBody,
    ) -> Result<ResponseValue<crate::generated::types::KeyQuorum>, PrivySignedApiError> {
        let sig = generate_authorization_signatures(
            ctx,
            &self.app_id,
            crate::Method::PATCH,
            format!("{}/v1/key_quorums/{}", self.base_url, key_quorum_id),
            body,
            None,
        )
        .await?;

        Ok(self._update(key_quorum_id, Some(&sig), body).await?)
    }

    /// Delete a key quorum
    ///
    /// # Errors
    ///
    /// Can fail either if the authorization signature could not be generated,
    /// or if the api call fails whether than be due to network issues, auth problems,
    /// or the Privy API returning an error.
    pub async fn delete<'a>(
        &'a self,
        key_quorum_id: &'a str,
        ctx: &'a AuthorizationContext,
    ) -> Result<ResponseValue<crate::generated::types::DeleteKeyQuorumResponse>, PrivySignedApiError>
    {
        let sig = generate_authorization_signatures(
            ctx,
            &self.app_id,
            crate::Method::DELETE,
            format!("{}/v1/key_quorums/{}", self.base_url, key_quorum_id),
            &serde_json::json!({}),
            None,
        )
        .await?;

        Ok(self._delete(key_quorum_id, Some(&sig)).await?)
    }
}

impl WalletsClient {
    /// Make a wallet rpc call
    ///
    /// # Errors
    ///
    /// Can fail either if the authorization signature could not be generated,
    /// or if the api call fails whether than be due to network issues, auth problems,
    /// or the Privy API returning an error.
    pub async fn rpc<'a>(
        &'a self,
        wallet_id: &'a str,
        ctx: &'a AuthorizationContext,
        privy_idempotency_key: Option<&'a str>,
        body: &'a crate::generated::types::WalletRpcBody,
    ) -> Result<ResponseValue<crate::generated::types::WalletRpcResponse>, PrivySignedApiError>
    {
        let sig = generate_authorization_signatures(
            ctx,
            &self.app_id,
            crate::Method::POST,
            format!("{}/v1/wallets/{}/rpc", self.base_url, wallet_id),
            body,
            None,
        )
        .await?;

        Ok(self
            ._rpc(wallet_id, Some(&sig), privy_idempotency_key, body)
            .await?)
    }

    /// Make a wallet raw sign call
    ///
    /// # Errors
    ///
    /// Can fail either if the authorization signature could not be generated,
    /// or if the api call fails whether than be due to network issues, auth problems,
    /// or the Privy API returning an error.
    pub async fn raw_sign<'a>(
        &'a self,
        wallet_id: &'a str,
        ctx: &'a AuthorizationContext,
        privy_idempotency_key: Option<&'a str>,
        body: &'a crate::generated::types::RawSign,
    ) -> Result<ResponseValue<crate::generated::types::RawSignResponse>, PrivySignedApiError> {
        let sig = generate_authorization_signatures(
            ctx,
            &self.app_id,
            crate::Method::POST,
            format!("{}/v1/wallets/{}/raw_sign", self.base_url, wallet_id),
            body,
            None,
        )
        .await?;

        Ok(self
            ._raw_sign(wallet_id, Some(&sig), privy_idempotency_key, body)
            .await?)
    }

    /// Update a wallet
    ///
    /// # Errors
    ///
    /// Can fail either if the authorization signature could not be generated,
    /// or if the api call fails whether than be due to
    pub async fn update<'a>(
        &'a self,
        wallet_id: &'a str,
        ctx: &'a AuthorizationContext,
        body: &'a crate::generated::types::UpdateWalletBody,
    ) -> Result<ResponseValue<crate::generated::types::Wallet>, PrivySignedApiError> {
        let sig = generate_authorization_signatures(
            ctx,
            &self.app_id,
            crate::Method::PATCH,
            format!("{}/v1/wallets/{}", self.base_url, wallet_id),
            body,
            None,
        )
        .await?;

        Ok(self._update(wallet_id, Some(&sig), body).await?)
    }

    /// Export a wallet
    ///
    /// # Errors
    ///
    /// Can fail either if the authorization signature could not be generated,
    /// or if the api call fails whether than be due to network issues, auth problems,
    /// or the Privy API returning an error. Additionally, if the privy platform
    /// were to produce a response from which we cannot decrypt the secret key,
    /// a `PrivyExportError::Key` will be returned.
    pub async fn export<'a>(
        &'a self,
        wallet_id: &'a str,
        ctx: &'a AuthorizationContext,
    ) -> Result<SecretKey<p256::NistP256>, PrivyExportError> {
        let privy_hpke = PrivyHpke::new();
        let body = WalletExportRequestBody {
            encryption_type: HpkeEncryption::Hpke,
            recipient_public_key: privy_hpke.public_key().unwrap(),
        };

        let sig = generate_authorization_signatures(
            ctx,
            &self.app_id,
            crate::Method::POST,
            format!("{}/v1/wallets/{}/export", self.base_url, wallet_id),
            &body,
            None,
        )
        .await?;

        let resp = self._export(wallet_id, Some(&sig), &body).await?;

        Ok(privy_hpke.decrypt(&resp.encapsulated_key, &resp.ciphertext)?)
    }

    /// Import a wallet into the Privy app
    ///
    /// # Errors
    ///
    ///
    pub async fn import(
        &self,
        address: String,
        private_key_hex: &str,
        chain_type: WalletImportSupportedChains,
        owner: Option<WalletImportSubmissionRequestOwner>,
        policy_ids: Vec<String>,
        additional_signers: Vec<WalletImportSubmissionRequestAdditionalSignersItem>,
    ) -> Result<ResponseValue<Wallet>, PrivyApiError> {
        WalletImport::new(
            self.clone(),
            crate::generated::types::WalletImportInitializationRequest::PrivateKeyInitInput(
                PrivateKeyInitInput {
                    address: address.clone(),
                    chain_type: chain_type.clone(),
                    encryption_type: HpkeEncryption::Hpke,
                    entropy_type:
                        crate::generated::types::PrivateKeyInitInputEntropyType::PrivateKey,
                },
            ),
        )
        .await
        .submit(private_key_hex, owner, policy_ids, additional_signers)
        .await
    }

    pub(crate) async fn submit_import<'a>(
        &'a self,
        body: &'a types::WalletImportSubmissionRequest,
    ) -> Result<ResponseValue<types::Wallet>, Error<()>> {
        self._submit_import(body).await
    }

    /// Returns an `EthereumService` instance for interacting with the Ethereum API
    pub fn ethereum(&self) -> EthereumService {
        EthereumService::new(self.clone())
    }

    /// Returns an `SolanaService` instance for interacting with the Solana API
    pub fn solana(&self) -> SolanaService {
        SolanaService::new(self.clone())
    }
}
