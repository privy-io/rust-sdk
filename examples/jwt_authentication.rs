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
use privy_rust::{PrivyClient, generated::types::AuthenticateBody};
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
    let user_jwt =
        std::env::var("PRIVY_USER_JWT").expect("PRIVY_USER_JWT environment variable not set");

    tracing::info!(
        "initializing privy with app_id: {}, app_secret: {}",
        app_id,
        app_secret,
    );

    let client = PrivyClient::new(app_id, app_secret, Default::default())?;

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
