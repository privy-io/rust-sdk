//! Wallet RPC Example
//!
//! This example demonstrates how to sign transactions using Privy's wallet RPC interface.
//! It shows how to:
//! - Initialize a Privy client with app credentials
//! - Submit RPC requests for transaction signing
//! - Handle RPC responses with signed transaction data
//!
//! ## Required Environment Variables
//! - `PRIVY_APP_ID`: Your Privy app ID
//! - `PRIVY_APP_SECRET`: Your Privy app secret
//! - `PRIVY_WALLET_ID`: The wallet ID to use for signing
//!
//! ## Usage
//! ```bash
//! cargo run --example wallet_rpc
//! ```

use anyhow::Result;
use base64::{Engine, engine::general_purpose::STANDARD};
use privy_rust::{
    AuthorizationContext, JwtUser, PrivateKeyFromFile, PrivyClient,
    generated::types::{
        SolanaSignMessageRpcInput, SolanaSignMessageRpcInputMethod,
        SolanaSignMessageRpcInputParams, SolanaSignMessageRpcInputParamsEncoding, WalletRpcBody,
    },
};
use progenitor_client::Error;
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

    let ctx = AuthorizationContext::new();
    let client = PrivyClient::new(app_id.clone(), app_secret)?;

    ctx.push(PrivateKeyFromFile("private_key.pem".into()));
    ctx.push(JwtUser(client.clone(), "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJhbGV4QGFybHlvbi5kZXYiLCJpYXQiOjEwMDAwMDAwMDAwMH0.IpNgavH95CFZPjkzQW4eyxMIfJ-O_5cIaDyu_6KRXffykjYDRwxTgFJuYq0F6d8wSXf4de-vzfBRWSKMISM3rJdlhximYINGJB14mJFCD87VMLFbTpHIXcv7hc1AAYMPGhOsRkYfYXuvVopKszMvhupmQYJ1npSvKWNeBniIyOHYv4xebZD8L0RVlPvuEKTXTu-CDfs2rMwvD9g_wiBznS3uMF3v_KPaY6x0sx9zeCSxAH9zvhMMtct_Ad9kuoUncGpRzNhEk6JlVccN2Leb1JzbldxSywyS2AApD05u-GFAgFDN3P39V3qgRTGDuuUfUvKQ9S4rbu5El9Qq1CJTeA".to_string()));

    // Example: Sign a Solana transaction
    let rpc_response = match client
        .wallets()
        .rpc(
            &wallet_id,
            &ctx,
            None, // No idempotency key
            &WalletRpcBody::SolanaSignMessageRpcInput(SolanaSignMessageRpcInput {
                address: Some("7EcDhSYGxXyscszYEp35KHN8vvw3svAuLKTzXwCFLtV".to_string()),
                chain_type: None,
                method: SolanaSignMessageRpcInputMethod::SignMessage,
                params: SolanaSignMessageRpcInputParams {
                    encoding: SolanaSignMessageRpcInputParamsEncoding::Base64,
                    message: STANDARD.encode("hello world"),
                },
            }),
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

    tracing::info!("RPC response: {:?}", rpc_response);

    Ok(())
}
