use super::{Error, ResponseValue, types};
use crate::{
    AuthorizationContext, PrivyApiError, PrivyExportError, PrivyHpke, PrivySignedApiError,
    ethereum::EthereumService,
    generate_authorization_signatures,
    generated::types::{
        HpkeEncryption, PrivateKeyInitInput, Wallet, WalletExportRequestBody,
        WalletImportSubmissionRequestAdditionalSignersItem, WalletImportSubmissionRequestOwner,
        WalletImportSupportedChains,
    },
    import::WalletImport,
    solana::SolanaService,
    subclients::WalletsClient,
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
        body: &'a crate::generated::types::WalletRpcBody,
    ) -> Result<ResponseValue<crate::generated::types::WalletRpcResponse>, PrivySignedApiError>
    {
        let sig = generate_authorization_signatures(
            ctx,
            &self.app_id,
            crate::Method::POST,
            format!("{}/v1/wallets/{}/rpc", self.base_url, wallet_id),
            body,
            privy_idempotency_key.map(|k| k.to_owned()),
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
            privy_idempotency_key.map(|k| k.to_owned()),
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
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, PrivyExportError> {
        let privy_hpke = PrivyHpke::new();
        let body = WalletExportRequestBody {
            encryption_type: HpkeEncryption::Hpke,
            recipient_public_key: privy_hpke.public_key()?,
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
        owner: Option<WalletImportSubmissionRequestOwner>,
        policy_ids: Vec<String>,
        additional_signers: Vec<WalletImportSubmissionRequestAdditionalSignersItem>,
    ) -> Result<ResponseValue<Wallet>, PrivyApiError> {
        WalletImport::new(
            self.clone(),
            crate::generated::types::WalletImportInitializationRequest::PrivateKeyInitInput(
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
