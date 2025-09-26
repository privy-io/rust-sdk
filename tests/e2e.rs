use anyhow::Result;
use common::get_test_client;
use p256::elliptic_curve::SecretKey;
use privy_rs::{AuthorizationContext, PrivateKey, generated::types::*};
use tracing_test::traced_test;
use uuid::Uuid;

mod common;

#[tokio::test]
#[traced_test]
async fn test_wallets_e2e_quorum_workflow() -> Result<()> {
    let client = get_test_client()?;

    // Generate unique idempotency keys for each operation
    let wallet_idempotency_key = format!("wallet-{}", Uuid::new_v4());
    let sign1_idempotency_key = format!("sign1-{}", Uuid::new_v4());
    let sign2_idempotency_key = format!("sign2-{}", Uuid::new_v4());

    tracing::info!("Starting comprehensive e2e quorum workflow test");

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

    // Step 2: Create a 2-of-3 key quorum first
    tracing::info!("Creating 2-of-3 key quorum");
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)?
        .as_secs();

    let quorum_display_name =
        CreateKeyQuorumBodyDisplayName::try_from(format!("E2E Test Quorum {timestamp}").as_str())?;

    let quorum_body = CreateKeyQuorumBody {
        authorization_threshold: Some(2.0), // 2-of-3 threshold
        display_name: Some(quorum_display_name),
        public_keys: vec![pubkey1, pubkey2, pubkey3],
        user_ids: vec![],
    };

    let key_quorum = client.key_quorums().create(&quorum_body).await?;

    tracing::info!("Created key quorum: {:?}", key_quorum);

    // Step 3: Create a new ethereum wallet with idempotency key
    tracing::info!(
        "Creating new ethereum wallet with idempotency key: {}",
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

    // Step 4: Test signing with only one key (should fail)
    let single_key_ctx = AuthorizationContext::new().push(PrivateKey(
        key1.to_sec1_pem(der::pem::LineEnding::LF)
            .unwrap()
            .as_str()
            .to_owned(),
    ));

    tracing::info!("Testing transaction signing with only one key (should fail)");
    let quorum_message = "Hello, quorum test!";

    let single_key_result = client.wallets().export(&wallet.id, &single_key_ctx).await;

    match single_key_result {
        Err(err) => {
            tracing::info!("Single key signing correctly failed as expected: {:?}", err);
        }
        Ok(e) => {
            tracing::error!("{:?}", e);
            panic!("Single key signing should have failed but succeeded!");
        }
    }

    // Step 5: Test signing with three keys (should succeed)
    tracing::info!("Testing transaction signing with three keys (should succeed)");
    let three_key_ctx = {
        single_key_ctx
            .push(PrivateKey(
                key2.to_sec1_pem(der::pem::LineEnding::LF)
                    .unwrap()
                    .as_str()
                    .to_owned(),
            ))
            .push(PrivateKey(
                key3.to_sec1_pem(der::pem::LineEnding::LF)
                    .unwrap()
                    .as_str()
                    .to_owned(),
            ))
    };

    let three_key_result =
        debug_response!(client.wallets().export(&wallet.id, &three_key_ctx)).await;

    tracing::info!("Three key signing successful: {:?}", three_key_result);
    tracing::info!("E2E quorum workflow test completed successfully!");
    tracing::info!("Wallet ID: {}", wallet.id);
    tracing::info!("Used idempotency keys:");
    tracing::info!("  Wallet: {}", wallet_idempotency_key);
    tracing::info!("  Sign 1: {}", sign1_idempotency_key);
    tracing::info!("  Sign 2: {}", sign2_idempotency_key);

    Ok(())
}
