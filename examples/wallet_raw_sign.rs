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
use privy_rs::{
    AuthorizationContext, JwtUser, PrivateKey, PrivyApiError, PrivyClient, PrivySignedApiError,
    generated::types::{RawSign, RawSignParams},
};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Get wallet ID from environment
    let wallet_id =
        std::env::var("PRIVY_WALLET_ID").expect("PRIVY_WALLET_ID environment variable not set");

    tracing::info!(
        "initializing privy client from environment, wallet_id: {}",
        wallet_id
    );

    let private_key = std::fs::read_to_string("private_key.pem")?;

    let client = PrivyClient::new_from_env()?;

    let ctx = AuthorizationContext::new()
        .push(PrivateKey(private_key))
        .push(JwtUser(client.clone(), "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJhbGV4QGFybHlvbi5kZXYiLCJpYXQiOjEwMDAwMDAwMDAwMH0.IpNgavH95CFZPjkzQW4eyxMIfJ-O_5cIaDyu_6KRXffykjYDRwxTgFJuYq0F6d8wSXf4de-vzfBRWSKMISM3rJdlhximYINGJB14mJFCD87VMLFbTpHIXcv7hc1AAYMPGhOsRkYfYXuvVopKszMvhupmQYJ1npSvKWNeBniIyOHYv4xebZD8L0RVlPvuEKTXTu-CDfs2rMwvD9g_wiBznS3uMF3v_KPaY6x0sx9zeCSxAH9zvhMMtct_Ad9kuoUncGpRzNhEk6JlVccN2Leb1JzbldxSywyS2AApD05u-GFAgFDN3P39V3qgRTGDuuUfUvKQ9S4rbu5El9Qq1CJTeA".to_string()));

    // Example: Sign raw message data
    let raw_sign_response = match client
        .wallets()
        .raw_sign(
            &wallet_id,
            &ctx,
            None, // No idempotency key
            &RawSign {
                params: RawSignParams::Variant0 {
                    hash: "0xdeadbeef".to_string(),
                },
            },
        )
        .await
    {
        Ok(r) => r,
        Err(PrivySignedApiError::Api(PrivyApiError::UnexpectedResponse(resp))) => {
            tracing::error!("Unexpected response: {:?}", resp.text().await);
            return Err(anyhow::anyhow!("Unexpected response"));
        }
        Err(e) => return Err(e.into()),
    };

    tracing::info!("Raw sign response: {:?}", raw_sign_response);

    Ok(())
}
