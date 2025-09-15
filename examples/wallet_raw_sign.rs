//! Wallet Raw Sign Example
//!
//! This example demonstrates how to sign raw data using Privy's wallet raw signing.
//! It shows how to:
//! - Initialize a Privy client with app credentials
//! - Sign arbitrary data with a wallet
//! - Handle raw signature responses
//!
//! ## Required Environment Variables
//! - `PRIVY_APP_ID`: Your Privy app ID
//! - `PRIVY_APP_SECRET`: Your Privy app secret
//! - `PRIVY_WALLET_ID`: The wallet ID to use for signing
//!
//! ## Usage
//! ```bash
//! cargo run --example wallet_raw_sign
//! ```

use anyhow::Result;
use privy_rust::{
    AuthorizationContext, JwtUser, PrivateKeyFromFile, PrivyClient,
    generated::{
        Error,
        types::{RawSign, RawSignParams},
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
    let app_id = std::env::var("PRIVY_APP_ID").expect("PRIVY_APP_ID environment variable not set");
    let app_secret =
        std::env::var("PRIVY_APP_SECRET").expect("PRIVY_APP_SECRET environment variable not set");
    let wallet_id =
        std::env::var("PRIVY_WALLET_ID").expect("PRIVY_WALLET_ID environment variable not set");

    tracing::info!(
        "initializing privy with app_id: {}, app_secret: {}, wallet_id: {}",
        app_id,
        app_secret,
        wallet_id
    );
    let client = PrivyClient::new(app_id.clone(), app_secret)?;

    let ctx = AuthorizationContext::new();
    ctx.push(PrivateKeyFromFile("private_key.pem".into()));
    ctx.push(JwtUser(client.clone(), "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJhbGV4QGFybHlvbi5kZXYiLCJpYXQiOjEwMDAwMDAwMDAwMH0.IpNgavH95CFZPjkzQW4eyxMIfJ-O_5cIaDyu_6KRXffykjYDRwxTgFJuYq0F6d8wSXf4de-vzfBRWSKMISM3rJdlhximYINGJB14mJFCD87VMLFbTpHIXcv7hc1AAYMPGhOsRkYfYXuvVopKszMvhupmQYJ1npSvKWNeBniIyOHYv4xebZD8L0RVlPvuEKTXTu-CDfs2rMwvD9g_wiBznS3uMF3v_KPaY6x0sx9zeCSxAH9zvhMMtct_Ad9kuoUncGpRzNhEk6JlVccN2Leb1JzbldxSywyS2AApD05u-GFAgFDN3P39V3qgRTGDuuUfUvKQ9S4rbu5El9Qq1CJTeA".to_string()));

    // Example: Sign raw message data
    let raw_sign_response = match client
        .wallets()
        .raw_sign(
            &wallet_id,
            &ctx,
            None, // No idempotency key
            &RawSign {
                params: RawSignParams {
                    hash: Some("0xdeadbeef".to_string()),
                },
            },
        )
        .await
    {
        Ok(r) => r,
        Err(Error::UnexpectedResponse(resp)) => {
            tracing::error!("Unexpected response: {:?}", resp.text().await);
            return Err(anyhow::anyhow!("Unexpected response"));
        }
        Err(e) => return Err(e.into()),
    };

    tracing::info!("Raw sign response: {:?}", raw_sign_response);

    Ok(())
}
