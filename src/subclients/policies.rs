use super::ResponseValue;
use crate::{
    AuthorizationContext, PrivySignedApiError, generate_authorization_signatures,
    generated::types::{Policy, UpdatePolicyBody, UpdatePolicyPolicyId},
    subclients::PoliciesClient,
};

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
