//! Get Wallet Example
//!
//! This example demonstrates how to retrieve a specific wallet by its ID.
//! It shows how to:
//! - Initialize a Privy client with app credentials
//! - Get detailed information about a specific wallet
//! - Handle the response containing wallet data
//!
//! ## Required Environment Variables
//! - `PRIVY_APP_ID`: Your Privy app ID
//! - `PRIVY_APP_SECRET`: Your Privy app secret
//! - `PRIVY_WALLET_ID`: The wallet ID to retrieve
//!
//! ## Usage
//! ```bash
//! cargo run --example get_wallet
//! ```

use anyhow::Result;
use privy_rs::PrivyClient;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Get wallet ID from environment and initialize client
    let wallet_id =
        std::env::var("PRIVY_WALLET_ID").expect("PRIVY_WALLET_ID environment variable not set");
    let client = PrivyClient::new_from_env()?;

    tracing::info!(
        "initialized privy client from environment, wallet_id: {}",
        wallet_id
    );

    let wallet = client.wallets().get(&wallet_id).await?;

    tracing::info!("got wallet: {:?}", wallet);

    Ok(())
}
