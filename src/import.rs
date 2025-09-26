use base64::Engine;
use hpke::{
    Deserializable, OpModeS, Serializable, aead::ChaCha20Poly1305, kdf::HkdfSha256,
    kem::DhP256HkdfSha256,
};

use crate::{
    PrivyApiError,
    generated::{
        Error, ResponseValue,
        types::{
            PrivateKeySubmitInput, Wallet, WalletImportInitializationRequest,
            WalletImportInitializationResponse, WalletImportSubmissionRequest,
            WalletImportSubmissionRequestOwner, WalletImportSubmissionRequestWallet,
            WalletImportSupportedChains,
        },
    },
    subclients::WalletsClient,
};

pub struct WalletImport {
    client: WalletsClient,
    initialization_response: WalletImportInitializationResponse,
    address: String,
    chain_type: WalletImportSupportedChains,
}

impl WalletImport {
    pub(crate) async fn new(
        client: WalletsClient,
        request: WalletImportInitializationRequest,
    ) -> Result<Self, PrivyApiError> {
        let (address, chain_type) = match &request {
            WalletImportInitializationRequest::PrivateKeyInitInput(input) => {
                (input.address.to_owned(), input.chain_type)
            }
            WalletImportInitializationRequest::HdInitInput(input) => {
                (input.address.to_owned(), input.chain_type)
            }
        };

        let initialization_response = client._init_import(&request).await?;

        Ok(Self {
            client,
            initialization_response: initialization_response.into_inner(),
            address,
            chain_type,
        })
    }

    fn encrypt_private_key(
        &self,
        private_key_hex: &str,
    ) -> Result<(String, String), Box<dyn std::error::Error>> {
        // Decode the public key from base64
        let public_key_bytes = base64::engine::general_purpose::STANDARD
            .decode(&self.initialization_response.encryption_public_key)?;

        // Deserialize the public key using HPKE trait
        let public_key = <DhP256HkdfSha256 as hpke::Kem>::PublicKey::from_bytes(&public_key_bytes)
            .map_err(|e| format!("Failed to deserialize public key: {e:?}"))?;

        // Convert hex private key to bytes (remove 0x prefix if present)
        let private_key_hex = private_key_hex
            .strip_prefix("0x")
            .unwrap_or(private_key_hex);
        let private_key_bytes = hex::decode(private_key_hex)?;

        // Setup HPKE sender context
        let mut rng = rand::thread_rng();
        let (encapsulated_key, mut encryption_context) =
            hpke::setup_sender::<ChaCha20Poly1305, HkdfSha256, DhP256HkdfSha256, _>(
                &OpModeS::Base,
                &public_key,
                &[],
                &mut rng,
            )
            .map_err(|e| format!("HPKE setup failed: {e:?}"))?;

        // Encrypt the private key
        let ciphertext = encryption_context
            .seal(&private_key_bytes, &[])
            .map_err(|e| format!("HPKE encryption failed: {e:?}"))?;

        // Encode results as base64
        let ciphertext_b64 = base64::engine::general_purpose::STANDARD.encode(&ciphertext);
        let encapsulated_key_b64 =
            base64::engine::general_purpose::STANDARD.encode(encapsulated_key.to_bytes());

        Ok((ciphertext_b64, encapsulated_key_b64))
    }

    pub(crate) async fn submit(
        self,
        private_key_hex: &str,
        owner: Option<WalletImportSubmissionRequestOwner>,
        policy_ids: Vec<String>,
        additional_signers: Vec<
            crate::generated::types::WalletImportSubmissionRequestAdditionalSignersItem,
        >,
    ) -> Result<ResponseValue<Wallet>, PrivyApiError> {
        // Encrypt the private key using HPKE
        let (ciphertext, encapsulated_key) = self
            .encrypt_private_key(private_key_hex)
            .map_err(|_| Error::InvalidRequest("Failed to encrypt private key".to_string()))?;

        // Create the wallet submission input
        let wallet_input = PrivateKeySubmitInput {
            address: self.address,
            chain_type: self.chain_type,
            ciphertext,
            encapsulated_key,
            encryption_type: self.initialization_response.encryption_type,
            entropy_type: crate::generated::types::PrivateKeySubmitInputEntropyType::PrivateKey,
        };

        // Create the submission request
        let submission_request = WalletImportSubmissionRequest {
            wallet: WalletImportSubmissionRequestWallet::PrivateKeySubmitInput(wallet_input),
            owner,
            owner_id: None,
            policy_ids,
            additional_signers,
        };

        // Submit the import request
        self.client.submit_import(&submission_request).await
    }
}
