use super::ResponseValue;
use crate::{
    AuthorizationContext, PrivySignedApiError, RequestOptions,
    generated::types::KeyQuorumId,
    subclients::{KeyQuorumsClient, sign_request},
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
        key_quorum_id: &'a KeyQuorumId,
        ctx: &'a AuthorizationContext,
        body: &'a crate::generated::types::KeyQuorumUpdateRequestBody,
    ) -> Result<ResponseValue<crate::generated::types::KeyQuorum>, PrivySignedApiError> {
        self.update_with_options(key_quorum_id, ctx, body, &RequestOptions::default())
            .await
    }

    /// Update a key quorum, with per-request [`RequestOptions`].
    ///
    /// `options.request_expiry`, when set, is sent as the `privy-request-expiry`
    /// header and included in the authorization signature, even if request
    /// expiry is disabled on the client.
    ///
    /// # Errors
    ///
    /// Can fail either if the authorization signature could not be generated,
    /// or if the api call fails whether than be due to network issues, auth problems,
    /// or the Privy API returning an error.
    pub async fn update_with_options<'a>(
        &'a self,
        key_quorum_id: &'a KeyQuorumId,
        ctx: &'a AuthorizationContext,
        body: &'a crate::generated::types::KeyQuorumUpdateRequestBody,
        options: &'a RequestOptions,
    ) -> Result<ResponseValue<crate::generated::types::KeyQuorum>, PrivySignedApiError> {
        let signed = sign_request(
            ctx,
            &self.app_id,
            crate::Method::PATCH,
            format!(
                "{}/v1/key_quorums/{}",
                self.base_url,
                key_quorum_id.as_str()
            ),
            body,
            None,
            self.request_expiry.resolve(options.request_expiry),
        )
        .await?;

        Ok(self
            ._update(
                key_quorum_id,
                Some(&signed.signature),
                signed.request_expiry.as_deref(),
                body,
            )
            .await?)
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
        key_quorum_id: &'a KeyQuorumId,
        ctx: &'a AuthorizationContext,
    ) -> Result<ResponseValue<crate::generated::types::SuccessResponse>, PrivySignedApiError> {
        self.delete_with_options(key_quorum_id, ctx, &RequestOptions::default())
            .await
    }

    /// Delete a key quorum, with per-request [`RequestOptions`].
    ///
    /// `options.request_expiry`, when set, is sent as the `privy-request-expiry`
    /// header and included in the authorization signature, even if request
    /// expiry is disabled on the client.
    ///
    /// # Errors
    ///
    /// Can fail either if the authorization signature could not be generated,
    /// or if the api call fails whether than be due to network issues, auth problems,
    /// or the Privy API returning an error.
    pub async fn delete_with_options<'a>(
        &'a self,
        key_quorum_id: &'a KeyQuorumId,
        ctx: &'a AuthorizationContext,
        options: &'a RequestOptions,
    ) -> Result<ResponseValue<crate::generated::types::SuccessResponse>, PrivySignedApiError> {
        let signed = sign_request(
            ctx,
            &self.app_id,
            crate::Method::DELETE,
            format!(
                "{}/v1/key_quorums/{}",
                self.base_url,
                key_quorum_id.as_str()
            ),
            &serde_json::json!({}),
            None,
            self.request_expiry.resolve(options.request_expiry),
        )
        .await?;

        Ok(self
            ._delete(
                key_quorum_id,
                Some(&signed.signature),
                signed.request_expiry.as_deref(),
            )
            .await?)
    }
}
