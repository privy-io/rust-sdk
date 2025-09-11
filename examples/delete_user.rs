//! Delete User Example
//!
//! This example demonstrates how to delete a user from your Privy app.
//! It shows how to:
//! - Initialize a Privy client with app credentials
//! - Delete a user by user ID
//! - Handle the deletion response
//!
//! ## Required Environment Variables
//! - `PRIVY_APP_ID`: Your Privy app ID
//! - `PRIVY_APP_SECRET`: Your Privy app secret
//! - `PRIVY_USER_ID`: The user ID to delete
//!
//! ## Usage
//! ```bash
//! cargo run --example delete_user
//! ```

use anyhow::Result;
use privy_rust::PrivyClient;
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

    let user = client.users().delete("cmf56qacr01qpl90brxql83lx").await?;

    tracing::info!("deleted user: {:?}", user);

    Ok(())
}
