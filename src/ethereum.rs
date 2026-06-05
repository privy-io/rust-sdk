//! Ethereum wallet operations service.
//!
//! This module provides convenient methods for Ethereum wallet operations including
//! message signing, transaction signing, typed data signing, and more. All methods
//! are designed to work with Privy's embedded wallet infrastructure.

use crate::{
    AuthorizationContext, PrivySignedApiError,
    generated::{
        Error, ResponseValue,
        types::{
            EthereumPersonalSignRpcInput, EthereumPersonalSignRpcInputMethod,
            EthereumPersonalSignRpcInputParams, EthereumPersonalSignRpcInputParamsEncoding,
            EthereumPersonalSignRpcInputParamsMessage, EthereumSecp256k1SignRpcInput,
            EthereumSecp256k1SignRpcInputMethod, EthereumSecp256k1SignRpcInputParams,
            EthereumSendTransactionRpcInput, EthereumSendTransactionRpcInputMethod,
            EthereumSendTransactionRpcInputParams, EthereumSign7702AuthorizationRpcInput,
            EthereumSign7702AuthorizationRpcInputMethod,
            EthereumSign7702AuthorizationRpcInputParams, EthereumSignTransactionRpcInput,
            EthereumSignTransactionRpcInputMethod, EthereumSignTransactionRpcInputParams,
            EthereumSignTypedDataRpcInput, EthereumSignTypedDataRpcInputMethod,
            EthereumSignTypedDataRpcInputParams, EthereumTypedDataInput, Hex,
            UnsignedEthereumTransaction, WalletRpcRequestBody, WalletRpcResponse,
        },
    },
};

/// Options for sending an Ethereum transaction.
///
/// This struct uses `#[non_exhaustive]` to allow new fields to be added in the future
/// without breaking existing code. Always construct using `..Default::default()`:
///
/// ```rust
/// use privy_rs::ethereum::SendTransactionOptions;
///
/// let options = SendTransactionOptions::new().with_sponsor(true);
/// ```
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct SendTransactionOptions {
    /// Whether to sponsor (pay for) the gas fees of this transaction.
    /// - `Some(true)` — enable gas sponsorship
    /// - `Some(false)` — explicitly disable gas sponsorship
    /// - `None` — use the server default
    pub sponsor: Option<bool>,
}

impl SendTransactionOptions {
    /// Creates a new `SendTransactionOptions` with all defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the sponsor option.
    pub fn with_sponsor(mut self, sponsor: bool) -> Self {
        self.sponsor = Some(sponsor);
        self
    }
}

/// Service for Ethereum-specific wallet operations.
///
/// Provides convenient methods for common Ethereum wallet operations such as:
/// - Personal message signing (UTF-8 strings and raw bytes)
/// - secp256k1 signature generation
/// - EIP-712 typed data signing
/// - Transaction signing and broadcasting
/// - EIP-7702 authorization signing
///
/// # Examples
///
/// Basic usage:
///
/// ```rust,no_run
/// # use anyhow::Result;
/// # async fn example() -> Result<()> {
/// use privy_rs::{AuthorizationContext, generated::types::*};
/// # use privy_rs::PrivyClient;
///
/// # let client = PrivyClient::new("app_id".to_string(), "app_secret".to_string())?;
/// let ethereum_service = client.wallets().ethereum();
/// let auth_ctx = AuthorizationContext::new();
///
/// // Sign a UTF-8 message
/// let result = ethereum_service
///     .sign_message(
///         "wallet_id",
///         "Hello, Ethereum!",
///         &auth_ctx,
///         None, // no idempotency key
///     )
///     .await?;
/// # Ok(())
/// # }
/// ```
pub struct EthereumService {
    wallets_client: crate::subclients::WalletsClient,
}

impl EthereumService {
    /// Creates a new [`EthereumService`] instance.
    ///
    /// This is typically called internally by `WalletsClient::ethereum()`.
    pub(crate) fn new(wallets_client: crate::subclients::WalletsClient) -> Self {
        Self { wallets_client }
    }

    /// Signs a UTF-8 encoded message for an Ethereum wallet using the `personal_sign` method.
    ///
    /// This method signs arbitrary UTF-8 text messages using Ethereum's personal message
    /// signing standard. The message will be prefixed with the Ethereum signed message
    /// prefix before signing.
    ///
    /// # Parameters
    ///
    /// * `wallet_id` - The ID of the wallet to use for signing
    /// * `message` - The UTF-8 message string to be signed
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
    /// # use anyhow::Result;
    /// # async fn example() -> Result<()> {
    /// use privy_rs::{AuthorizationContext, generated::types::*};
    /// # use privy_rs::PrivyClient;
    ///
    /// # let client = PrivyClient::new("app_id".to_string(), "app_secret".to_string())?;
    /// let ethereum_service = client.wallets().ethereum();
    /// let auth_ctx = AuthorizationContext::new();
    ///
    /// let signature = ethereum_service
    ///     .sign_message(
    ///         "clz2rqy4500061234abcd1234",
    ///         "Hello, Ethereum!",
    ///         &auth_ctx,
    ///         Some("unique-request-id-123"),
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
    /// - Network communication fails
    /// - The signing operation fails on the server
    pub async fn sign_message(
        &self,
        wallet_id: &str,
        message: &str,
        authorization_context: &AuthorizationContext,
        idempotency_key: Option<&str>,
    ) -> Result<ResponseValue<WalletRpcResponse>, PrivySignedApiError> {
        let rpc_body = WalletRpcRequestBody::EthereumPersonalSignRpcInput(EthereumPersonalSignRpcInput {
            address: None,
            chain_type: None,
            method: EthereumPersonalSignRpcInputMethod::PersonalSign,
            params: EthereumPersonalSignRpcInputParams {
                encoding: EthereumPersonalSignRpcInputParamsEncoding::Utf8,
                message: message.parse::<EthereumPersonalSignRpcInputParamsMessage>()
                    .map_err(|e| Error::InvalidRequest(e.to_string()))?,
            },
            wallet_id: None,
        });

        self.wallets_client
            .rpc(wallet_id, authorization_context, idempotency_key, &rpc_body)
            .await
    }

    /// Signs a raw byte array message for an Ethereum wallet using the `personal_sign` method.
    ///
    /// This method signs raw binary data by first encoding it as a hex string (with 0x prefix)
    /// and then using Ethereum's personal message signing standard.
    ///
    /// # Parameters
    ///
    /// * `wallet_id` - The ID of the wallet to use for signing
    /// * `message` - The message byte array to be signed
    /// * `authorization_context` - The authorization context containing JWT or private keys for request signing
    /// * `idempotency_key` - Optional idempotency key for the request
    ///
    /// # Returns
    ///
    /// Returns a `ResponseValue<WalletRpcResponse>` containing the signature data.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use anyhow::Result;
    /// # async fn example() -> Result<()> {
    /// use privy_rs::{AuthorizationContext, generated::types::*};
    /// # use privy_rs::PrivyClient;
    ///
    /// # let client = PrivyClient::new("app_id".to_string(), "app_secret".to_string())?;
    /// let ethereum_service = client.wallets().ethereum();
    /// let auth_ctx = AuthorizationContext::new();
    ///
    /// let message_bytes = b"Hello, bytes!";
    /// let signature = ethereum_service
    ///     .sign_message_bytes("clz2rqy4500061234abcd1234", message_bytes, &auth_ctx, None)
    ///     .await?;
    ///
    /// println!("Byte message signed successfully");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn sign_message_bytes(
        &self,
        wallet_id: &str,
        message: &[u8],
        authorization_context: &AuthorizationContext,
        idempotency_key: Option<&str>,
    ) -> Result<ResponseValue<WalletRpcResponse>, PrivySignedApiError> {
        let hex_message = format!("0x{}", hex::encode(message));

        let rpc_body = WalletRpcRequestBody::EthereumPersonalSignRpcInput(EthereumPersonalSignRpcInput {
            address: None,
            chain_type: None,
            method: EthereumPersonalSignRpcInputMethod::PersonalSign,
            params: EthereumPersonalSignRpcInputParams {
                encoding: EthereumPersonalSignRpcInputParamsEncoding::Hex,
                message: hex_message.parse::<EthereumPersonalSignRpcInputParamsMessage>()
                    .map_err(|e| Error::InvalidRequest(e.to_string()))?,
            },
            wallet_id: None,
        });

        self.wallets_client
            .rpc(wallet_id, authorization_context, idempotency_key, &rpc_body)
            .await
    }

    /// Signs a message using secp256k1 signature algorithm.
    ///
    /// This method performs low-level secp256k1 signing on a pre-computed hash.
    /// The hash should be exactly 32 bytes and is typically the result of keccak256
    /// hashing of the data to be signed.
    ///
    /// # Parameters
    ///
    /// * `wallet_id` - The ID of the wallet to use for signing
    /// * `hash` - The hash to sign (typically 32 bytes as hex string with 0x prefix)
    /// * `authorization_context` - The authorization context containing JWT or private keys for request signing
    /// * `idempotency_key` - Optional idempotency key for the request
    ///
    /// # Returns
    ///
    /// Returns a `ResponseValue<WalletRpcResponse>` containing the secp256k1 signature.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use anyhow::Result;
    /// # async fn example() -> Result<()> {
    /// use privy_rs::{AuthorizationContext, generated::types::*};
    /// # use privy_rs::PrivyClient;
    ///
    /// # let client = PrivyClient::new("app_id".to_string(), "app_secret".to_string())?;
    /// let ethereum_service = client.wallets().ethereum();
    /// let auth_ctx = AuthorizationContext::new();
    ///
    /// // Pre-computed keccak256 hash
    /// let hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
    /// let signature = ethereum_service
    ///     .sign_secp256k1("clz2rqy4500061234abcd1234", hash, &auth_ctx, None)
    ///     .await?;
    ///
    /// println!("Hash signed with secp256k1");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Notes
    ///
    /// This is a lower-level signing method. For most use cases, prefer `sign_message()`
    /// or `sign_typed_data()` which handle the hashing automatically.
    pub async fn sign_secp256k1(
        &self,
        wallet_id: &str,
        hash: &str,
        authorization_context: &AuthorizationContext,
        idempotency_key: Option<&str>,
    ) -> Result<ResponseValue<WalletRpcResponse>, PrivySignedApiError> {
        let rpc_body =
            WalletRpcRequestBody::EthereumSecp256k1SignRpcInput(EthereumSecp256k1SignRpcInput {
                address: None,
                chain_type: None,
                method: EthereumSecp256k1SignRpcInputMethod::Secp256k1Sign,
                params: EthereumSecp256k1SignRpcInputParams {
                    hash: hash.parse::<Hex>()
                        .map_err(|e| Error::InvalidRequest(e.to_string()))?,
                },
                wallet_id: None,
            });

        self.wallets_client
            .rpc(wallet_id, authorization_context, idempotency_key, &rpc_body)
            .await
    }

    /// Signs a 7702 authorization using the eth_sign7702Authorization RPC method.
    ///
    /// EIP-7702 introduces account abstraction by allowing EOAs to temporarily delegate
    /// control to a smart contract. This method signs the authorization that allows
    /// the delegation to take place.
    ///
    /// # Parameters
    ///
    /// * `wallet_id` - The ID of the wallet to use for signing
    /// * `params` - The parameters for the eth_sign7702Authorization RPC method including contract address, chain ID, and nonce
    /// * `authorization_context` - The authorization context containing JWT or private keys for request signing
    /// * `idempotency_key` - Optional idempotency key for the request
    ///
    /// # Returns
    ///
    /// Returns a `ResponseValue<WalletRpcResponse>` containing the signed authorization data.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use anyhow::Result;
    /// # async fn example() -> Result<()> {
    /// use privy_rs::{AuthorizationContext, generated::types::*};
    /// # use privy_rs::PrivyClient;
    ///
    /// # let client = PrivyClient::new("app_id".to_string(), "app_secret".to_string())?;
    /// let ethereum_service = client.wallets().ethereum();
    /// let auth_ctx = AuthorizationContext::new();
    ///
    /// let params = EthereumSign7702AuthorizationRpcInputParams {
    ///     chain_id: Quantity::Integer(1),
    ///     contract: "0x1234567890abcdef1234567890abcdef12345678".into(),
    ///     executor: None,
    ///     nonce: None,
    /// };
    ///
    /// let authorization = ethereum_service
    ///     .sign_7702_authorization("clz2rqy4500061234abcd1234", params, &auth_ctx, None)
    ///     .await?;
    ///
    /// println!("7702 authorization signed successfully");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn sign_7702_authorization(
        &self,
        wallet_id: &str,
        params: EthereumSign7702AuthorizationRpcInputParams,
        authorization_context: &AuthorizationContext,
        idempotency_key: Option<&str>,
    ) -> Result<ResponseValue<WalletRpcResponse>, PrivySignedApiError> {
        let rpc_body = WalletRpcRequestBody::EthereumSign7702AuthorizationRpcInput(
            EthereumSign7702AuthorizationRpcInput {
                address: None,
                chain_type: None,
                method: EthereumSign7702AuthorizationRpcInputMethod::EthSign7702Authorization,
                params,
                wallet_id: None,
            },
        );

        self.wallets_client
            .rpc(wallet_id, authorization_context, idempotency_key, &rpc_body)
            .await
    }

    /// Signs typed data using EIP-712 standard.
    ///
    /// EIP-712 defines a standard for typed structured data signing that provides
    /// better UX and security compared to signing arbitrary strings. This method
    /// implements the `eth_signTypedData_v4` RPC method.
    ///
    /// # Parameters
    ///
    /// * `wallet_id` - The ID of the wallet to use for signing
    /// * `typed_data` - The typed data structure to be signed, conforming to EIP-712 format
    /// * `authorization_context` - The authorization context containing JWT or private keys for request signing
    /// * `idempotency_key` - Optional idempotency key for the request
    ///
    /// # Returns
    ///
    /// Returns a `ResponseValue<WalletRpcResponse>` containing the signed typed data.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use anyhow::Result;
    /// # async fn example() -> Result<()> {
    /// use privy_rs::{AuthorizationContext, generated::types::*};
    /// # use privy_rs::PrivyClient;
    ///
    /// # let client = PrivyClient::new("app_id".to_string(), "app_secret".to_string())?;
    /// let ethereum_service = client.wallets().ethereum();
    /// let auth_ctx = AuthorizationContext::new();
    ///
    /// // Create EIP-712 typed data structure
    /// let typed_data = EthereumTypedDataInput {
    ///     domain: TypedDataDomainInputParams(serde_json::Map::new()),
    ///     message: serde_json::Map::new(),
    ///     primary_type: "Mail".to_string(),
    ///     types: TypedDataTypesInputParams(std::collections::HashMap::new()),
    /// };
    ///
    /// let signature = ethereum_service
    ///     .sign_typed_data("clz2rqy4500061234abcd1234", typed_data, &auth_ctx, None)
    ///     .await?;
    ///
    /// println!("Typed data signed successfully");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Notes
    ///
    /// The typed data must conform to the EIP-712 specification with proper domain,
    /// types, primaryType, and message fields. Refer to EIP-712 for the complete
    /// specification of the required structure.
    pub async fn sign_typed_data(
        &self,
        wallet_id: &str,
        typed_data: EthereumTypedDataInput,
        authorization_context: &AuthorizationContext,
        idempotency_key: Option<&str>,
    ) -> Result<ResponseValue<WalletRpcResponse>, PrivySignedApiError> {
        let rpc_body =
            WalletRpcRequestBody::EthereumSignTypedDataRpcInput(EthereumSignTypedDataRpcInput {
                address: None,
                chain_type: None,
                method: EthereumSignTypedDataRpcInputMethod::EthSignTypedDataV4,
                params: EthereumSignTypedDataRpcInputParams { typed_data },
                wallet_id: None,
            });

        self.wallets_client
            .rpc(wallet_id, authorization_context, idempotency_key, &rpc_body)
            .await
    }

    /// Signs a transaction using the eth_signTransaction method.
    ///
    /// This method signs an Ethereum transaction but does not broadcast it to the network.
    /// The signed transaction can be broadcast later using other tools or the `send_transaction` method.
    ///
    /// # Parameters
    ///
    /// * `wallet_id` - The ID of the wallet to use for signing
    /// * `transaction` - The transaction object to be signed including to, value, data, gas, etc.
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
    /// # use anyhow::Result;
    /// # async fn example() -> Result<()> {
    /// use privy_rs::{AuthorizationContext, generated::types::*};
    /// # use privy_rs::PrivyClient;
    ///
    /// # let client = PrivyClient::new("app_id".to_string(), "app_secret".to_string())?;
    /// let ethereum_service = client.wallets().ethereum();
    /// let auth_ctx = AuthorizationContext::new();
    ///
    /// let transaction = UnsignedStandardEthereumTransaction {
    ///     to: Some("0x742d35Cc6635C0532925a3b8c17d6d1E9C2F7ca".to_string()),
    ///     value: None,
    ///     gas_limit: None,
    ///     gas_price: None,
    ///     nonce: None,
    ///     chain_id: None,
    ///     data: None,
    ///     from: None,
    ///     max_fee_per_gas: None,
    ///     max_priority_fee_per_gas: None,
    ///     type_: None,
    ///     authorization_list: vec![],
    /// };
    ///
    /// let signed_tx = ethereum_service
    ///     .sign_transaction("clz2rqy4500061234abcd1234", transaction.into(), &auth_ctx, None)
    ///     .await?;
    ///
    /// println!("Transaction signed successfully");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn sign_transaction(
        &self,
        wallet_id: &str,
        transaction: UnsignedEthereumTransaction,
        authorization_context: &AuthorizationContext,
        idempotency_key: Option<&str>,
    ) -> Result<ResponseValue<WalletRpcResponse>, PrivySignedApiError> {
        let rpc_body =
            WalletRpcRequestBody::EthereumSignTransactionRpcInput(EthereumSignTransactionRpcInput {
                address: None,
                chain_type: None,
                method: EthereumSignTransactionRpcInputMethod::EthSignTransaction,
                params: EthereumSignTransactionRpcInputParams { transaction },
                wallet_id: None,
            });

        self.wallets_client
            .rpc(wallet_id, authorization_context, idempotency_key, &rpc_body)
            .await
    }

    /// Signs and sends a transaction using the eth_sendTransaction method.
    ///
    /// This method both signs and broadcasts an Ethereum transaction to the specified network.
    /// It's a convenience method that combines signing and sending in one operation.
    ///
    /// # Parameters
    ///
    /// * `wallet_id` - The ID of the wallet used for the transaction
    /// * `caip2` - The CAIP-2 chain ID of the Ethereum network (e.g., "eip155:1" for Ethereum Mainnet, "eip155:11155111" for Sepolia)
    /// * `transaction` - The transaction object to be sent
    /// * `authorization_context` - The authorization context containing JWT or private keys for request signing
    /// * `idempotency_key` - Optional idempotency key for the request
    ///
    /// # Returns
    ///
    /// Returns a `ResponseValue<WalletRpcResponse>` containing the transaction hash or other relevant data.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use anyhow::Result;
    /// # async fn example() -> Result<()> {
    /// use privy_rs::{AuthorizationContext, generated::types::*};
    /// # use privy_rs::PrivyClient;
    ///
    /// # let client = PrivyClient::new("app_id".to_string(), "app_secret".to_string())?;
    /// let ethereum_service = client.wallets().ethereum();
    /// let auth_ctx = AuthorizationContext::new();
    ///
    /// let transaction = UnsignedStandardEthereumTransaction {
    ///     to: Some("0x742d35Cc6635C0532925a3b8c17d6d1E9C2F7ca".to_string()),
    ///     value: None,
    ///     gas_limit: None,
    ///     max_fee_per_gas: None,
    ///     max_priority_fee_per_gas: None,
    ///     data: None,
    ///     chain_id: None,
    ///     from: None,
    ///     gas_price: None,
    ///     nonce: None,
    ///     type_: None,
    ///     authorization_list: vec![],
    /// };
    ///
    /// let result = ethereum_service
    ///     .send_transaction(
    ///         "clz2rqy4500061234abcd1234",
    ///         "eip155:1",
    ///         transaction.into(),
    ///         &auth_ctx,
    ///         None,
    ///     )
    ///     .await?;
    ///
    /// println!("Transaction sent successfully");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Notes
    ///
    /// - The transaction will be broadcast to the network specified by the CAIP-2 chain ID
    /// - This method requires sufficient balance in the wallet to cover gas costs and transfer value
    /// - The transaction will be mined and included in a block if successful
    /// - Common CAIP-2 chain IDs: "eip155:1" (Ethereum), "eip155:137" (Polygon), "eip155:11155111" (Sepolia testnet)
    pub async fn send_transaction(
        &self,
        wallet_id: &str,
        caip2: &str,
        transaction: UnsignedEthereumTransaction,
        authorization_context: &AuthorizationContext,
        idempotency_key: Option<&str>,
    ) -> Result<ResponseValue<WalletRpcResponse>, PrivySignedApiError> {
        self.send_transaction_with_options(
            wallet_id,
            caip2,
            transaction,
            authorization_context,
            idempotency_key,
            &SendTransactionOptions::default(),
        )
        .await
    }

    /// Signs and sends a transaction with additional options such as gas sponsorship.
    ///
    /// This method is identical to [`send_transaction`](Self::send_transaction) but accepts
    /// a [`SendTransactionOptions`] struct for additional configuration.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use anyhow::Result;
    /// # async fn example() -> Result<()> {
    /// use privy_rs::{AuthorizationContext, ethereum::SendTransactionOptions, generated::types::*};
    /// # use privy_rs::PrivyClient;
    ///
    /// # let client = PrivyClient::new("app_id".to_string(), "app_secret".to_string())?;
    /// let ethereum_service = client.wallets().ethereum();
    /// let auth_ctx = AuthorizationContext::new();
    ///
    /// let transaction: UnsignedEthereumTransaction = UnsignedStandardEthereumTransaction {
    ///     to: Some("0x742d35Cc6635C0532925a3b8c17d6d1E9C2F7ca".to_string()),
    ///     value: None, gas_limit: None, max_fee_per_gas: None,
    ///     max_priority_fee_per_gas: None, data: None, chain_id: None,
    ///     from: None, gas_price: None, nonce: None, type_: None,
    ///     authorization_list: vec![],
    /// }.into();
    ///
    /// let options = SendTransactionOptions::new().with_sponsor(true);
    ///
    /// let result = ethereum_service
    ///     .send_transaction_with_options(
    ///         "wallet_id",
    ///         "eip155:1",
    ///         transaction,
    ///         &auth_ctx,
    ///         None,
    ///         &options,
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_transaction_with_options(
        &self,
        wallet_id: &str,
        caip2: &str,
        transaction: UnsignedEthereumTransaction,
        authorization_context: &AuthorizationContext,
        idempotency_key: Option<&str>,
        options: &SendTransactionOptions,
    ) -> Result<ResponseValue<WalletRpcResponse>, PrivySignedApiError> {
        let rpc_body =
            WalletRpcRequestBody::EthereumSendTransactionRpcInput(EthereumSendTransactionRpcInput {
                address: None,
                caip2: caip2
                    .parse()
                    .map_err(|_| Error::InvalidRequest("Invalid CAIP-2 format".to_string()))?,
                chain_type: None,
                experimental_data_suffix: None,
                method: EthereumSendTransactionRpcInputMethod::EthSendTransaction,
                params: EthereumSendTransactionRpcInputParams { transaction },
                reference_id: None,
                sponsor: options.sponsor,
                wallet_id: None,
            });

        self.wallets_client
            .rpc(wallet_id, authorization_context, idempotency_key, &rpc_body)
            .await
    }

    /// Create an Alloy-compatible signer for this wallet
    ///
    /// This returns a `PrivyAlloyWallet` that implements Alloy's signer traits,
    /// allowing it to be used with any Alloy-compatible library.
    ///
    /// # Feature Flag
    /// Requires the `alloy` feature to be enabled.
    ///
    /// # Example
    /// ```no_run
    /// use privy_rs::{PrivyClient, AuthorizationContext, PrivateKey};
    /// use alloy_signer::SignerSync;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = PrivyClient::new_from_env()?;
    /// let private_key = std::fs::read_to_string("private_key.pem")?;
    /// let ctx = AuthorizationContext::new().push(PrivateKey(private_key));
    ///
    /// let signer = client.wallets().ethereum().alloy("wallet_id", &ctx).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "alloy")]
    pub async fn alloy(
        &self,
        wallet_id: &str,
        authorization_context: &AuthorizationContext,
    ) -> Result<crate::alloy::PrivyAlloyWallet, crate::PrivyApiError> {
        let wallet_response = self.wallets_client.get(wallet_id).await?;
        let wallet = wallet_response.into_inner();

        let address = wallet.address.parse().map_err(|e| {
            crate::PrivyApiError::InvalidRequest(format!("Failed to parse wallet address: {e}"))
        })?;

        Ok(crate::alloy::PrivyAlloyWallet::new(
            wallet_id.to_string(),
            address,
            self.wallets_client.clone(),
            authorization_context.clone(),
        ))
    }
}
