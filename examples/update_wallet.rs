//! Example usage - demonstrates how to use the Privy signer with tk-rs interface

use anyhow::Result;
use base64::{Engine, engine::general_purpose::STANDARD};
use privy_api::types::{
    PublicKeyOwner,
    builder::{OwnerInput, UpdateWalletBody},
};
use privy_rust::{IntoKey, IntoSignature, PrivyApiError, PrivySigner};
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

    let signer = PrivySigner::new(app_id.clone(), app_secret, wallet_id.clone(), public_key)?;

    // Create the request body that will be sent using the generated privy-api type
    let update_wallet_body: privy_api::types::UpdateWalletBody = UpdateWalletBody::default()
        .owner(Some(
            OwnerInput::default()
                .subtype_0(PublicKeyOwner {
                    public_key: include_str!("../public_key.pem").into(),
                })
                .try_into()?,
        ))
        .try_into()?;

    // Serialize the typed body to a generic `serde_json::Value`
    let request_body_json = serde_json::to_value(&update_wallet_body)?;

    // Build the canonical request data for signing using the serialized body
    let canonical_data = signer.build_update_wallet_canonical_request(
        &wallet_id,
        request_body_json,
        // Some(idempotency_key.clone()),
        None,
    )?;

    tracing::info!("canonical request data: {}", canonical_data);

    // Sign the canonical request data (UTF-8 bytes)
    let key = privy_rust::PrivateKeyFromFile("private_key.pem".into());
    let signature = key
        .get_key()
        .await
        .unwrap()
        .sign(canonical_data.as_bytes())
        .await
        .unwrap();

    let privy_authorization_signature = STANDARD.encode(signature.to_bytes());

    tracing::info!("got sig: {:?}", privy_authorization_signature);

    let wallet = match signer
        .update_wallet()
        .wallet_id(wallet_id)
        .body(update_wallet_body)
        .privy_authorization_signature(privy_authorization_signature)
        .send()
        .await
    {
        Ok(wallet) => Ok(wallet),
        Err(PrivyApiError::UnexpectedResponse(r)) => {
            let text = r.text().await.unwrap_or_default();
            tracing::warn!("unexpected response: {:?}", text);
            Err(PrivyApiError::Custom("unexpected response".into()))
        }
        Err(e) => Err(e),
    }?;

    tracing::info!("got new wallet: {:?}", wallet);

    Ok(())
}
