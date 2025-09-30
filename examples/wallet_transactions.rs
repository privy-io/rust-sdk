//! Wallet Transactions Example
//!
//! This example demonstrates how to retrieve a wallet's transaction history.
//! It shows how to:
//! - Initialize a Privy client with app credentials
//! - Query transaction history for specific assets and chains
//! - Handle pagination with cursors and limits
//!
//! ## Required Environment Variables
//! - `PRIVY_APP_ID`: Your Privy app ID
//! - `PRIVY_APP_SECRET`: Your Privy app secret
//! - `PRIVY_WALLET_ID`: The wallet ID to get transactions for
//!
//! ## Usage
//! ```bash
//! cargo run --example wallet_transactions
//! ```

use anyhow::Result;
use privy_rs::{
    PrivyClient,
    generated::types::{
        WalletTransactionsAsset, WalletTransactionsAssetString, WalletTransactionsChain,
    },
};
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

    // Get SOL transactions on Solana mainnet
    let transactions = client
        .wallets()
        .transactions()
        .get(
            &wallet_id,
            &WalletTransactionsAsset::String(WalletTransactionsAssetString::Sol),
            WalletTransactionsChain::Base,
            None,       // No cursor for first page
            Some(10.0), // Limit to 10 transactions
        )
        .await?;

    tracing::info!("got wallet transactions: {:?}", transactions);

    Ok(())
}
