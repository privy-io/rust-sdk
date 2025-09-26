use super::ResponseValue;
use crate::{
    AuthorizationContext, PrivySignedApiError, generate_authorization_signatures,
    subclients::KeyQuorumsClient,
};

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
