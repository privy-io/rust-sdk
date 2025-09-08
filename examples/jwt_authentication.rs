//! custom authorization example
//!
//! Enterprise customers may use external authorization providers to
//! authorize users. To do so, they register a subject on a user record
//! on Privy, then, a valid JWT for that subject can be used in exchange
//! for a short lived authorization key.

use anyhow::Result;
use privy_rust::{PrivyClient, generated::types::AuthenticateBody};
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

    tracing::info!(
        "initializing privy with app_id: {}, app_secret: {}",
        app_id,
        app_secret,
    );

    let client = PrivyClient::new(app_id, app_secret, Default::default())?;

    let jwt_auth = client
        .authenticate(&AuthenticateBody{
            user_jwt: "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJhbGV4QGFybHlvbi5kZXYiLCJpYXQiOjEwMDAwMDAwMDAwMH0.IpNgavH95CFZPjkzQW4eyxMIfJ-O_5cIaDyu_6KRXffykjYDRwxTgFJuYq0F6d8wSXf4de-vzfBRWSKMISM3rJdlhximYINGJB14mJFCD87VMLFbTpHIXcv7hc1AAYMPGhOsRkYfYXuvVopKszMvhupmQYJ1npSvKWNeBniIyOHYv4xebZD8L0RVlPvuEKTXTu-CDfs2rMwvD9g_wiBznS3uMF3v_KPaY6x0sx9zeCSxAH9zvhMMtct_Ad9kuoUncGpRzNhEk6JlVccN2Leb1JzbldxSywyS2AApD05u-GFAgFDN3P39V3qgRTGDuuUfUvKQ9S4rbu5El9Qq1CJTeA".into(),
            encryption_type: None,
            recipient_public_key: None,
        })
        .await?;

    tracing::info!("got jwt auth: {:?}", jwt_auth);

    Ok(())
}
