use alloy_consensus::SignableTransaction;
use alloy_network::{TxSigner, TxSignerSync};
use alloy_primitives::{Address, B256, ChainId, Signature};
use alloy_signer::{Result, Signer, SignerSync};

use crate::{AuthorizationContext, subclients::WalletsClient};

/// A Privy wallet that implements Alloy's signer traits
///
/// This allows Privy-managed wallets to be used anywhere Alloy signers are accepted,
/// including transaction signing, message signing, and typed data signing.
///
/// # Example
/// ```no_run
/// use privy_rs::{PrivyClient, AuthorizationContext};
/// use alloy_signer::SignerSync;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = PrivyClient::new_from_env()?;
/// let ctx = AuthorizationContext::new();
///
/// let signer = client.wallets().ethereum().signer("wallet_id", &ctx).await?;
/// let signature = wallet.sign_hash_sync(&hash)?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct PrivyAlloyWallet {
    wallet_id: String,
    address: Address,
    wallets_client: WalletsClient,
    authorization_context: AuthorizationContext,
    chain_id: Option<ChainId>,
}

impl PrivyAlloyWallet {
    /// Create a new Privy Alloy wallet adapter
    ///
    /// # Arguments
    /// * `wallet_id` - The Privy wallet ID
    /// * `address` - The Ethereum address of the wallet
    /// * `wallets_client` - Client for making wallet API calls
    /// * `authorization_context` - Authorization context for signing requests
    pub fn new(
        wallet_id: String,
        address: Address,
        wallets_client: WalletsClient,
        authorization_context: AuthorizationContext,
    ) -> Self {
        Self {
            wallet_id,
            address,
            wallets_client,
            authorization_context,
            chain_id: None,
        }
    }

    /// Set the chain ID for EIP-155 replay protection
    pub fn with_chain_id(mut self, chain_id: ChainId) -> Self {
        self.chain_id = Some(chain_id);
        self
    }

    /// Get the wallet ID
    pub fn wallet_id(&self) -> &str {
        &self.wallet_id
    }
}

impl SignerSync for PrivyAlloyWallet {
    fn sign_hash_sync(&self, hash: &B256) -> Result<Signature> {
        // todo: discuss implementation
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            tokio::task::block_in_place(|| handle.block_on(self.sign_hash(hash)))
        } else {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|e| alloy_signer::Error::other(format!("Failed to build runtime: {e}")))?
                .block_on(self.sign_hash(hash))
        }
    }

    fn chain_id_sync(&self) -> Option<ChainId> {
        self.chain_id
    }
}

impl TxSignerSync<Signature> for PrivyAlloyWallet {
    fn address(&self) -> Address {
        self.address
    }

    fn sign_transaction_sync(
        &self,
        tx: &mut dyn SignableTransaction<Signature>,
    ) -> Result<Signature> {
        let sig_hash = tx.signature_hash();

        self.sign_hash_sync(&sig_hash)
    }
}

#[async_trait::async_trait]
impl Signer for PrivyAlloyWallet {
    async fn sign_hash(&self, hash: &B256) -> Result<Signature> {
        let hash_hex = format!("{hash:#x}");

        let response = match self
            .wallets_client
            .ethereum()
            .sign_secp256k1(
                &self.wallet_id,
                &hash_hex,
                &self.authorization_context,
                Some(&hash_hex), // use digest as idempotency key
            )
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                let msg = match e {
                    crate::PrivySignedApiError::Api(
                        crate::generated::Error::UnexpectedResponse(resp),
                    ) => {
                        let status = resp.status();
                        let body = resp
                            .text()
                            .await
                            .unwrap_or_else(|_| "<body read error>".into());
                        format!("Privy API unexpected response: {status} — {body}")
                    }
                    other => format!("Privy API error: {other}"),
                };
                return Err(alloy_signer::Error::other(msg));
            }
        };

        let wallet_response = response.into_inner();

        let sig_hex = match wallet_response {
            crate::generated::types::WalletRpcResponse::EthereumSecp256k1SignRpcResponse(
                sig_response,
            ) => sig_response.data.signature,
            _ => {
                return Err(alloy_signer::Error::other(
                    "Unexpected response type from Privy API",
                ));
            }
        };

        sig_hex
            .parse::<Signature>()
            .map_err(|e| alloy_signer::Error::other(format!("Failed to parse signature: {e}")))
    }

    fn address(&self) -> Address {
        self.address
    }

    fn chain_id(&self) -> Option<ChainId> {
        self.chain_id
    }

    fn set_chain_id(&mut self, chain_id: Option<ChainId>) {
        self.chain_id = chain_id;
    }
}

#[async_trait::async_trait]
impl TxSigner<Signature> for PrivyAlloyWallet {
    fn address(&self) -> Address {
        self.address
    }

    async fn sign_transaction(
        &self,
        tx: &mut dyn SignableTransaction<Signature>,
    ) -> Result<Signature> {
        let sig_hash = tx.signature_hash();

        self.sign_hash(&sig_hash).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::address;

    fn create_test_wallet() -> PrivyAlloyWallet {
        // Mock components for testing
        let wallet_id = "test_wallet_123".to_string();
        let address = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");

        let client = crate::PrivyClient::new("test_app_id".to_string(), "test_secret".to_string())
            .expect("Failed to create test client");

        let wallets_client = client.wallets();
        let auth_context = AuthorizationContext::new();

        PrivyAlloyWallet::new(wallet_id, address, wallets_client, auth_context)
    }

    #[test]
    fn test_wallet_creation() {
        let wallet = create_test_wallet();

        assert_eq!(wallet.wallet_id(), "test_wallet_123");
        assert_eq!(
            wallet.address,
            address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045")
        );
        assert_eq!(wallet.chain_id, None);
    }

    #[test]
    fn test_with_chain_id() {
        let wallet = create_test_wallet();
        assert_eq!(wallet.chain_id, None);

        let wallet_with_chain = wallet.with_chain_id(1);
        assert_eq!(wallet_with_chain.chain_id, Some(1));

        let wallet_mainnet = create_test_wallet().with_chain_id(1);
        assert_eq!(wallet_mainnet.chain_id, Some(1));

        let wallet_sepolia = create_test_wallet().with_chain_id(11155111);
        assert_eq!(wallet_sepolia.chain_id, Some(11155111));
    }

    #[test]
    fn test_signer_chain_id() {
        let wallet = create_test_wallet();
        assert_eq!(Signer::chain_id(&wallet), None);

        let wallet_with_chain = wallet.with_chain_id(42161); // Arbitrum
        assert_eq!(Signer::chain_id(&wallet_with_chain), Some(42161));
    }

    #[test]
    fn test_set_chain_id() {
        let mut wallet = create_test_wallet();
        assert_eq!(wallet.chain_id, None);

        wallet.set_chain_id(Some(10)); // Optimism
        assert_eq!(wallet.chain_id, Some(10));

        wallet.set_chain_id(None);
        assert_eq!(wallet.chain_id, None);
    }

    #[test]
    fn test_address_from_signer_trait() {
        let wallet = create_test_wallet();
        let expected = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");

        assert_eq!(Signer::address(&wallet), expected);
    }

    #[test]
    fn test_address_from_tx_signer_sync_trait() {
        let wallet = create_test_wallet();
        let expected = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");

        assert_eq!(TxSignerSync::address(&wallet), expected);
    }

    #[test]
    fn test_wallet_id_getter() {
        let wallet = create_test_wallet();
        assert_eq!(wallet.wallet_id(), "test_wallet_123");
    }
}
