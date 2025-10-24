//! Create Ethereum Wallet Example
//!
//! This example creates a new embedded Ethereum wallet for your app.
//! It requires only your app credentials from the environment.
//!
//! Required env vars:
//! - `PRIVY_APP_ID`
//! - `PRIVY_APP_SECRET`
//!
//! Run:
//! ```bash
//! cargo run --example create_wallet_ethereum
//! ```

use anyhow::Result;
use privy_rs::{
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

    let client = PrivyClient::new_from_env()?;
    tracing::info!("initialized privy client from environment");

    let wallet = client
        .wallets()
        .create(
            None,
            &CreateWalletBody {
                chain_type: WalletChainType::Ethereum,
                additional_signers: None,
                owner: None,
                owner_id: None,
                policy_ids: vec![],
            },
        )
        .await?;

    tracing::info!(
        "created wallet: id={}, chain_type={:?}, address={} ",
        wallet.id,
        wallet.chain_type,
        wallet.address
    );

    println!(
        "\nWallet created successfully!\n  id: {}\n  chain: {:?}\n  address: {}\n",
        wallet.id, wallet.chain_type, wallet.address
    );

    Ok(())
}
