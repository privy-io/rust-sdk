//! JWT Authentication Example
//!
//! This example demonstrates JWT-based authentication for wallet access.
//! Enterprise customers may use external authorization providers to authorize users.
//! This shows how to:
//! - Register a subject on a user record in Privy
//! - Exchange a valid JWT for that subject for a short-lived authorization key
//! - Use the authorization key for wallet operations
//!
//! ## Required Environment Variables
//! - `PRIVY_APP_ID`: Your Privy app ID
//! - `PRIVY_APP_SECRET`: Your Privy app secret
//! - `PRIVY_USER_JWT`: Valid JWT token for the user
//!
//! ## Usage
//! ```bash
//! cargo run --example jwt_authentication
//! ```

use anyhow::Result;
use privy_rs::{PrivyClient, generated::types::AuthenticateBody};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Get JWT from environment and initialize client
    let user_jwt =
        std::env::var("PRIVY_USER_JWT").expect("PRIVY_USER_JWT environment variable not set");
    let client = PrivyClient::new_from_env()?;

    tracing::info!("initialized privy client from environment");

    let jwt_auth = client
        .wallets()
        .authenticate_with_jwt(&AuthenticateBody {
            user_jwt,
            encryption_type: None,
            recipient_public_key: None,
        })
        .await?;

    tracing::info!("got jwt auth: {:?}", jwt_auth);

    Ok(())
}
