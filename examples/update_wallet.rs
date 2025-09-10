//! Example usage - demonstrates how to use the Privy signer with tk-rs interface

use std::sync::Arc;

use anyhow::Result;
use base64::{Engine, engine::general_purpose::STANDARD};
use futures::TryStreamExt;
use privy_api::types::{
    PublicKeyOwner,
    builder::{OwnerInput, UpdateWalletBody},
};
use privy_rust::{
    AuthorizationContext, IntoKey, JwtUser, PrivateKeyFromFile, PrivyApiError, PrivyClient,
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
    let app_id = std::env::var("PRIVY_APP_ID").expect("PRIVY_APP_ID environment variable not set");
    let app_secret =
        std::env::var("PRIVY_APP_SECRET").expect("PRIVY_APP_SECRET environment variable not set");
    let wallet_id =
        std::env::var("PRIVY_WALLET_ID").expect("PRIVY_WALLET_ID environment variable not set");

    tracing::info!(
        "initializing privy with app_id: {}, app_secret: {}, wallet_id: {}",
        app_id,
        app_secret,
        wallet_id,
    );

    let client = PrivyClient::new(app_id.clone(), app_secret)?;

    let key = PrivateKeyFromFile("private_key.pem".into());
    let public_key = key.get_key().await?.0.public_key();

    let client = Arc::new(client);
    let ctx = AuthorizationContext::new();
    ctx.push(PrivateKeyFromFile("private_key.pem".into()));
    ctx.push(JwtUser(client.clone(), "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJhbGV4QGFybHlvbi5kZXYiLCJpYXQiOjEwMDAwMDAwMDAwMH0.IpNgavH95CFZPjkzQW4eyxMIfJ-O_5cIaDyu_6KRXffykjYDRwxTgFJuYq0F6d8wSXf4de-vzfBRWSKMISM3rJdlhximYINGJB14mJFCD87VMLFbTpHIXcv7hc1AAYMPGhOsRkYfYXuvVopKszMvhupmQYJ1npSvKWNeBniIyOHYv4xebZD8L0RVlPvuEKTXTu-CDfs2rMwvD9g_wiBznS3uMF3v_KPaY6x0sx9zeCSxAH9zvhMMtct_Ad9kuoUncGpRzNhEk6JlVccN2Leb1JzbldxSywyS2AApD05u-GFAgFDN3P39V3qgRTGDuuUfUvKQ9S4rbu5El9Qq1CJTeA".to_string()));

    let wallet = client.get_wallet().wallet_id(&wallet_id).send().await?;

    tracing::info!("got wallet: {:?}", wallet);

    // Create the request body that will be sent using the generated privy-api type
    let update_wallet_body: privy_api::types::UpdateWalletBody = UpdateWalletBody::default()
        .owner(Some(
            OwnerInput::default()
                .subtype_0(PublicKeyOwner {
                    public_key: public_key.to_string(),
                })
                // OwnerInput::default()
                //     .subtype_1(UserOwner {
                //         user_id: "did:privy:cmf5wqe2l0005k10blt7x5dq2".to_string(),
                //     })
                .try_into()?,
        ))
        .try_into()?;

    // Build the canonical request data for signing using the serialized body
    let canonical_data = client.build_update_wallet_canonical_request(
        &wallet_id,
        update_wallet_body.clone(),
        // Some(idempotency_key.clone()),
        None,
    )?;

    tracing::info!("canonical request data: {}", canonical_data);

    // Sign the canonical request data (UTF-8 bytes)
    let signature = ctx
        .sign(canonical_data.as_bytes())
        .map_ok(|s| {
            let der_bytes = s.to_der();
            STANDARD.encode(&der_bytes)
        })
        .try_collect::<Vec<_>>()
        .await?
        .join(",");

    let wallet = match client
        .update_wallet()
        .wallet_id(wallet_id)
        .body(update_wallet_body)
        .privy_authorization_signature(signature)
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
