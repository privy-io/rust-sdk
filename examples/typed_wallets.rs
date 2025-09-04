//! typed wallet example
//!
//! This example shows how to use the type-safe wallet interface to work with
//! both Solana and Ethereum wallets, ensuring compile-time safety for
//! blockchain-specific operations.

use std::env;

use anyhow::{Context, Result};
use privy_rust::{
    PrivyClient,
    wallet::{Ethereum, Solana},
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for better logging
    tracing_subscriber::fmt::init();

    // Load environment variables
    let app_id =
        env::var("PRIVY_APP_ID").context("PRIVY_APP_ID environment variable is required")?;
    let app_secret = env::var("PRIVY_APP_SECRET")
        .context("PRIVY_APP_SECRET environment variable is required")?;
    let wallet_id =
        env::var("PRIVY_WALLET_ID").context("PRIVY_WALLET_ID environment variable is required")?;

    // Create the PrivyClient
    let client = PrivyClient::new(app_id, app_secret)?;

    println!("🎯 Typed Wallet API Demo");
    println!("========================\n");

    // Demonstrate Solana wallet operations
    demonstrate_solana_wallet(&client, &wallet_id).await?;

    // Demonstrate Ethereum wallet operations
    demonstrate_ethereum_wallet(&client, &wallet_id).await?;

    println!("✅ Demo completed successfully!");
    Ok(())
}

async fn demonstrate_solana_wallet(client: &PrivyClient, wallet_id: &str) -> Result<()> {
    println!("🔵 Solana Wallet Operations");
    println!("-----------------------------");

    // Create a typed Solana wallet
    let solana_wallet = client.wallet::<Solana>(wallet_id);

    // Get the native Solana public key
    match solana_wallet.pubkey().await {
        Ok(pubkey) => {
            println!("✓ Solana Public Key: {}", pubkey);
        }
        Err(e) => {
            println!("⚠ Failed to get Solana public key: {}", e);
        }
    }

    // Access wallet metadata
    println!("  Wallet ID: {}", solana_wallet.id());
    println!("  Public Key String: {:?}", solana_wallet.pubkey().await);

    // Demonstrate message signing
    let test_message = b"Hello from typed Solana wallet!";
    println!(
        "  Attempting to sign message: {:?}",
        std::str::from_utf8(test_message)
    );

    match solana_wallet.sign_message(test_message).await {
        Ok(signature) => {
            println!("✓ Message signed successfully!");
            println!("  Signature: {}", signature);
        }
        Err(e) => {
            println!("⚠ Message signing failed: {}", e);
        }
    }

    // Note: Transaction signing would work similarly
    // let transaction = Transaction::default();
    // let signature = solana_wallet.sign_transaction(&transaction).await?;

    println!();
    Ok(())
}

async fn demonstrate_ethereum_wallet(client: &PrivyClient, wallet_id: &str) -> Result<()> {
    println!("🟠 Ethereum Wallet Operations");
    println!("------------------------------");

    // Create a typed Ethereum wallet
    let ethereum_wallet = client.wallet::<Ethereum>(wallet_id);

    // Get the native Ethereum address
    match ethereum_wallet.pubkey().await {
        Ok(address) => {
            println!("✓ Ethereum Address: {:?}", address);
        }
        Err(e) => {
            println!("⚠ Failed to parse Ethereum address: {}", e);
        }
    }

    // Access wallet metadata
    println!("  Wallet ID: {}", ethereum_wallet.id());
    println!("  Address String: {:?}", ethereum_wallet.pubkey().await);

    // Demonstrate message signing (currently returns error as not implemented)
    let test_message = b"Hello from typed Ethereum wallet!";
    println!(
        "  Attempting to sign message: {:?}",
        std::str::from_utf8(test_message)
    );

    match ethereum_wallet.sign_message(test_message).await {
        Ok(signature) => {
            println!("✓ Message signed successfully!");
            println!("  Signature: {:?}", signature);
        }
        Err(e) => {
            println!("⚠ Message signing not yet implemented: {}", e);
        }
    }

    println!();
    Ok(())
}

/// This function demonstrates compile-time safety of the typed API.
///
/// The commented code below would NOT compile, ensuring you can only
/// call blockchain-specific methods on wallets of the correct type.
#[allow(dead_code)]
fn demonstrate_compile_time_safety() {
    // This code is intentionally commented out to show what would NOT compile:

    let client = PrivyClient::new("a".to_string(), "b".to_string()).unwrap();
    let solana_wallet = client.wallet::<Solana>("wallet_id");
    let ethereum_wallet = client.wallet::<Ethereum>("0x...");

    // ✅ This compiles - calling Solana method on Solana wallet
    let _solana_pubkey = solana_wallet.pubkey();

    // ✅ This compiles - calling Ethereum method on Ethereum wallet
    let _eth_address = ethereum_wallet.pubkey();

    // ❌ This would NOT compile - mismatched signature types
    // let mut sig_a = solana_wallet.sign_message(b"test").await;
    // let sig_b = ethereum_wallet.sign_message(b"test").await;
    //
    // sig_a = sig_b;
}
