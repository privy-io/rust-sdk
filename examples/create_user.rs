//! Example usage - demonstrates how to use the Privy signer with tk-rs interface

use anyhow::Result;
use privy_api::types::{
    LinkedAccountInput,
    builder::{CreateUserBody, LinkedAccountEmailInput},
};
use privy_rust::PrivySigner;
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

    let signer = PrivySigner::new(app_id, app_secret, wallet_id, public_key)?;

    let wallet = signer
        .create_user()
        .body(
            CreateUserBody::default().linked_accounts(vec![LinkedAccountInput::EmailInput(
                LinkedAccountEmailInput::default()
                    .address("alex@arlyon.dev")
                    .type_("email")
                    .try_into()
                    .unwrap(),
            )]),
        )
        .send()
        .await?;

    tracing::info!("got new user: {:?}", wallet);

    Ok(())
}
