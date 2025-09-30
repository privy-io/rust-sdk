//! Quorum Wallet Example
//!
//! This example demonstrates the complete quorum workflow:
//! - Generate P-256 key pairs for quorum members
//! - Create a 2-of-3 key quorum
//! - Create a wallet owned by the quorum
//! - Test signing with insufficient keys (should fail)
//! - Test signing with sufficient keys (should succeed)
//!
//! This shows how to use Privy's key-based quorum system for multi-signature
//! wallet operations, where multiple cryptographic keys are required to authorize
//! wallet operations like signing transactions or exporting private keys.
//!
//! ## Required Environment Variables
//! - `PRIVY_APP_ID`: Your Privy app ID
//! - `PRIVY_APP_SECRET`: Your Privy app secret
//!
//! ## Usage
//! ```bash
//! cargo run --example quorum_wallet
//! ```

use anyhow::Result;
use p256::elliptic_curve::SecretKey;
use privy_rs::{
    AuthorizationContext, PrivateKey, PrivyClient,
    generated::types::{
        CreateKeyQuorumBody, CreateKeyQuorumBodyDisplayName, CreateWalletBody, WalletChainType,
    },
};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Initialize client from environment variables
    let client = PrivyClient::new_from_env()?;

    tracing::info!("initialized privy client from environment");

    // Generate unique identifiers for this example run
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)?
        .as_secs();
    let wallet_idempotency_key = format!("quorum-wallet-{}", Uuid::new_v4());

    tracing::info!("Starting quorum wallet example");

    // Step 1: Generate three P-256 private keys for quorum
    tracing::info!("Generating three P-256 private keys for 2-of-3 quorum");
    let mut rng = rand::thread_rng();
    let key1 = SecretKey::<p256::NistP256>::random(&mut rng);
    let key2 = SecretKey::<p256::NistP256>::random(&mut rng);
    let key3 = SecretKey::<p256::NistP256>::random(&mut rng);

    let pubkey1 = key1.public_key().to_string();
    let pubkey2 = key2.public_key().to_string();
    let pubkey3 = key3.public_key().to_string();

    tracing::info!("Generated public keys:");
    tracing::info!("Key 1: {}", pubkey1);
    tracing::info!("Key 2: {}", pubkey2);
    tracing::info!("Key 3: {}", pubkey3);

    // Step 2: Create a 2-of-3 key quorum
    tracing::info!("Creating 2-of-3 key quorum");

    let quorum_display_name =
        CreateKeyQuorumBodyDisplayName::try_from(format!("Quorum Example {timestamp}").as_str())?;

    let quorum_body = CreateKeyQuorumBody {
        authorization_threshold: Some(2.0), // 2-of-3 threshold
        display_name: Some(quorum_display_name),
        public_keys: vec![pubkey1, pubkey2, pubkey3],
        user_ids: vec![],
    };

    let key_quorum = client.key_quorums().create(&quorum_body).await?;

    tracing::info!("Created key quorum with ID: {}", key_quorum.id);
    tracing::info!("Quorum threshold: {:?}", key_quorum.authorization_threshold);

    // Step 3: Create a new Ethereum wallet owned by the quorum
    tracing::info!(
        "Creating new Ethereum wallet owned by quorum with idempotency key: {}",
        wallet_idempotency_key
    );

    let create_body = CreateWalletBody {
        chain_type: WalletChainType::Ethereum,
        additional_signers: None,
        owner: None,
        owner_id: Some(key_quorum.id.parse().unwrap()),
        policy_ids: vec![],
    };

    let wallet = client
        .wallets()
        .create(Some(&wallet_idempotency_key), &create_body)
        .await?;

    tracing::info!("Created wallet with ID: {}", wallet.id);
    tracing::info!("Wallet address: {}", wallet.address);
    tracing::info!("Wallet owner ID: {:?}", wallet.owner_id);

    // Step 4: Test signing with only one key (should fail)
    tracing::info!("Testing wallet export with only one key (should fail due to quorum threshold)");

    let single_key_ctx = AuthorizationContext::new().push(PrivateKey(
        key1.to_sec1_pem(der::pem::LineEnding::LF)
            .unwrap()
            .as_str()
            .to_owned(),
    ));

    let single_key_result = client.wallets().export(&wallet.id, &single_key_ctx).await;

    match single_key_result {
        Err(err) => {
            tracing::info!(
                "✓ Single key authorization correctly failed as expected: {:?}",
                err
            );
        }
        Ok(_) => {
            tracing::error!("✗ Single key authorization should have failed but succeeded!");
            return Err(anyhow::anyhow!(
                "Single key authorization should have failed due to quorum threshold"
            ));
        }
    }

    // Step 5: Test signing with two keys (should succeed)
    tracing::info!("Testing wallet export with two keys (should succeed)");

    let two_key_ctx = single_key_ctx.push(PrivateKey(
        key2.to_sec1_pem(der::pem::LineEnding::LF)
            .unwrap()
            .as_str()
            .to_owned(),
    ));

    let two_key_result = client.wallets().export(&wallet.id, &two_key_ctx).await;

    match two_key_result {
        Ok(export_result) => {
            tracing::info!("✓ Two key authorization succeeded as expected");
            tracing::info!("Exported private key length: {} bytes", export_result.len());
        }
        Err(err) => {
            tracing::error!("✗ Two key authorization failed unexpectedly: {:?}", err);
            return Err(anyhow::anyhow!(
                "Two key authorization should have succeeded"
            ));
        }
    }

    Ok(())
}
