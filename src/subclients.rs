//! Subclients for the Privy API.
//!
//! This module includes the generated subclients implementations,
//! as well as some manual overrides for things that need the authctx,
//! following the stainless spec.

use crate::{
    AuthorizationContext, generate_authorization_signatures,
    generated::types::{Policy, UpdatePolicyBody, UpdatePolicyPolicyId},
};

include!(concat!(env!("OUT_DIR"), "/subclients.rs"));

impl PoliciesClient {
    pub async fn update<'a>(
        &'a self,
        policy_id: &'a UpdatePolicyPolicyId,
        ctx: &'a AuthorizationContext,
        body: &'a UpdatePolicyBody,
    ) -> Result<ResponseValue<Policy>, Error<()>> {
        let sig = generate_authorization_signatures(
            ctx,
            &self.app_id,
            crate::Method::PATCH,
            format!("{}/v1/policies/{}", self.base_url, policy_id.as_str()),
            body,
            None,
        )
        .await
        .unwrap();

        self._update(policy_id, Some(&sig), body).await
    }

    pub async fn delete<'a>(
        &'a self,
        policy_id: &'a crate::generated::types::DeletePolicyPolicyId,
        ctx: &'a AuthorizationContext,
    ) -> Result<ResponseValue<crate::generated::types::DeletePolicyResponse>, Error<()>> {
        let sig = generate_authorization_signatures(
            ctx,
            &self.app_id,
            crate::Method::DELETE,
            format!("{}/v1/policies/{}", self.base_url, policy_id.as_str()),
            &serde_json::json!({}),
            None,
        )
        .await
        .unwrap();

        self._delete(policy_id, Some(&sig)).await
    }

    pub async fn create_rule<'a>(
        &'a self,
        policy_id: &'a crate::generated::types::CreatePolicyRulePolicyId,
        ctx: &'a AuthorizationContext,
        body: &'a crate::generated::types::PolicyRule,
    ) -> Result<ResponseValue<crate::generated::types::RuleResponse>, Error<()>> {
        let sig = generate_authorization_signatures(
            ctx,
            &self.app_id,
            crate::Method::POST,
            format!("{}/v1/policies/{}/rules", self.base_url, policy_id.as_str()),
            body,
            None,
        )
        .await
        .unwrap();

        self._create_rule(policy_id, Some(&sig), body).await
    }

    pub async fn update_rule<'a>(
        &'a self,
        policy_id: &'a crate::generated::types::UpdatePolicyRulePolicyId,
        rule_id: &'a crate::generated::types::UpdatePolicyRuleRuleId,
        ctx: &'a AuthorizationContext,
        body: &'a crate::generated::types::PolicyRule,
    ) -> Result<ResponseValue<crate::generated::types::UpdatePolicyRuleResponse>, Error<()>> {
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
        .await
        .unwrap();

        self._update_rule(policy_id, rule_id, Some(&sig), body)
            .await
    }

    pub async fn delete_rule<'a>(
        &'a self,
        policy_id: &'a crate::generated::types::DeletePolicyRulePolicyId,
        rule_id: &'a crate::generated::types::DeletePolicyRuleRuleId,
        ctx: &'a AuthorizationContext,
    ) -> Result<ResponseValue<crate::generated::types::DeletePolicyRuleResponse>, Error<()>> {
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
        .await
        .unwrap();

        self._delete_rule(policy_id, rule_id, Some(&sig)).await
    }
}

impl KeyQuorumsClient {
    pub async fn update<'a>(
        &'a self,
        key_quorum_id: &'a str,
        ctx: &'a AuthorizationContext,
        body: &'a crate::generated::types::UpdateKeyQuorumBody,
    ) -> Result<ResponseValue<crate::generated::types::KeyQuorum>, Error<()>> {
        let sig = generate_authorization_signatures(
            ctx,
            &self.app_id,
            crate::Method::PATCH,
            format!("{}/v1/key_quorums/{}", self.base_url, key_quorum_id),
            body,
            None,
        )
        .await
        .unwrap();

        self._update(key_quorum_id, Some(&sig), body).await
    }

    pub async fn delete<'a>(
        &'a self,
        key_quorum_id: &'a str,
        ctx: &'a AuthorizationContext,
    ) -> Result<ResponseValue<crate::generated::types::DeleteKeyQuorumResponse>, Error<()>> {
        let sig = generate_authorization_signatures(
            ctx,
            &self.app_id,
            crate::Method::DELETE,
            format!("{}/v1/key_quorums/{}", self.base_url, key_quorum_id),
            &serde_json::json!({}),
            None,
        )
        .await
        .unwrap();

        self._delete(key_quorum_id, Some(&sig)).await
    }
}

impl WalletsClient {
    pub async fn rpc<'a>(
        &'a self,
        wallet_id: &'a str,
        ctx: &'a AuthorizationContext,
        privy_idempotency_key: Option<&'a str>,
        body: &'a crate::generated::types::WalletRpcBody,
    ) -> Result<ResponseValue<crate::generated::types::WalletRpcResponse>, Error<()>> {
        let sig = generate_authorization_signatures(
            ctx,
            &self.app_id,
            crate::Method::POST,
            format!("{}/v1/wallets/{}/rpc", self.base_url, wallet_id),
            body,
            None,
        )
        .await
        .unwrap();

        self._rpc(wallet_id, Some(&sig), privy_idempotency_key, body)
            .await
    }

    pub async fn raw_sign<'a>(
        &'a self,
        wallet_id: &'a str,
        ctx: &'a AuthorizationContext,
        privy_idempotency_key: Option<&'a str>,
        body: &'a crate::generated::types::RawSign,
    ) -> Result<ResponseValue<crate::generated::types::RawSignResponse>, Error<()>> {
        let sig = generate_authorization_signatures(
            ctx,
            &self.app_id,
            crate::Method::POST,
            format!("{}/v1/wallets/{}/raw_sign", self.base_url, wallet_id),
            body,
            None,
        )
        .await
        .unwrap();

        self._raw_sign(wallet_id, Some(&sig), privy_idempotency_key, body)
            .await
    }

    pub async fn update<'a>(
        &'a self,
        wallet_id: &'a str,
        ctx: &'a AuthorizationContext,
        body: &'a crate::generated::types::UpdateWalletBody,
    ) -> Result<ResponseValue<crate::generated::types::Wallet>, Error<()>> {
        let sig = generate_authorization_signatures(
            ctx,
            &self.app_id,
            crate::Method::PATCH,
            format!("{}/v1/wallets/{}", self.base_url, wallet_id),
            body,
            None,
        )
        .await
        .unwrap();

        self._update(wallet_id, Some(&sig), body).await
    }

    pub async fn export<'a>(
        &'a self,
        wallet_id: &'a str,
        ctx: &'a AuthorizationContext,
        body: &'a crate::generated::types::WalletExportRequest,
    ) -> Result<ResponseValue<crate::generated::types::WalletExportResponse>, Error<()>> {
        let sig = generate_authorization_signatures(
            ctx,
            &self.app_id,
            crate::Method::POST,
            format!("{}/v1/wallets/{}/export", self.base_url, wallet_id),
            body,
            None,
        )
        .await
        .unwrap();

        self._export(wallet_id, Some(&sig), body).await
    }
}
