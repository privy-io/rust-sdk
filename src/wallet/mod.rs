//! Typed wallet API for compile-time safe blockchain operations.
//!
//! This module provides a type-safe interface for working with Privy wallets,
//! ensuring that blockchain-specific operations can only be called on wallets
//! of the correct type.
//!
//! # Architecture
//!
//! - [`Wallet<T>`] - Generic wallet struct parameterized by blockchain type
//! - [`Chain`] - Trait implemented by blockchain types
//! - [`Solana`] and [`Ethereum`] - Blockchain type markers
//!
//! # Example
//!
//! ```no_run
//! use privy_rust::{
//!     PrivyClient,
//!     wallet::{Ethereum, Solana, Wallet},
//! };
//!
//! let client = PrivyClient::new("app_id".to_string(), "app_secret".to_string())?;
//!
//! // Create a typed Solana wallet
//! let solana_wallet = client.wallet::<Solana>("wallet_id");
//!
//! // Create a typed Ethereum wallet
//! let ethereum_wallet = client.wallet::<Ethereum>("wallet_id");
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use std::str::FromStr;

#[cfg(feature = "ethereum")]
mod eth;
#[cfg(feature = "solana")]
mod sol;

#[cfg(feature = "ethereum")]
pub use eth::Ethereum;
#[cfg(feature = "solana")]
pub use sol::Solana;

use crate::{PrivyClient, PrivyError};

/// Trait implemented by blockchain types to constrain the generic `Wallet<T>`.
///
/// This trait serves as a marker to ensure only valid blockchain types
/// can be used with the `Wallet<T>` struct.
pub trait Chain: Send + Sync + 'static {
    /// The native public key type for this blockchain
    type PublicKey: FromStr + Clone;
    /// The native signature type for this blockchain
    type Signature;
    /// The native transaction type for this blockchain
    type Transaction;
}

/// Typed wallet for blockchain-specific operations.
///
/// The `Wallet<T>` struct provides type-safe access to wallet operations,
/// ensuring that chain-specific methods can only be called on wallets of
/// the correct blockchain type.
///
/// This wraps the existing [`PrivyClient`] and adds type safety through
/// generic parameters.
#[derive(Clone, Debug)]
pub struct Wallet<T: Chain> {
    pub(crate) client: PrivyClient,
    pub(crate) id: String,
    pub(crate) public_key: Option<T::PublicKey>,
}

impl<T: Chain> Wallet<T> {
    /// Create a new typed wallet.
    ///
    /// The public key will be populated lazily on first access.
    ///
    /// # Arguments
    ///
    /// * `client` - The underlying `PrivyClient` instance
    /// * `id` - The Privy wallet ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use privy_rust::{PrivyClient, wallet::{Wallet, Ethereum}};
    /// # let client = PrivyClient::new("app_id".to_string(), "app_secret".to_string())?;
    /// let ethereum_wallet = Wallet::<Ethereum>::new(client, "wallet_id".to_string());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn new(client: PrivyClient, id: String) -> Self {
        Self {
            client,
            id,
            public_key: None,
        }
    }

    /// Get access to the underlying `PrivyClient` for global operations.
    ///
    /// This allows access to all the general wallet management methods
    /// like `get_wallets()`, `create_wallet()`, etc.
    pub fn client(&self) -> &PrivyClient {
        &self.client
    }

    /// Get the wallet ID for this wallet.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the public key for this wallet.
    ///
    /// This method fetches the public key from the Privy API and caches it.
    ///
    /// # Errors
    ///
    /// Returns [`PrivyError`] if the API call fails or the public key cannot be parsed.
    pub async fn pubkey(&self) -> Result<T::PublicKey, PrivyError> {
        if let Some(pk) = &self.public_key {
            return Ok(pk.clone());
        }

        self.client
            .get_wallet()
            .wallet_id(&self.id)
            .send()
            .await
            .map_err(PrivyError::Api)
            .and_then(|resp| {
                T::PublicKey::from_str(&resp.address).map_err(|_| {
                    tracing::error!("Invalid public key returned from Privy API");
                    PrivyError::Config("Invalid public key format".to_string())
                })
            })
    }
}
