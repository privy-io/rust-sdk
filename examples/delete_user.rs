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
use privy_rs::PrivyClient;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Initialize client from environment variables
    let client = PrivyClient::new_from_env()?;

    tracing::info!("initialized privy client from environment");

    let user = client.users().delete("cmf56qacr01qpl90brxql83lx").await?;

    tracing::info!("deleted user: {:?}", user);

    Ok(())
}
