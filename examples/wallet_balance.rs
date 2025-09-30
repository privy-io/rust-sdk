//! Wallet Balance Example
//!
//! This example demonstrates how to retrieve a wallet's balance for specific assets.
//! It shows how to:
//! - Initialize a Privy client with app credentials
//! - Query wallet balance for specific assets and chains
//! - Include currency conversion data in responses
//!
//! ## Required Environment Variables
//! - `PRIVY_APP_ID`: Your Privy app ID
//! - `PRIVY_APP_SECRET`: Your Privy app secret
//! - `PRIVY_WALLET_ID`: The wallet ID to check balance for
//!
//! ## Usage
//! ```bash
//! cargo run --example wallet_balance
//! ```

use anyhow::Result;
use privy_rs::{
    PrivyClient,
    generated::types::{
        GetWalletBalanceAsset, GetWalletBalanceAssetString, GetWalletBalanceChain,
        GetWalletBalanceChainString, GetWalletBalanceIncludeCurrency,
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

    // Get SOL balance on Solana
    let balance = client
        .wallets()
        .balance()
        .get(
            &wallet_id,
            &GetWalletBalanceAsset::String(GetWalletBalanceAssetString::Sol),
            &GetWalletBalanceChain::String(GetWalletBalanceChainString::Solana),
            Some(GetWalletBalanceIncludeCurrency::Usd),
        )
        .await?;

    tracing::info!("got wallet balance: {:?}", balance);

    Ok(())
}
