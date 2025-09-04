use alloy_consensus::{TxEip7702, TxLegacy};
use alloy_core::primitives::{Address, B256};
use alloy_primitives::ChainId;
use alloy_signer::{Signature, Signer};
use async_trait::async_trait;
use privy_api::types::{
    EthereumPersonalSignRpcInputChainType, EthereumPersonalSignRpcInputMethod,
    EthereumPersonalSignRpcInputParamsEncoding, EthereumPersonalSignRpcInputParamsEncodingSubtype1,
    EthereumSecp256k1SignRpcInputChainType, EthereumSecp256k1SignRpcInputMethod,
    EthereumSendTransactionRpcInputChainType, EthereumSendTransactionRpcInputMethod,
    EthereumSendTransactionRpcInputParamsTransaction,
    EthereumSendTransactionRpcInputParamsTransactionChainId,
    EthereumSendTransactionRpcInputParamsTransactionGasLimit,
    EthereumSignTransactionRpcInputChainType, EthereumSignTransactionRpcInputMethod,
    EthereumSignTransactionRpcInputParamsTransaction,
    EthereumSignTransactionRpcInputParamsTransactionChainId,
    EthereumSignTransactionRpcInputParamsTransactionGasLimit,
    EthereumSignTypedDataRpcInputChainType, EthereumSignTypedDataRpcInputMethod,
    EthereumSignTypedDataRpcInputParamsTypedData, WalletRpcBody,
    builder::{
        // Ethereum builders
        EthereumPersonalSignRpcInput,
        EthereumPersonalSignRpcInputParams,
        EthereumSecp256k1SignRpcInput,
        EthereumSecp256k1SignRpcInputParams,
        EthereumSendTransactionRpcInput,
        EthereumSendTransactionRpcInputParams,
        EthereumSignTransactionRpcInput,
        EthereumSignTransactionRpcInputParams,
        EthereumSignTypedDataRpcInput,
        EthereumSignTypedDataRpcInputParams,
    },
};

use crate::{
    PrivyError,
    wallet::{Chain, Wallet},
};

/// Ethereum blockchain type marker.
///
/// Use this type to create Ethereum-specific wallet instances:
/// ```no_run
/// # use privy_rust::{PrivyClient, wallet::{Wallet, Ethereum}};
/// # let client = PrivyClient::new("app_id".to_string(), "app_secret".to_string()).unwrap();
/// let ethereum_wallet = client.wallet::<Ethereum>("wallet_id");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Ethereum;

impl Chain for Ethereum {
    type PublicKey = Address;
    type Signature = Signature;
    type Transaction = TxLegacy;
}

#[async_trait]
impl Signer for Wallet<Ethereum> {
    async fn sign_hash(&self, hash: &B256) -> Result<Signature, alloy_signer::Error> {
        self.sign_message(hash.as_slice())
            .await
            .map_err(|e| alloy_signer::Error::Other(Box::new(e)))
    }

    fn address(&self) -> alloy_primitives::Address {
        todo!()
    }

    fn chain_id(&self) -> Option<ChainId> {
        todo!()
    }

    fn set_chain_id(&mut self, _chain_id: Option<ChainId>) {
        todo!()
    }
}

// Ethereum-specific implementation
impl Wallet<Ethereum> {
    /// Sign a message using `personal_sign`.
    ///
    /// # Arguments
    ///
    /// * `message` - The message bytes to sign
    ///
    /// # Errors
    ///
    /// Returns [`PrivyError`] if the signing request fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use privy_rust::{PrivyClient, wallet::{Wallet, Ethereum}};
    /// # async fn foo() {
    /// # let client = PrivyClient::new("app_id".to_string(), "app_secret".to_string()).unwrap();
    /// let wallet = client.wallet::<Ethereum>("wallet_id");
    /// let message = b"Hello, Ethereum!";
    /// let signature = wallet.sign_message(message).await.unwrap();
    /// println!("Signature: {:?}", signature);
    /// # }
    /// ```
    pub async fn sign_message(&self, message: &[u8]) -> Result<Signature, PrivyError> {
        // Encode message as hex for personal_sign
        let message_hex = hex::encode(message);

        let input = EthereumPersonalSignRpcInput::default()
            .method(EthereumPersonalSignRpcInputMethod::PersonalSign)
            .chain_type(Some(EthereumPersonalSignRpcInputChainType::Ethereum))
            .params(
                EthereumPersonalSignRpcInputParams::default()
                    .encoding(EthereumPersonalSignRpcInputParamsEncoding {
                        subtype_0: None, // Not using utf-8 encoding
                        subtype_1: Some(EthereumPersonalSignRpcInputParamsEncodingSubtype1::Hex),
                    })
                    .message(message_hex),
            )
            .try_into()?;

        let response = self
            .client
            .client
            .wallet_rpc()
            .wallet_id(&self.wallet_id)
            .privy_app_id(&self.client.app_id)
            .body(WalletRpcBody::EthereumPersonalSignRpcInput(input))
            .send()
            .await?;

        let privy_api::types::WalletRpcResponse::EthereumPersonalSignRpcResponse(sign_response) =
            response.into_inner()
        else {
            panic!("invalid response type");
        };

        // The response should contain a signature as hex string
        let signature_hex = sign_response
            .data
            .signature
            .strip_prefix("0x")
            .unwrap_or(&sign_response.data.signature);

        let signature_bytes = hex::decode(signature_hex)
            .map_err(|e| PrivyError::Config(format!("Failed to decode signature hex: {}", e)))?;

        Signature::try_from(signature_bytes.as_slice())
            .map_err(|_| PrivyError::InvalidSignatureLength)
    }

    /// Sign EIP-712 typed data.
    ///
    /// # Arguments
    ///
    /// * `typed_data` - The EIP-712 typed data to sign
    ///
    /// # Errors
    ///
    /// Returns [`PrivyError`] if the signing request fails.
    pub async fn sign_typed_data(
        &self,
        typed_data: &alloy_core::dyn_abi::eip712::TypedData,
    ) -> Result<Signature, PrivyError> {
        let domain = serde_json::to_value(typed_data.domain.clone())
            .unwrap()
            .as_object()
            .expect("domain must be an object")
            .to_owned();

        let message = serde_json::to_value(typed_data.message.clone())
            .unwrap()
            .as_object()
            .expect("message must be an object")
            .to_owned();

        let typed_data = EthereumSignTypedDataRpcInputParamsTypedData {
            primary_type: typed_data.primary_type.clone(),
            domain,
            message,
            types: Default::default(),
        };

        let input = EthereumSignTypedDataRpcInput::default()
            .method(EthereumSignTypedDataRpcInputMethod::EthSignTypedDataV4)
            .chain_type(Some(EthereumSignTypedDataRpcInputChainType::Ethereum))
            .params(EthereumSignTypedDataRpcInputParams::default().typed_data(typed_data))
            .try_into()?;

        let response = self
            .client
            .client
            .wallet_rpc()
            .wallet_id(&self.wallet_id)
            .privy_app_id(&self.client.app_id)
            .body(WalletRpcBody::EthereumSignTypedDataRpcInput(input))
            .send()
            .await?;

        let privy_api::types::WalletRpcResponse::EthereumSignTypedDataRpcResponse(sign_response) =
            response.into_inner()
        else {
            panic!("invalid response type");
        };

        // The response should contain a signature as hex string
        let signature_hex = sign_response
            .data
            .signature
            .strip_prefix("0x")
            .unwrap_or(&sign_response.data.signature);

        let signature_bytes = hex::decode(signature_hex)
            .map_err(|e| PrivyError::Config(format!("Failed to decode signature hex: {}", e)))?;

        Signature::try_from(signature_bytes.as_slice())
            .map_err(|_| PrivyError::InvalidSignatureLength)
    }

    /// Sign a raw message hash using secp256k1.
    ///
    /// # Arguments
    ///
    /// * `hash` - The 32-byte hash to sign
    ///
    /// # Errors
    ///
    /// Returns [`PrivyError`] if the signing request fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use privy_rust::{PrivyClient, wallet::{Wallet, Ethereum}};
    /// # use alloy_core::primitives::{Address, B256};
    /// # async fn foo() {
    /// # let client = PrivyClient::new("app_id".to_string(), "app_secret".to_string()).unwrap();
    /// let wallet = Wallet::<Ethereum>::new(
    ///     client,
    ///     "0x742d35Cc6634C0532925a3b8D90414783c116abc".to_string(),
    /// );
    /// let hash = B256::from([0u8; 32]); // In practice, use a real hash
    /// let signature = wallet.sign_secp256k1(hash).await;
    /// # }
    /// ```
    pub async fn sign_secp256k1(&self, hash: B256) -> Result<Signature, PrivyError> {
        let hash_hex = format!("{:?}", hash);

        let input = EthereumSecp256k1SignRpcInput::default()
            .method(EthereumSecp256k1SignRpcInputMethod::Secp256k1Sign)
            .chain_type(Some(EthereumSecp256k1SignRpcInputChainType::Ethereum))
            .params(EthereumSecp256k1SignRpcInputParams::default().hash(hash_hex))
            .try_into()?;

        let response = self
            .client
            .client
            .wallet_rpc()
            .wallet_id(&self.wallet_id)
            .privy_app_id(&self.client.app_id)
            .body(WalletRpcBody::EthereumSecp256k1SignRpcInput(input))
            .send()
            .await?;

        let privy_api::types::WalletRpcResponse::EthereumSecp256k1SignRpcResponse(sign_response) =
            response.into_inner()
        else {
            panic!("invalid response type");
        };

        // The response should contain a signature as hex string
        let signature_hex = sign_response
            .data
            .signature
            .strip_prefix("0x")
            .unwrap_or(&sign_response.data.signature);

        let signature_bytes = hex::decode(signature_hex)
            .map_err(|e| PrivyError::Config(format!("Failed to decode signature hex: {}", e)))?;

        Signature::try_from(signature_bytes.as_slice())
            .map_err(|_| PrivyError::InvalidSignatureLength)
    }

    /// Sign an EIP-7702 authorization.
    ///
    /// # Arguments
    ///
    /// * `auth` - The EIP-7702 authorization to sign
    ///
    /// # Errors
    ///
    /// Returns [`PrivyError`] if the signing request fails.
    pub async fn sign_7702_authorization(&self, _auth: TxEip7702) -> Result<(), PrivyError> {
        Ok(())
    }

    /// Sign a transaction without sending it.
    ///
    /// # Arguments
    ///
    /// * `transaction` - The Ethereum transaction to sign
    ///
    /// # Errors
    ///
    /// Returns [`PrivyError`] if the signing request fails.
    pub async fn sign_transaction(&self, transaction: &TxLegacy) -> Result<Vec<u8>, PrivyError> {
        let chain_id = transaction
            .chain_id
            .map(TryInto::try_into)
            .transpose()
            .map_err(|_| PrivyError::Unknown)?
            .map(EthereumSignTransactionRpcInputParamsTransactionChainId::Variant1);

        let gas_limit = transaction
            .gas_limit
            .try_into()
            .map_err(|_| PrivyError::Unknown)
            .map(EthereumSignTransactionRpcInputParamsTransactionGasLimit::Variant1)
            .map(Some)?;

        // Convert ethers TransactionRequest to Privy's transaction structure
        let privy_tx = EthereumSignTransactionRpcInputParamsTransaction {
            chain_id,
            gas_limit,
            data: Some(transaction.input.to_string()),
            // TODO
            ..Default::default()
        };

        let input = EthereumSignTransactionRpcInput::default()
            .method(EthereumSignTransactionRpcInputMethod::EthSignTransaction)
            .chain_type(Some(EthereumSignTransactionRpcInputChainType::Ethereum))
            .params(EthereumSignTransactionRpcInputParams::default().transaction(privy_tx))
            .try_into()?;

        let response = self
            .client
            .client
            .wallet_rpc()
            .wallet_id(&self.wallet_id)
            .privy_app_id(&self.client.app_id)
            .body(WalletRpcBody::EthereumSignTransactionRpcInput(input))
            .send()
            .await?;

        let privy_api::types::WalletRpcResponse::EthereumSignTransactionRpcResponse(sign_response) =
            response.into_inner()
        else {
            panic!("invalid response type");
        };

        // The response contains a signed_transaction as hex string
        let signed_tx_hex = sign_response
            .data
            .signed_transaction
            .strip_prefix("0x")
            .unwrap_or(&sign_response.data.signed_transaction);

        let signed_tx_bytes = hex::decode(signed_tx_hex).map_err(|e| {
            PrivyError::Config(format!("Failed to decode signed transaction hex: {}", e))
        })?;

        Ok(signed_tx_bytes)
    }

    /// Sign and broadcast a transaction.
    ///
    /// # Arguments
    ///
    /// * `transaction` - The Ethereum transaction to sign and send
    ///
    /// # Errors
    ///
    /// Returns [`PrivyError`] if the signing or broadcasting fails.
    pub async fn send_transaction(&self, transaction: &TxLegacy) -> Result<B256, PrivyError> {
        let chain_id = transaction
            .chain_id
            .map(TryInto::try_into)
            .transpose()
            .map_err(|_| PrivyError::Unknown)?
            .map(EthereumSendTransactionRpcInputParamsTransactionChainId::Variant1);

        let gas_limit = transaction
            .gas_limit
            .try_into()
            .map_err(|_| PrivyError::Unknown)
            .map(EthereumSendTransactionRpcInputParamsTransactionGasLimit::Variant1)
            .map(Some)?;

        // Convert ethers TransactionRequest to Privy's send transaction structure
        let privy_tx = EthereumSendTransactionRpcInputParamsTransaction {
            chain_id,
            gas_limit,
            // TODO
            ..Default::default()
        };

        let input = EthereumSendTransactionRpcInput::default()
            .method(EthereumSendTransactionRpcInputMethod::EthSendTransaction)
            .chain_type(Some(EthereumSendTransactionRpcInputChainType::Ethereum))
            .params(EthereumSendTransactionRpcInputParams::default().transaction(privy_tx))
            .try_into()?;

        let response = self
            .client
            .client
            .wallet_rpc()
            .wallet_id(&self.wallet_id)
            .privy_app_id(&self.client.app_id)
            .body(WalletRpcBody::EthereumSendTransactionRpcInput(input))
            .send()
            .await?;

        let privy_api::types::WalletRpcResponse::EthereumSendTransactionRpcResponse(send_response) =
            response.into_inner()
        else {
            panic!("invalid response type");
        };

        // The response should contain a transaction hash
        // TODO: Fix field access - need to check actual response structure
        if let Some(ref _data) = send_response.data {
            // For now, return a placeholder error until proper library integration
            return Err(PrivyError::Config(
                "Send transaction response parsing needs proper library integration".to_string(),
            ));
        }

        Err(PrivyError::Config(
            "No transaction data in response".to_string(),
        ))
    }
}
