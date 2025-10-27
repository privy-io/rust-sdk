//! Alloy Integration Example
//!
//! This example demonstrates using a Privy wallet with the Alloy ecosystem.
//!
//! ## Required Environment Variables
//! - `PRIVY_APP_ID`: Your Privy app ID
//! - `PRIVY_APP_SECRET`: Your Privy app secret
//! - `PRIVY_WALLET_ID`: The wallet ID to use
//!
//! ## Usage
//! ```bash
//! cargo run --example alloy_integration --features alloy
//! ```

use anyhow::Result;
use privy_rs::{AuthorizationContext, PrivateKey, PrivyClient};

use alloy_consensus::TxLegacy;
use alloy_network::TxSignerSync;
use alloy_primitives::{U256, address, bytes};
use alloy_signer::SignerSync;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let wallet_id = std::env::var("PRIVY_WALLET_ID")?;
    let private_key = std::fs::read_to_string("private_key.pem")?;

    let client = PrivyClient::new_from_env()?;
    let ctx = AuthorizationContext::new().push(PrivateKey(private_key));

    println!("Creating Alloy signer");
    let signer = client.wallets().ethereum().signer(&wallet_id, &ctx).await?;

    println!("Address: {:?}", TxSignerSync::address(&signer));
    println!("Chain ID: {:?}\n", signer.chain_id_sync());

    println!("Sign a transaction (sync)");
    let mut tx = TxLegacy {
        to: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into(),
        value: U256::from(1_000_000_000),
        gas_limit: 2_000_000,
        nonce: 0,
        gas_price: 21_000_000_000,
        input: bytes!(),
        chain_id: Some(1),
    };

    println!("Signing transaction...");
    let signature = signer.sign_transaction_sync(&mut tx)?;
    println!("Signature: {:?}\n", signature);

    Ok(())
}
