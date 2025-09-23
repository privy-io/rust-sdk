//! Solana wallet operations service.
//!
//! This module provides convenient methods for Solana wallet operations including
//! message signing, transaction signing, and transaction broadcasting. All methods
//! are designed to work with Privy's embedded wallet infrastructure and expect
//! Base64-encoded data following Solana's standard encoding practices.

use std::str::FromStr;

use crate::{
    AuthorizationContext,
    generated::{
        Error, ResponseValue,
        types::{
            SolanaSignAndSendTransactionRpcInput, SolanaSignAndSendTransactionRpcInputCaip2,
            SolanaSignAndSendTransactionRpcInputMethod, SolanaSignAndSendTransactionRpcInputParams,
            SolanaSignAndSendTransactionRpcInputParamsEncoding, SolanaSignMessageRpcInput,
            SolanaSignMessageRpcInputMethod, SolanaSignMessageRpcInputParams,
            SolanaSignMessageRpcInputParamsEncoding, SolanaSignTransactionRpcInput,
            SolanaSignTransactionRpcInputMethod, SolanaSignTransactionRpcInputParams,
            SolanaSignTransactionRpcInputParamsEncoding, WalletRpcBody, WalletRpcResponse,
        },
    },
};

/// Service for Solana-specific wallet operations.
///
/// Provides convenient methods for common Solana wallet operations such as:
/// - Message signing with Base64 encoding
/// - Transaction signing for offline use
/// - Transaction signing and broadcasting in one operation
///
/// All Solana operations expect Base64-encoded data as input, following Solana's
/// standard encoding practices for transactions and messages.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust,no_run
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// use privy_rs::{AuthorizationContext, PrivyClient};
///
/// let client = PrivyClient::new("app_id".to_string(), "app_secret".to_string())?;
/// let solana_service = client.wallets().solana();
/// let auth_ctx = AuthorizationContext::new();
///
/// // Sign a Base64-encoded message
/// let result = solana_service
///     .sign_message(
///         "wallet_id",
///         "SGVsbG8sIFNvbGFuYSE=", // "Hello, Solana!" in Base64
///         &auth_ctx,
///         None, // no idempotency key
///     )
///     .await?;
/// # Ok(())
/// # }
/// ```
pub struct SolanaService {
    wallets_client: crate::subclients::WalletsClient,
}

impl SolanaService {
    /// Creates a new SolanaService instance.
    ///
    /// This is typically called internally by `WalletsClient::solana()`.
    pub(crate) fn new(wallets_client: crate::subclients::WalletsClient) -> Self {
        Self { wallets_client }
    }

    /// Signs a Base64 encoded message for a Solana wallet.
    ///
    /// This method signs arbitrary messages using Solana's message signing standard.
    /// The message must be provided as a Base64-encoded string. This is typically
    /// used for authentication or verification purposes where you need to prove
    /// ownership of a Solana wallet.
    ///
    /// # Parameters
    ///
    /// * `wallet_id` - The ID of the wallet to use for signing
    /// * `message` - The message string to be signed (expected to be Base64 encoded)
    /// * `authorization_context` - The authorization context containing JWT or private keys for request signing
    /// * `idempotency_key` - Optional idempotency key for the request to prevent duplicate operations
    ///
    /// # Returns
    ///
    /// Returns a `ResponseValue<WalletRpcResponse>` containing the signature data.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// use privy_rs::{AuthorizationContext, PrivyClient};
    ///
    /// let client = PrivyClient::new("app_id".to_string(), "app_secret".to_string())?;
    /// let solana_service = client.wallets().solana();
    /// let auth_ctx = AuthorizationContext::new();
    ///
    /// // Base64 encode your message first
    /// let message = base64::encode("Hello, Solana!");
    /// let signature = solana_service
    ///     .sign_message(
    ///         "clz2rqy4500061234abcd1234",
    ///         &message,
    ///         &auth_ctx,
    ///         Some("unique-request-id-456"),
    ///     )
    ///     .await?;
    ///
    /// println!("Message signed successfully");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - The wallet ID is invalid or not found
    /// - The authorization context is invalid
    /// - The message is not properly Base64 encoded
    /// - Network communication fails
    /// - The signing operation fails on the server
    ///
    /// # Notes
    ///
    /// Unlike Ethereum personal message signing, Solana message signing doesn't add
    /// any prefixes to the message. The signature is computed directly over the
    /// decoded message bytes.
    pub async fn sign_message(
        &self,
        wallet_id: &str,
        message: &str,
        authorization_context: &AuthorizationContext,
        idempotency_key: Option<&str>,
    ) -> Result<ResponseValue<WalletRpcResponse>, Error<()>> {
        let rpc_body = WalletRpcBody::SolanaSignMessageRpcInput(SolanaSignMessageRpcInput {
            address: None,
            chain_type: None,
            method: SolanaSignMessageRpcInputMethod::SignMessage,
            params: SolanaSignMessageRpcInputParams {
                encoding: SolanaSignMessageRpcInputParamsEncoding::Base64,
                message: message.to_string(),
            },
        });

        self.wallets_client
            .rpc(wallet_id, authorization_context, idempotency_key, &rpc_body)
            .await
    }

    /// Signs a Solana transaction for a specific wallet.
    ///
    /// This method signs a Solana transaction but does not broadcast it to the network.
    /// The transaction must be provided as a Base64-encoded string representing the
    /// serialized transaction. The signed transaction can be broadcast later using
    /// other tools or the `sign_and_send_transaction` method.
    ///
    /// # Parameters
    ///
    /// * `wallet_id` - The ID of the wallet to use for signing
    /// * `transaction` - The transaction string to be signed (expected to be Base64 encoded)
    /// * `authorization_context` - The authorization context containing JWT or private keys for request signing
    /// * `idempotency_key` - Optional idempotency key for the request
    ///
    /// # Returns
    ///
    /// Returns a `ResponseValue<WalletRpcResponse>` containing the signed transaction data.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// use privy_rs::{AuthorizationContext, PrivyClient};
    ///
    /// let client = PrivyClient::new("app_id".to_string(), "app_secret".to_string())?;
    /// let solana_service = client.wallets().solana();
    /// let auth_ctx = AuthorizationContext::new();
    ///
    /// // Base64-encoded Solana transaction (example)
    /// let transaction = "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAEDArczbMia1tLmq7zz4DinMNN0pJ1JtLdqIJPUw3YrGCzYAMHBsgN27lcgB6H2WQvFgyZuJYHa46puOQo9yQ8CVQbd9uHXZaGT2cvhRs7reawctIXtX1s3kTqM9YV+/wCp20C7Wj2aiuk5TReAXo+VTVg8QTHjs0UjNMMKCvpzZ+ABAgEBARU=";
    ///
    /// let signed_tx = solana_service.sign_transaction(
    ///     "clz2rqy4500061234abcd1234",
    ///     transaction,
    ///     &auth_ctx,
    ///     None
    /// ).await?;
    ///
    /// println!("Transaction signed successfully");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Notes
    ///
    /// - The transaction must be a properly serialized Solana transaction in Base64 format
    /// - The transaction should include all necessary fields (recent blockhash, instructions, etc.)
    /// - This method only signs the transaction; use `sign_and_send_transaction` to also broadcast it
    pub async fn sign_transaction(
        &self,
        wallet_id: &str,
        transaction: &str,
        authorization_context: &AuthorizationContext,
        idempotency_key: Option<&str>,
    ) -> Result<ResponseValue<WalletRpcResponse>, Error<()>> {
        let rpc_body =
            WalletRpcBody::SolanaSignTransactionRpcInput(SolanaSignTransactionRpcInput {
                address: None,
                chain_type: None,
                method: SolanaSignTransactionRpcInputMethod::SignTransaction,
                params: SolanaSignTransactionRpcInputParams {
                    encoding: SolanaSignTransactionRpcInputParamsEncoding::Base64,
                    transaction: transaction.to_string(),
                },
            });

        self.wallets_client
            .rpc(wallet_id, authorization_context, idempotency_key, &rpc_body)
            .await
    }

    /// Signs and sends a Solana transaction.
    ///
    /// This method both signs and broadcasts a Solana transaction to the specified network.
    /// It's a convenience method that combines signing and sending in one operation.
    /// The transaction will be immediately submitted to the Solana network after signing.
    ///
    /// # Parameters
    ///
    /// * `wallet_id` - The ID of the wallet used for the transaction
    /// * `caip2` - The CAIP-2 chain ID of the Solana network (e.g., "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp" for mainnet-beta)
    /// * `transaction` - The transaction string to be signed and sent (expected to be Base64 encoded)
    /// * `authorization_context` - The authorization context containing JWT or private keys for request signing
    /// * `idempotency_key` - Optional idempotency key for the request
    ///
    /// # Returns
    ///
    /// Returns a `ResponseValue<WalletRpcResponse>` containing the transaction signature and other relevant data.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// use privy_rs::{AuthorizationContext, PrivyClient};
    ///
    /// let client = PrivyClient::new("app_id".to_string(), "app_secret".to_string())?;
    /// let solana_service = client.wallets().solana();
    /// let auth_ctx = AuthorizationContext::new();
    ///
    /// // Base64-encoded Solana transaction
    /// let transaction = "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAEDRpb0mdmKftapwzzqUtlcDnuWbw8vwlyiyuWyyieQFKESezu52HWNss0SAcb60ftz7DSpgTwUmfUSl1CYHJ91GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAScgJ7J0AXFr1azCEvB1Y5zpiF4eXR+yTW0UB7am+E/MBAgIAAQwCAAAAQEIPAAAAAAA=";
    ///
    /// let result = solana_service.sign_and_send_transaction(
    ///     "clz2rqy4500061234abcd1234",
    ///     "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp", // Solana mainnet-beta
    ///     transaction,
    ///     &auth_ctx,
    ///     None
    /// ).await?;
    ///
    /// println!("Transaction sent successfully");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - The wallet ID is invalid or not found
    /// - The CAIP-2 chain ID format is invalid
    /// - The transaction format is invalid or corrupted
    /// - The wallet has insufficient balance for the transaction
    /// - Network communication fails
    /// - The transaction is rejected by the Solana network
    ///
    /// # Notes
    ///
    /// - The transaction will be broadcast to the network specified by the CAIP-2 chain ID
    /// - This method requires sufficient SOL balance in the wallet to cover transaction fees
    /// - The transaction will be processed by the Solana network and may take time to confirm
    /// - Common CAIP-2 chain IDs:
    ///   - "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp" (mainnet-beta)
    ///   - "solana:4uhcVJyU9pJkvQyS88uRDiswHXSCkY3z" (testnet)
    ///   - "solana:EtWTRABZaYq6iMfeYKouRu166VU2xqa1" (devnet)
    pub async fn sign_and_send_transaction(
        &self,
        wallet_id: &str,
        caip2: &str,
        transaction: &str,
        authorization_context: &AuthorizationContext,
        idempotency_key: Option<&str>,
    ) -> Result<ResponseValue<WalletRpcResponse>, Error<()>> {
        let caip2_parsed = SolanaSignAndSendTransactionRpcInputCaip2::from_str(caip2)
            .map_err(|_| Error::InvalidRequest("Invalid CAIP-2 format".to_string()))?;

        let rpc_body = WalletRpcBody::SolanaSignAndSendTransactionRpcInput(
            SolanaSignAndSendTransactionRpcInput {
                address: None,
                caip2: caip2_parsed,
                chain_type: None,
                method: SolanaSignAndSendTransactionRpcInputMethod::SignAndSendTransaction,
                params: SolanaSignAndSendTransactionRpcInputParams {
                    encoding: SolanaSignAndSendTransactionRpcInputParamsEncoding::Base64,
                    transaction: transaction.to_string(),
                },
                sponsor: Some(false),
            },
        );

        self.wallets_client
            .rpc(wallet_id, authorization_context, idempotency_key, &rpc_body)
            .await
    }
}
