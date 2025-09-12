//! Create User Example
//!
//! This example demonstrates how to create a new user in Privy with linked accounts.
//! It shows how to:
//! - Initialize a Privy client with app credentials
//! - Create a user with email and custom JWT linked accounts
//! - Handle the response containing the new user data
//!
//! ## Required Environment Variables
//! - `PRIVY_APP_ID`: Your Privy staging app ID
//! - `PRIVY_APP_SECRET`: Your Privy staging app secret
//!
//! ## Usage
//! ```bash
//! cargo run --example create_user
//! ```

use anyhow::Result;
use privy_rust::{
    PrivyClient,
    generated::types::{
        CreateUserBody, LinkedAccountCustomJwtInput, LinkedAccountCustomJwtInputType,
        LinkedAccountEmailInput, LinkedAccountEmailInputType, LinkedAccountInput,
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

    // Get credentials from environment
    let app_id =
        std::env::var("PRIVY_APP_ID").expect("STAGING_APP_ID environment variable not set");
    let app_secret =
        std::env::var("STAGING_APP_SECRET").expect("PRIVY_APP_SECRET environment variable not set");

    tracing::info!(
        "initializing privy with app_id: {}, app_secret: {}",
        app_id,
        app_secret,
    );

    let client = PrivyClient::new(app_id, app_secret, Default::default())?;

    let user = client
        .users()
        .create(&CreateUserBody {
            linked_accounts: vec![
                LinkedAccountInput::EmailInput(LinkedAccountEmailInput {
                    address: "alex@arlyon.dev".into(),
                    type_: LinkedAccountEmailInputType::Email,
                }),
                LinkedAccountInput::CustomJwtInput(LinkedAccountCustomJwtInput {
                    custom_user_id: "alex@arlyon.dev".try_into().unwrap(),
                    type_: LinkedAccountCustomJwtInputType::CustomAuth,
                }),
            ],
            custom_metadata: None,
            wallets: vec![],
        })
        .await?;

    tracing::info!("got new user: {:?}", user);

    Ok(())
}
