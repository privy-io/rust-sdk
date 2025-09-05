//! Example usage - demonstrates how to use the Privy signer with tk-rs interface

use anyhow::Result;
use privy_api::types::{
    LinkedAccountInput,
    builder::{CreateUserBody, LinkedAccountCustomJwtInput, LinkedAccountEmailInput},
};
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
    let app_id =
        std::env::var("STAGING_APP_ID").expect("STAGING_APP_ID environment variable not set");
    let app_secret = std::env::var("STAGING_APP_SECRET")
        .expect("STAGING_APP_SECRET environment variable not set");

    tracing::info!(
        "initializing privy with app_id: {}, app_secret: {}",
        app_id,
        app_secret,
    );

    let client = PrivyClient::new_with_url(app_id, app_secret, "https://api.staging.privy.io")?;

    let wallet = match client
        .create_user()
        .body(CreateUserBody::default().linked_accounts(vec![
                LinkedAccountInput::EmailInput(
                    LinkedAccountEmailInput::default()
                        .address("alex@arlyon.dev")
                        .type_("email")
                        .try_into()
                        .unwrap(),
                ),
                LinkedAccountInput::CustomJwtInput(
                    LinkedAccountCustomJwtInput::default()
                        .custom_user_id("alex@arlyon.dev")
                        .type_("custom_auth")
                        .try_into()
                        .unwrap(),
                ),
            ]))
        .send()
        .await
    {
        Ok(r) => Ok(r.into_inner()),
        Err(privy_api::Error::UnexpectedResponse(response)) => {
            tracing::error!("unexpected response {:?}", response.text().await);
            Err(privy_api::Error::Custom("whoops".to_string()))
        }
        Err(e) => {
            tracing::error!("error {:?}", e);
            Err(e)
        }
    }?;

    tracing::info!("got new user: {:?}", wallet);

    Ok(())
}
