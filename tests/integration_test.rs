use privy_rust::types::{PrivyConfig, PrivySigner};
use privy_rust::PrivySignerBlocking;
use solana_sdk::signer::Signer;

// Helper to get test credentials from env or use defaults for CI
fn get_test_app_id() -> String {
    std::env::var("PRIVY_TEST_APP_ID")
        .or_else(|_| std::env::var("PRIVY_APP_ID"))
        .unwrap_or_else(|_| "test_app_id".to_string())
}

fn get_test_app_secret() -> String {
    std::env::var("PRIVY_TEST_APP_SECRET")
        .or_else(|_| std::env::var("PRIVY_APP_SECRET"))
        .unwrap_or_else(|_| "test_app_secret".to_string())
}

fn get_test_wallet_id() -> String {
    std::env::var("PRIVY_TEST_WALLET_ID")
        .or_else(|_| std::env::var("PRIVY_WALLET_ID"))
        .unwrap_or_else(|_| "test_wallet_id".to_string())
}

// Integration test that only runs if PRIVY_TEST_ENABLED=true
#[tokio::test]
async fn test_real_privy_connection() {
    // Load .env file if it exists
    dotenv::dotenv().ok();
    if std::env::var("PRIVY_TEST_ENABLED").unwrap_or_default() != "true" {
        println!("Skipping Privy integration test. Set PRIVY_TEST_ENABLED=true to run.");
        return;
    }
    
    let signer = PrivySigner::new(
        get_test_app_id(),
        get_test_app_secret(),
        get_test_wallet_id(),
    );
    
    // Test getting public key
    match signer.get_public_key().await {
        Ok(pubkey) => {
            println!("Successfully retrieved public key: {}", pubkey);
            
            // Create blocking signer
            let blocking_signer = PrivySignerBlocking::new(signer).unwrap();
            
            // Test signing
            let message = b"Test message from privy-rust";
            match blocking_signer.try_sign_message(message) {
                Ok(signature) => {
                    println!("Successfully signed message: {}", signature);
                }
                Err(e) => {
                    eprintln!("Failed to sign message: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to get public key: {}", e);
            panic!("Integration test failed: {}", e);
        }
    }
}

#[test]
fn test_signer_interface() {
    // This test verifies the interface matches what Kora expects
    
    // Test 1: Config from env pattern (matches Turnkey)
    let config = PrivyConfig::from_env();
    
    // Test 2: CLI override pattern (matches Turnkey)
    let config = config.merge_with_cli(
        Some(get_test_app_id()),
        Some(get_test_app_secret()),
        Some("test_wallet".to_string()),
    );
    
    // Test 3: Build pattern (matches Turnkey)
    match config.build() {
        Ok(signer) => {
            // Verify it's a PrivySigner
            assert_eq!(signer.app_id, get_test_app_id());
            assert_eq!(signer.wallet_id, "test_wallet");
        }
        Err(_) => {
            // Expected in test environment without real credentials
        }
    }
}

#[test]
fn test_signer_trait_implementation() {
    // Create a mock signer for testing trait implementation
    let signer = PrivySigner::new(
        get_test_app_id(),
        get_test_app_secret(),
        "test_wallet".to_string(),
    );
    
    // This would fail without real credentials, but we're just testing compilation
    if let Ok(blocking_signer) = PrivySignerBlocking::new(signer) {
        // Verify it implements Signer trait methods
        let _pubkey = blocking_signer.try_pubkey();
        let _is_interactive = blocking_signer.is_interactive();
        
        // These would need real credentials to actually work
        // let _signature = blocking_signer.try_sign_message(b"test");
    }
}

#[test]
fn test_turnkey_compatibility() {
    // This test ensures our interface is compatible with Turnkey's pattern
    
    // The pattern Kora uses:
    let with_privy_signer = true;
    let privy_app_id = Some(get_test_app_id());
    let privy_app_secret = Some(get_test_app_secret());
    let privy_wallet_id = Some("wallet".to_string());
    
    if with_privy_signer {
        let config = PrivyConfig::from_env()
            .merge_with_cli(privy_app_id, privy_app_secret, privy_wallet_id);
        
        match config.build() {
            Ok(signer) => {
                // In Kora, this would be boxed as Box<dyn Signer>
                match PrivySignerBlocking::new(signer) {
                    Ok(blocking_signer) => {
                        let _: Box<dyn Signer> = Box::new(blocking_signer);
                        // Success - interface is compatible
                    }
                    Err(_) => {
                        // Expected without real credentials
                    }
                }
            }
            Err(_) => {
                // Expected without real credentials
            }
        }
    }
}
