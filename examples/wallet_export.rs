//! Wallet Export Example
//!
//! This example demonstrates how to export a wallet's private key.
//! ⚠️ **SECURITY WARNING**: This operation exposes sensitive private key material!
//!
//! It shows how to:
//! - Initialize a Privy client with app credentials
//! - Export wallet private keys with proper authorization
//! - Handle export responses containing sensitive key data
//!
//! ## Required Environment Variables
//! - `PRIVY_APP_ID`: Your Privy app ID
//! - `PRIVY_APP_SECRET`: Your Privy app secret
//! - `PRIVY_WALLET_ID`: The wallet ID to export
//! - `PRIVY_AUTH_SIGNATURE`: Signature authorization (required)
//!
//! ## Usage
//! ```bash
//! cargo run --example wallet_export
//! ```

use anyhow::Result;
use hex::ToHex;
use privy_rust::{
    AuthorizationContext, JwtUser, PrivateKey, PrivyClient,
    generated::types::{HpkeEncryption, WalletExportRequestBody},
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

    // Generate HPKE key pair for encryption
    let hpke_keypair = privy_rust::privy_hpke::PrivyHpke::new();
    let recipient_public_key = hpke_keypair.public_key()?;

    tracing::info!("Generated HPKE key pair for encryption");

    let private_key = std::fs::read_to_string("private_key.pem")?;

    let client = PrivyClient::new(app_id.clone(), app_secret)?;

    let ctx = AuthorizationContext::new();
    ctx.push(PrivateKey(private_key));
    ctx.push(JwtUser(client.clone(), "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJhbGV4QGFybHlvbi5kZXYiLCJpYXQiOjEwMDAwMDAwMDAwMH0.IpNgavH95CFZPjkzQW4eyxMIfJ-O_5cIaDyu_6KRXffykjYDRwxTgFJuYq0F6d8wSXf4de-vzfBRWSKMISM3rJdlhximYINGJB14mJFCD87VMLFbTpHIXcv7hc1AAYMPGhOsRkYfYXuvVopKszMvhupmQYJ1npSvKWNeBniIyOHYv4xebZD8L0RVlPvuEKTXTu-CDfs2rMwvD9g_wiBznS3uMF3v_KPaY6x0sx9zeCSxAH9zvhMMtct_Ad9kuoUncGpRzNhEk6JlVccN2Leb1JzbldxSywyS2AApD05u-GFAgFDN3P39V3qgRTGDuuUfUvKQ9S4rbu5El9Qq1CJTeA".to_string()));

    // Export wallet private key (requires authorization signature)
    let export_response = client
        .wallets()
        .export(
            &wallet_id,
            &ctx,
            &WalletExportRequestBody {
                encryption_type: HpkeEncryption::Hpke,
                recipient_public_key,
            },
        )
        .await?;

    tracing::info!("Received encrypted wallet export response");

    // Decrypt the exported private key
    let decrypted_key = hpke_keypair.decrypt(
        &export_response.encapsulated_key,
        &export_response.ciphertext,
    )?;

    tracing::info!("Successfully decrypted private key");
    tracing::warn!("SECURITY WARNING: Private key exported and decrypted!");
    println!(
        "Decrypted private key (hex): {}",
        decrypted_key.to_bytes().encode_hex::<String>()
    );

    Ok(())
}
