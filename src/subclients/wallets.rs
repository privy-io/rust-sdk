use super::{Error, ResponseValue, types};
use crate::{
    AuthorizationContext, PrivyApiError, PrivyExportError, PrivyHpke, PrivySignedApiError,
    RequestOptions,
    ethereum::EthereumService,
    generated::types::{
        AdditionalSignerInput, HpkeEncryption, OwnerInput, PrivateKeyInitInput, Wallet,
        WalletExportRequestBody, WalletImportSupportedChains,
    },
    import::WalletImport,
    solana::SolanaService,
    subclients::{WalletsClient, sign_request},
};

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
        body: &'a crate::generated::types::WalletRpcRequestBody,
    ) -> Result<ResponseValue<crate::generated::types::WalletRpcResponse>, PrivySignedApiError>
    {
        self.rpc_with_options(
            wallet_id,
            ctx,
            privy_idempotency_key,
            body,
            &RequestOptions::default(),
        )
        .await
    }

    /// Make a wallet rpc call, with per-request [`RequestOptions`].
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
    pub async fn rpc_with_options<'a>(
        &'a self,
        wallet_id: &'a str,
        ctx: &'a AuthorizationContext,
        privy_idempotency_key: Option<&'a str>,
        body: &'a crate::generated::types::WalletRpcRequestBody,
        options: &'a RequestOptions,
    ) -> Result<ResponseValue<crate::generated::types::WalletRpcResponse>, PrivySignedApiError>
    {
        let signed = sign_request(
            ctx,
            &self.app_id,
            crate::Method::POST,
            format!("{}/v1/wallets/{}/rpc", self.base_url, wallet_id),
            body,
            privy_idempotency_key,
            self.request_expiry.resolve(options.request_expiry),
        )
        .await?;

        Ok(self
            ._rpc(
                wallet_id,
                Some(&signed.signature),
                privy_idempotency_key,
                signed.request_expiry.as_deref(),
                body,
            )
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
        body: &'a crate::generated::types::RawSignInput,
    ) -> Result<ResponseValue<crate::generated::types::RawSignResponse>, PrivySignedApiError> {
        self.raw_sign_with_options(
            wallet_id,
            ctx,
            privy_idempotency_key,
            body,
            &RequestOptions::default(),
        )
        .await
    }

    /// Make a wallet raw sign call, with per-request [`RequestOptions`].
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
    pub async fn raw_sign_with_options<'a>(
        &'a self,
        wallet_id: &'a str,
        ctx: &'a AuthorizationContext,
        privy_idempotency_key: Option<&'a str>,
        body: &'a crate::generated::types::RawSignInput,
        options: &'a RequestOptions,
    ) -> Result<ResponseValue<crate::generated::types::RawSignResponse>, PrivySignedApiError> {
        let signed = sign_request(
            ctx,
            &self.app_id,
            crate::Method::POST,
            format!("{}/v1/wallets/{}/raw_sign", self.base_url, wallet_id),
            body,
            privy_idempotency_key,
            self.request_expiry.resolve(options.request_expiry),
        )
        .await?;

        Ok(self
            ._raw_sign(
                wallet_id,
                Some(&signed.signature),
                privy_idempotency_key,
                signed.request_expiry.as_deref(),
                body,
            )
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
        body: &'a crate::generated::types::WalletUpdateRequestBody,
    ) -> Result<ResponseValue<crate::generated::types::Wallet>, PrivySignedApiError> {
        self.update_with_options(wallet_id, ctx, body, &RequestOptions::default())
            .await
    }

    /// Update a wallet, with per-request [`RequestOptions`].
    ///
    /// `options.request_expiry`, when set, is sent as the `privy-request-expiry`
    /// header and included in the authorization signature, even if request
    /// expiry is disabled on the client.
    ///
    /// # Errors
    ///
    /// Can fail either if the authorization signature could not be generated,
    /// or if the api call fails whether than be due to
    pub async fn update_with_options<'a>(
        &'a self,
        wallet_id: &'a str,
        ctx: &'a AuthorizationContext,
        body: &'a crate::generated::types::WalletUpdateRequestBody,
        options: &'a RequestOptions,
    ) -> Result<ResponseValue<crate::generated::types::Wallet>, PrivySignedApiError> {
        let signed = sign_request(
            ctx,
            &self.app_id,
            crate::Method::PATCH,
            format!("{}/v1/wallets/{}", self.base_url, wallet_id),
            body,
            None,
            self.request_expiry.resolve(options.request_expiry),
        )
        .await?;

        Ok(self
            ._update(
                wallet_id,
                Some(&signed.signature),
                signed.request_expiry.as_deref(),
                body,
            )
            .await?)
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
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, PrivyExportError> {
        self.export_with_options(wallet_id, ctx, &RequestOptions::default())
            .await
    }

    /// Export a wallet, with per-request [`RequestOptions`].
    ///
    /// `options.request_expiry`, when set, is sent as the `privy-request-expiry`
    /// header and included in the authorization signature, even if request
    /// expiry is disabled on the client.
    ///
    /// # Errors
    ///
    /// Can fail either if the authorization signature could not be generated,
    /// or if the api call fails whether than be due to network issues, auth problems,
    /// or the Privy API returning an error. Additionally, if the privy platform
    /// were to produce a response from which we cannot decrypt the secret key,
    /// a `PrivyExportError::Key` will be returned.
    pub async fn export_with_options<'a>(
        &'a self,
        wallet_id: &'a str,
        ctx: &'a AuthorizationContext,
        options: &'a RequestOptions,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, PrivyExportError> {
        let privy_hpke = PrivyHpke::new();
        let body = WalletExportRequestBody {
            encryption_type: HpkeEncryption::Hpke,
            export_seed_phrase: None,
            recipient_public_key: privy_hpke.public_key()?,
        };

        let signed = sign_request(
            ctx,
            &self.app_id,
            crate::Method::POST,
            format!("{}/v1/wallets/{}/export", self.base_url, wallet_id),
            &body,
            None,
            self.request_expiry.resolve(options.request_expiry),
        )
        .await?;

        let resp = self
            ._export(
                wallet_id,
                Some(&signed.signature),
                signed.request_expiry.as_deref(),
                &body,
            )
            .await?;

        tracing::debug!("Encapsulated key: {:?}", resp);

        Ok(privy_hpke.decrypt_raw(&resp.encapsulated_key, &resp.ciphertext)?)
    }

    /// Import a wallet into the Privy app
    ///
    /// # Errors
    pub async fn import(
        &self,
        address: String,
        private_key_hex: &str,
        chain_type: WalletImportSupportedChains,
        owner: Option<OwnerInput>,
        policy_ids: Vec<String>,
        additional_signers: Option<AdditionalSignerInput>,
    ) -> Result<ResponseValue<Wallet>, PrivyApiError> {
        WalletImport::new(
            self.clone(),
            crate::generated::types::WalletImportInitBody::PrivateKeyInitInput(
                PrivateKeyInitInput {
                    address: address.clone(),
                    chain_type,
                    encryption_type: HpkeEncryption::Hpke,
                    entropy_type:
                        crate::generated::types::PrivateKeyInitInputEntropyType::PrivateKey,
                },
            ),
        )
        .await?
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
