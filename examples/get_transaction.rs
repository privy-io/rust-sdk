//! Get Transaction Example
//!
//! This example demonstrates how to retrieve transaction details by transaction ID.
//! It shows how to:
//! - Initialize a Privy client with app credentials
//! - Query transaction details using transaction ID
//! - Handle transaction response data
//!
//! ## Required Environment Variables
//! - `PRIVY_APP_ID`: Your Privy app ID
//! - `PRIVY_APP_SECRET`: Your Privy app secret
//! - `PRIVY_TRANSACTION_ID`: The transaction ID to retrieve
//!
//! ## Usage
//! ```bash
//! cargo run --example get_transaction
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

    // Get transaction ID from environment and initialize client
    let transaction_id = std::env::var("PRIVY_TRANSACTION_ID")
        .expect("PRIVY_TRANSACTION_ID environment variable not set");
    let client = PrivyClient::new_from_env()?;

    tracing::info!(
        "initialized privy client from environment, transaction_id: {}",
        transaction_id
    );

    let transaction = client.transactions().get(&transaction_id).await?;

    tracing::info!("got transaction: {:?}", transaction);

    Ok(())
}
