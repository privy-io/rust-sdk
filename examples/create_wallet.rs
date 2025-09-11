//! Create Wallet Example
//!
//! This example demonstrates how to create a new embedded wallet for a user.
//! It shows how to:
//! - Initialize a Privy client with app credentials
//! - Create a wallet for a specific user with chain configuration
//! - Handle the response containing the new wallet data
//!
//! ## Required Environment Variables
//! - `PRIVY_APP_ID`: Your Privy app ID
//! - `PRIVY_APP_SECRET`: Your Privy app secret
//! - `PRIVY_USER_ID`: The user ID to create a wallet for
//!
//! ## Usage
//! ```bash
//! cargo run --example create_wallet
//! ```

use anyhow::Result;
use privy_rust::{
    PrivyClient,
    generated::types::{CreateWalletBody, WalletChainType},
};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Get credentials from environment
    let app_id = std::env::var("PRIVY_APP_ID").expect("PRIVY_APP_ID environment variable not set");
    let app_secret =
        std::env::var("PRIVY_APP_SECRET").expect("PRIVY_APP_SECRET environment variable not set");

    tracing::info!(
        "initializing privy with app_id: {}, app_secret: {}",
        app_id,
        app_secret,
    );

    let client = PrivyClient::new(app_id, app_secret)?;

    let wallet = client
        .wallets()
        .create(
            None,
            &CreateWalletBody {
                chain_type: WalletChainType::Solana,
                additional_signers: None,
                owner: None,
                owner_id: None,
                policy_ids: vec![],
            },
        )
        .await?;

    tracing::info!("got new wallet: {:?}", wallet);

    Ok(())
}
