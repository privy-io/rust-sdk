//! Integration tests for Alloy support
//!
//! These tests verify that Privy wallets work correctly with Alloy traits.
//! Note: These tests make real API calls. They will auto-skip if required
//! environment variables are not present.

#![cfg(feature = "alloy")]

use alloy_consensus::{SignableTransaction, TxLegacy};
use alloy_network::TxSignerSync;
use alloy_primitives::{Address, U256, address, b256};
use alloy_signer::{Signer, SignerSync};
use privy_rs::{AuthorizationContext, PrivateKey, PrivyClient};
use std::env;

fn get_test_env() -> Option<(String, String)> {
    if let (Ok(w), Ok(k)) = (env::var("TEST_WALLET_ID"), env::var("TEST_PRIVATE_KEY")) {
        return Some((w, k));
    }

    if let Ok(w) = env::var("PRIVY_WALLET_ID") {
        if let Ok(k) = std::fs::read_to_string("private_key.pem") {
            return Some((w, k));
        }
    }

    eprintln!(
        "[alloy_integration] skipping: set TEST_WALLET_ID and TEST_PRIVATE_KEY, or PRIVY_WALLET_ID and have private_key.pem present"
    );
    None
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sign_hash_sync() {
    let Some((wallet_id, private_key)) = get_test_env() else {
        return;
    };

    let client = PrivyClient::new_from_env().expect("Failed to create client");
    let ctx = AuthorizationContext::new().push(PrivateKey::new(private_key));

    let signer = client
        .wallets()
        .ethereum()
        .alloy(&wallet_id, &ctx)
        .await
        .expect("Failed to create signer");

    let hash = b256!("0000000000000000000000000000000000000000000000000000000000000001");

    let signature = signer.sign_hash_sync(&hash).expect("Failed to sign hash");

    assert_eq!(signature.as_bytes().len(), 65);

    let recovered = signature
        .recover_address_from_prehash(&hash)
        .expect("Failed to recover address");
    assert_eq!(recovered, TxSignerSync::address(&signer));
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sign_hash_async() {
    let Some((wallet_id, private_key)) = get_test_env() else {
        return;
    };

    let client = PrivyClient::new_from_env().expect("Failed to create client");
    let ctx = AuthorizationContext::new().push(PrivateKey::new(private_key));

    let signer = client
        .wallets()
        .ethereum()
        .alloy(&wallet_id, &ctx)
        .await
        .expect("Failed to create signer");

    let hash = b256!("0000000000000000000000000000000000000000000000000000000000000002");

    let signature = signer.sign_hash(&hash).await.expect("Failed to sign hash");

    assert_eq!(signature.as_bytes().len(), 65);

    let recovered = signature
        .recover_address_from_prehash(&hash)
        .expect("Failed to recover address");
    assert_eq!(recovered, Signer::address(&signer));
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sign_transaction() {
    let Some((wallet_id, private_key)) = get_test_env() else {
        return;
    };

    let client = PrivyClient::new_from_env().expect("Failed to create client");
    let ctx = AuthorizationContext::new().push(PrivateKey::new(private_key));

    let signer = client
        .wallets()
        .ethereum()
        .alloy(&wallet_id, &ctx)
        .await
        .expect("Failed to create signer");

    let mut tx = TxLegacy {
        chain_id: Some(1),
        nonce: 0,
        gas_price: 20_000_000_000u128,
        gas_limit: 21_000u64,
        to: alloy_primitives::TxKind::Call(address!("0000000000000000000000000000000000000000")),
        value: U256::from(1000000000000000000u128),
        input: Default::default(),
    };

    let signature = signer
        .sign_transaction_sync(&mut tx)
        .expect("Failed to sign transaction");

    assert_eq!(signature.as_bytes().len(), 65);

    let sig_hash = tx.signature_hash();
    let recovered = signature
        .recover_address_from_prehash(&sig_hash)
        .expect("Failed to recover address");
    assert_eq!(recovered, TxSignerSync::address(&signer));
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sign_message() {
    let Some((wallet_id, private_key)) = get_test_env() else {
        return;
    };

    let client = PrivyClient::new_from_env().expect("Failed to create client");
    let ctx = AuthorizationContext::new().push(PrivateKey::new(private_key));

    let signer = client
        .wallets()
        .ethereum()
        .alloy(&wallet_id, &ctx)
        .await
        .expect("Failed to create signer");

    let message = "Hello, Privy!";

    let signature = signer
        .sign_message(message.as_bytes())
        .await
        .expect("Failed to sign message");

    assert_eq!(signature.as_bytes().len(), 65);

    let recovered = signature
        .recover_address_from_msg(message.as_bytes())
        .expect("Failed to recover address");
    assert_eq!(recovered, Signer::address(&signer));
}

#[tokio::test(flavor = "multi_thread")]
async fn test_chain_id() {
    let Some((wallet_id, private_key)) = get_test_env() else {
        return;
    };

    let client = PrivyClient::new_from_env().expect("Failed to create client");
    let ctx = AuthorizationContext::new().push(PrivateKey::new(private_key));

    let signer = client
        .wallets()
        .ethereum()
        .alloy(&wallet_id, &ctx)
        .await
        .expect("Failed to create signer");

    assert_eq!(signer.chain_id(), None);
    assert_eq!(signer.chain_id_sync(), None);

    let signer_with_chain = signer.with_chain_id(1);
    assert_eq!(signer_with_chain.chain_id(), Some(1));
    assert_eq!(signer_with_chain.chain_id_sync(), Some(1));

    let mut mutable_signer = signer_with_chain;
    mutable_signer.set_chain_id(Some(137));
    assert_eq!(mutable_signer.chain_id(), Some(137));

    mutable_signer.set_chain_id(None);
    assert_eq!(mutable_signer.chain_id(), None);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_wallet_address() {
    let Some((wallet_id, private_key)) = get_test_env() else {
        return;
    };

    let client = PrivyClient::new_from_env().expect("Failed to create client");
    let ctx = AuthorizationContext::new().push(PrivateKey::new(private_key));

    let signer = client
        .wallets()
        .ethereum()
        .alloy(&wallet_id, &ctx)
        .await
        .expect("Failed to create signer");

    let addr_signer = Signer::address(&signer);
    let addr_tx_signer = TxSignerSync::address(&signer);

    assert_eq!(addr_signer, addr_tx_signer);

    assert_ne!(addr_signer, Address::ZERO);
}
