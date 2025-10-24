# privy-rs

[![privy-rs on crates.io](https://img.shields.io/crates/v/privy-rs.svg)](https://crates.io/crates/privy-rs)
[![privy-rs on docs.rs](https://docs.rs/privy-rs/badge.svg)](https://docs.rs/privy-rs)

A Rust library for interacting with the Privy API, providing secure embedded wallet signing capabilities for Solana transactions. This library enables seamless integration with Privy's wallet infrastructure for transaction signing operations.

[Get started with Privy Today!](https://docs.privy.io/basics/get-started/about)

## Installation

For up to date SDK documentation and examples, see [docs.rs](https://docs.rs/privy-rs/latest/privy_rs/client/struct.PrivyClient.html)
or the [examples](https://github.com/privy-io/privy-rs/tree/main/examples) folder in the repo.
For general documentation about the Privy API, see the [API docs](https://docs.privy.io).
Otherwise, some basic examples can be found below.

Add this to your `Cargo.toml`:

```toml
[dependencies]
privy-rs = "0.1.0-alpha"
```

## Usage

First, set up your Privy credentials:

```rust
use privy_rs::PrivyClient;

let client = PrivyClient::new(
    env::var("PRIVY_APP_ID").unwrap(),
    env::var("PRIVY_APP_SECRET").unwrap(),
)?;
```

Then, you can access all the sub-clients using the methods available on the client.

### Creating a wallet

```rust
use privy_rs::generated::types::*;

let wallet = client
    .wallets()
    .create(
        None,
        &CreateWalletBody {
            chain_type: WalletChainType::Ethereum,
            additional_signers: None,
            owner: None,
            owner_id: None,
            policy_ids: vec![],
        },
    )
    .await?;
```

### Signing a message on Ethereum

```rust
use privy_rs::{AuthorizationContext};

let ethereum_service = client.wallets().ethereum();
let auth_ctx = AuthorizationContext::new();

// Sign a UTF-8 message
let result = ethereum_service
    .sign_message(
        &wallet.id,
        "Hello, Ethereum!",
        &auth_ctx,
        None, // no idempotency key
    )
    .await?;
```

### Signing a message on Solana

```rust
use privy_rs::{AuthorizationContext};

let solana_service = client.wallets().solana();
let auth_ctx = AuthorizationContext::new();

// Base64 encode your message first
let message = base64::encode("Hello, Solana!");
let signature = solana_service
    .sign_message(
        &wallet.id, // make sure to use a solana wallet
        &message,
        &auth_ctx,
        Some("unique-request-id-456"),
    )
    .await?;
```

### Alloy Integration

Privy wallets can be used with the Alloy ecosystem by enabling the `alloy` feature:

```toml
[dependencies]
privy-rs = { version = "0.1.0-alpha", features = ["alloy"] }
```

Then use Privy wallets as Alloy signers:

```rust
use privy_rs::{PrivyClient, AuthorizationContext, PrivateKey};
use alloy_signer::SignerSync;
use alloy_consensus::TxLegacy;
use alloy_network::TxSignerSync;
use alloy_primitives::{address, bytes, U256};

let client = PrivyClient::new_from_env()?;
let private_key = std::fs::read_to_string("private_key.pem")?;
let ctx = AuthorizationContext::new().push(PrivateKey(private_key));

// Create Alloy signer
let signer = client.wallets().ethereum().signer("wallet_id", &ctx).await?;

// Sign transactions with Alloy
let mut tx = TxLegacy {
    to: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into(),
    value: U256::from(1_000_000_000),
    gas_limit: 21_000,
    nonce: 0,
    gas_price: 20_000_000_000_u128,
    input: bytes!(),
    chain_id: Some(1),
};

let signature = signer.sign_transaction_sync(&mut tx)?;
```

See the [alloy_integration example](examples/alloy_integration.rs) for more details.

## License

This project is dual-licensed under MIT and Apache-2.0.
