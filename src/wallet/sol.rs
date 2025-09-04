use base64::{Engine, engine::general_purpose::STANDARD};
use privy_api::types::{
    SolanaSignAndSendTransactionRpcInputChainType,
    SolanaSignAndSendTransactionRpcInputMethod,
    SolanaSignAndSendTransactionRpcInputParamsEncoding,
    // Solana RPC types
    SolanaSignMessageRpcInputChainType,
    SolanaSignMessageRpcInputMethod,
    SolanaSignMessageRpcInputParamsEncoding,
    SolanaSignTransactionRpcInputChainType,
    SolanaSignTransactionRpcInputMethod,
    SolanaSignTransactionRpcInputParamsEncoding,
    // EIP-7702 authorization parameter types (commented out due to import issues)
    // EthereumSign7702AuthorizationRpcInputParamsAuthorization,
    // Common types
    WalletRpcBody,
    builder::{
        // Ethereum builders
        SolanaSignAndSendTransactionRpcInput,
        SolanaSignAndSendTransactionRpcInputParams,
        // Solana builders
        SolanaSignMessageRpcInput,
        SolanaSignMessageRpcInputParams,
        SolanaSignTransactionRpcInput,
        SolanaSignTransactionRpcInputParams,
    },
};
use solana_sdk::{
    pubkey::Pubkey, signature::Signature as SolanaSignature, transaction::Transaction,
};

use crate::{
    PrivyError,
    wallet::{Chain, Wallet},
};

/// Solana blockchain type marker.
///
/// Use this type to create Solana-specific wallet instances:
/// ```no_run
/// # use privy_rust::{PrivyClient, wallet::{Wallet, Solana}};
/// # async fn foo() {
/// # let client = PrivyClient::new("app_id".to_string(), "app_secret".to_string()).unwrap();
/// let solana_wallet = client.wallet::<Solana>("wallet_id");
/// # }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Solana;

impl Chain for Solana {
    type PublicKey = Pubkey;
    type Signature = SolanaSignature;
    type Transaction = Transaction;
}

// Solana-specific implementation
impl Wallet<Solana> {
    /// Sign an arbitrary message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message bytes to sign
    ///
    /// # Errors
    ///
    /// Returns [`PrivyError`] if the signing request fails.
    ///
    /// # Panics
    ///
    /// If the server returns a mismatched RPC response
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use privy_rust::{PrivyClient, wallet::{Wallet, Solana}};
    /// # async fn foo() {
    /// let client = PrivyClient::new("app_id".to_string(), "app_secret".to_string()).unwrap();
    /// let wallet = client.wallet::<Solana>("wallet_id");
    /// let message = b"Hello, Solana!";
    /// let signature = wallet.sign_message(message).await.unwrap();
    /// println!("Signature: {}", signature);
    /// # }
    /// ```
    pub async fn sign_message(&self, message: &[u8]) -> Result<SolanaSignature, PrivyError> {
        let input = SolanaSignMessageRpcInput::default()
            .method(SolanaSignMessageRpcInputMethod::SignMessage)
            .chain_type(Some(SolanaSignMessageRpcInputChainType::Solana))
            .params(
                SolanaSignMessageRpcInputParams::default()
                    .encoding(SolanaSignMessageRpcInputParamsEncoding::Base64)
                    .message(STANDARD.encode(message)),
            )
            .try_into()?;

        let response = match self
            .client
            .client
            .wallet_rpc()
            .wallet_id(&self.wallet_id)
            .privy_app_id(&self.client.app_id)
            .body(WalletRpcBody::SolanaSignMessageRpcInput(input))
            .send()
            .await
        {
            Ok(response) => response,
            Err(privy_api::Error::UnexpectedResponse(e)) => {
                let body = e.text().await.unwrap();
                tracing::error!("Unexpected response from Privy API: {}", body);
                return Err(PrivyError::Unknown);
            }
            Err(e) => {
                tracing::error!("Failed to send request to Privy API: {}", e);
                return Err(PrivyError::Unknown);
            }
        };

        let privy_api::types::WalletRpcResponse::SolanaSignMessageRpcResponse(sign_response) =
            response.into_inner()
        else {
            panic!("invalid response type");
        };

        tracing::info!("sign_response: {:?}", sign_response);

        // for some reason, it complains about the signature being too short. we assert 64 and take the first 64 bytes
        let mut buffer = [0u8; 128];
        let written = STANDARD
            .decode_slice(&sign_response.data.signature, &mut buffer)
            .unwrap();

        if written != 64 {
            panic!("signature is not 64 bytes");
        }

        // grab first 64 bytes
        let mut slice = [0u8; 64];
        slice.copy_from_slice(&buffer[0..64]);

        Ok(SolanaSignature::from(slice))
    }

    /// Sign a transaction without sending it.
    ///
    /// # Arguments
    ///
    /// * `transaction` - The Solana transaction to sign
    ///
    /// # Errors
    ///
    /// Returns [`PrivyError`] if the signing request fails.
    ///
    /// # Panics
    ///
    /// If the server returns a mismatched RPC response
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use privy_rust::{PrivyClient, wallet::{Wallet, Solana}};
    /// # use solana_sdk::transaction::Transaction;
    /// # async fn foo() {
    /// # let client = PrivyClient::new("app_id".to_string(), "app_secret".to_string()).unwrap();
    /// # let transaction = Transaction::default(); // In practice, build a real transaction
    /// let wallet = client.wallet::<Solana>("wallet_id");
    /// let signature = wallet.sign_transaction(&transaction).await.unwrap();
    /// println!("Transaction signature: {}", signature);
    /// # }
    /// ```
    pub async fn sign_transaction(
        &self,
        transaction: &Transaction,
    ) -> Result<SolanaSignature, PrivyError> {
        // Serialize the transaction for sending to Privy
        let tx_bytes = bincode::serialize(transaction)
            .map_err(|e| PrivyError::Config(format!("Failed to serialize transaction: {}", e)))?;

        let input = SolanaSignTransactionRpcInput::default()
            .method(SolanaSignTransactionRpcInputMethod::SignTransaction)
            .chain_type(Some(SolanaSignTransactionRpcInputChainType::Solana))
            .params(
                SolanaSignTransactionRpcInputParams::default()
                    .encoding(SolanaSignTransactionRpcInputParamsEncoding::Base64)
                    .transaction(STANDARD.encode(tx_bytes)),
            )
            .try_into()?;

        let response = self
            .client
            .client
            .wallet_rpc()
            .wallet_id(&self.wallet_id)
            .privy_app_id(&self.client.app_id)
            .body(WalletRpcBody::SolanaSignTransactionRpcInput(input))
            .send()
            .await?;

        let privy_api::types::WalletRpcResponse::SolanaSignTransactionRpcResponse(sign_response) =
            response.into_inner()
        else {
            panic!("invalid response type");
        };

        // The response contains a signed_transaction, not just the signature
        // We need to deserialize it and extract the signature
        let signed_tx_bytes = STANDARD
            .decode(&sign_response.data.signed_transaction)
            .map_err(|e| {
                PrivyError::Config(format!("Failed to decode signed transaction: {}", e))
            })?;

        let signed_tx: Transaction = bincode::deserialize(&signed_tx_bytes).map_err(|e| {
            PrivyError::Config(format!("Failed to deserialize signed transaction: {}", e))
        })?;

        // Extract the first signature from the transaction
        if let Some(signature) = signed_tx.signatures.first() {
            Ok(*signature)
        } else {
            Err(PrivyError::Config(
                "No signature found in signed transaction".to_string(),
            ))
        }
    }

    /// Sign and broadcast a transaction.
    ///
    /// # Arguments
    ///
    /// * `transaction` - The Solana transaction to sign and send
    ///
    /// # Errors
    ///
    /// Returns [`PrivyError`] if the signing or broadcasting fails.
    pub async fn sign_and_send_transaction(
        &self,
        transaction: &Transaction,
    ) -> Result<SolanaSignature, PrivyError> {
        // Serialize the transaction for sending to Privy
        let tx_bytes = bincode::serialize(transaction)
            .map_err(|e| PrivyError::Config(format!("Failed to serialize transaction: {}", e)))?;

        let input = SolanaSignAndSendTransactionRpcInput::default()
            .method(SolanaSignAndSendTransactionRpcInputMethod::SignAndSendTransaction)
            .chain_type(Some(SolanaSignAndSendTransactionRpcInputChainType::Solana))
            .params(
                SolanaSignAndSendTransactionRpcInputParams::default()
                    .encoding(SolanaSignAndSendTransactionRpcInputParamsEncoding::Base64)
                    .transaction(STANDARD.encode(tx_bytes)),
            )
            .try_into()?;

        let response = self
            .client
            .client
            .wallet_rpc()
            .wallet_id(&self.wallet_id)
            .privy_app_id(&self.client.app_id)
            .body(WalletRpcBody::SolanaSignAndSendTransactionRpcInput(input))
            .send()
            .await?;

        let privy_api::types::WalletRpcResponse::SolanaSignAndSendTransactionRpcResponse(
            send_response,
        ) = response.into_inner()
        else {
            panic!("invalid response type");
        };

        // For sign_and_send_transaction, we return the transaction hash instead of signature
        // The response contains the transaction ID/hash, not the signature itself
        if let Some(ref data) = send_response.data {
            // Parse the hash as a signature for compatibility
            // Note: This might not be the actual signature, but the transaction hash
            let hash_bytes = STANDARD.decode(&data.hash).map_err(|e| {
                PrivyError::Config(format!("Failed to decode transaction hash: {}", e))
            })?;

            if hash_bytes.len() == 64 {
                let mut sig_bytes: [u8; 64] = [0; 64];
                sig_bytes.copy_from_slice(&hash_bytes);
                Ok(SolanaSignature::from(sig_bytes))
            } else {
                Err(PrivyError::Config(format!(
                    "Transaction hash has unexpected length: {}",
                    hash_bytes.len()
                )))
            }
        } else {
            Err(PrivyError::Config("No data in response".to_string()))
        }
    }
}
