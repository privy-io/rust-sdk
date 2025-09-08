//! Example usage - demonstrates how to use the Privy signer with tk-rs interface

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
        std::env::var("STAGING_APP_ID").expect("STAGING_APP_ID environment variable not set");
    let app_secret = std::env::var("STAGING_APP_SECRET")
        .expect("STAGING_APP_SECRET environment variable not set");

    tracing::info!(
        "initializing privy with app_id: {}, app_secret: {}",
        app_id,
        app_secret,
    );

    let client = PrivyClient::new(app_id, app_secret, Default::default())?;

    let user = client
        .create_user(&CreateUserBody {
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
