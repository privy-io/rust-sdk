//! solana
//!
//! Demonstrates getting a solana wallet and sending an RPC

use anyhow::Result;
use privy_rust::{PrivyClient, wallet::Solana};
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
    let wallet_id =
        std::env::var("PRIVY_WALLET_ID").expect("PRIVY_WALLET_ID environment variable not set");
    let public_key =
        std::env::var("PRIVY_PUBLIC_KEY").expect("PRIVY_PUBLIC_KEY environment variable not set");

    tracing::info!(
        "initializing privy with app_id: {}, app_secret: {}, wallet_id: {}, public_key: {}",
        app_id,
        app_secret,
        wallet_id,
        public_key
    );

    let client = PrivyClient::new(app_id, app_secret)?;
    let wallet = client.wallet::<Solana>(&wallet_id);

    {
        let key = wallet.pubkey().await?;
        tracing::info!("Public key: {:?}", key);
    }

    let message = b"Hello, Privy!";
    let signature = wallet.sign_message(message).await?;
    tracing::info!("Signature: {:?}", signature);

    Ok(())
}
