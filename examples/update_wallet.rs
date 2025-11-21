//! Update Wallet Example
//!
//! This example demonstrates how to update a wallet's owner using signature authorization.
//! It shows how to:
//! - Initialize a Privy client with app credentials
//! - Load private keys for signature authorization
//! - Update a wallet's owner with proper authorization
//! - Handle signature authorization errors and responses
//!
//! ## Required Environment Variables
//! - `PRIVY_APP_ID`: Your Privy app ID
//! - `PRIVY_APP_SECRET`: Your Privy app secret
//! - `PRIVY_WALLET_ID`: The wallet ID to update
//!
//! ## Usage
//! ```bash
//! cargo run --example update_wallet
//! ```

use anyhow::Result;
use privy_rs::{
    AuthorizationContext, IntoKey, JwtUser, PrivateKey, PrivyApiError, PrivyClient,
    PrivySignedApiError,
    generated::types::{OwnerInput, UpdateWalletBody},
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

    let file = std::fs::read_to_string("private_key.pem")?;

    let key = PrivateKey::new(file);
    let public_key = key.get_key().await?.public_key();

    let client = PrivyClient::new_from_env()?;

    let ctx = AuthorizationContext::new()
        .push(key)
        .push(JwtUser(client.clone(), "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJhbGV4QGFybHlvbi5kZXYiLCJpYXQiOjEwMDAwMDAwMDAwMH0.IpNgavH95CFZPjkzQW4eyxMIfJ-O_5cIaDyu_6KRXffykjYDRwxTgFJuYq0F6d8wSXf4de-vzfBRWSKMISM3rJdlhximYINGJB14mJFCD87VMLFbTpHIXcv7hc1AAYMPGhOsRkYfYXuvVopKszMvhupmQYJ1npSvKWNeBniIyOHYv4xebZD8L0RVlPvuEKTXTu-CDfs2rMwvD9g_wiBznS3uMF3v_KPaY6x0sx9zeCSxAH9zvhMMtct_Ad9kuoUncGpRzNhEk6JlVccN2Leb1JzbldxSywyS2AApD05u-GFAgFDN3P39V3qgRTGDuuUfUvKQ9S4rbu5El9Qq1CJTeA".to_string()));

    let wallets_client = client.wallets();
    let wallet = wallets_client.get(&wallet_id).await?;

    tracing::info!("got wallet: {:?}", wallet);

    let wallet = match wallets_client
        .update(
            &wallet_id,
            &ctx,
            &UpdateWalletBody {
                owner: Some(OwnerInput::PublicKey(public_key.to_string())),
                ..Default::default()
            },
        )
        .await
    {
        Ok(wallet) => wallet,
        Err(PrivySignedApiError::Api(PrivyApiError::UnexpectedResponse(r))) => {
            let text = r.text().await.unwrap_or_default();
            tracing::warn!("unexpected response: {:?}", text);
            anyhow::bail!("unexpected response")
        }
        Err(e) => {
            anyhow::bail!("unexpected error: {e:?}")
        }
    };

    tracing::info!("got updated wallet: {:?}", wallet);

    Ok(())
}
