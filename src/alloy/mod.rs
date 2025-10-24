//! Alloy integration for Privy wallets
//!
//! This module provides implementations of Alloy signer traits, allowing
//! Privy wallets to be used seamlessly within the Alloy ecosystem.
//!
//! # Feature Flag
//! This module is only available when the `alloy` feature is enabled.
//!
//! # Example
//! ```no_run
//! use privy_rs::{PrivyClient, AuthorizationContext, PrivateKey};
//! use alloy_signer::SignerSync;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = PrivyClient::new_from_env()?;
//! let private_key = std::fs::read_to_string("private_key.pem")?;
//! let ctx = AuthorizationContext::new().push(PrivateKey(private_key));
//!
//! let signer = client.wallets().ethereum().signer("wallet_id", &ctx).await?;
//! // Use signer with any Alloy-compatible library
//! # Ok(())
//! # }
//! ```

mod signer;

pub use signer::PrivyAlloyWallet;
