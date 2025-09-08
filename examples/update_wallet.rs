//! Example usage - demonstrates how to use the Privy signer with tk-rs interface

use std::sync::Arc;

use anyhow::Result;
use base64::{Engine, engine::general_purpose::STANDARD};
use futures::TryStreamExt;
use httpclient::ProtocolError;
use privy_libninja::model::{OwnerInput, PublicKeyOwner};
use privy_rust::{AuthorizationContext, IntoKey, JwtUser, PrivateKeyFromFile, PrivyClient};
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
    let public_key = key.get_key().await?.public_key();

    let client = Arc::new(client);
    let mut ctx = AuthorizationContext::new();
    ctx.push(PrivateKeyFromFile("private_key.pem".into()));
    ctx.push(JwtUser(client.clone(), "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJhbGV4QGFybHlvbi5kZXYiLCJpYXQiOjEwMDAwMDAwMDAwMH0.IpNgavH95CFZPjkzQW4eyxMIfJ-O_5cIaDyu_6KRXffykjYDRwxTgFJuYq0F6d8wSXf4de-vzfBRWSKMISM3rJdlhximYINGJB14mJFCD87VMLFbTpHIXcv7hc1AAYMPGhOsRkYfYXuvVopKszMvhupmQYJ1npSvKWNeBniIyOHYv4xebZD8L0RVlPvuEKTXTu-CDfs2rMwvD9g_wiBznS3uMF3v_KPaY6x0sx9zeCSxAH9zvhMMtct_Ad9kuoUncGpRzNhEk6JlVccN2Leb1JzbldxSywyS2AApD05u-GFAgFDN3P39V3qgRTGDuuUfUvKQ9S4rbu5El9Qq1CJTeA".to_string()));

    let wallet = client.get_wallet(&wallet_id).await.unwrap();

    tracing::info!("got wallet: {:?}", wallet);

    let request = client.update_wallet(&wallet_id).owner(
        OwnerInput::PublicKeyOwner(PublicKeyOwner {
            public_key: public_key.to_string(),
        }),
        // OwnerInput::UserOwner(UserOwner {
        //     user_id: "did:privy:cmf5wqe2l0005k10blt7x5dq2".to_string(),
        // }),
    );

    // Build the canonical request data for signing using the serialized body
    let canonical_data = client.build_update_wallet_canonical_request(
        &wallet_id,
        &request.params,
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

    let wallet = match request.privy_authorization_signature(&signature).await {
        Ok(wallet) => Ok(wallet),
        Err(e) => Err(e),
    }?;

    tracing::info!("got new wallet: {:?}", wallet);

    Ok(())
}
