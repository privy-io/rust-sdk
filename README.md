# privy-rust

A Rust library for interacting with the Privy API, providing secure embedded wallet signing capabilities for Solana transactions. This library enables seamless integration with Privy's wallet infrastructure for transaction signing operations.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
privy-rust = "0.1.0"
```

## Usage

First, set up your Privy credentials:

```rust
use privy_rust::PrivySigner;

let signer = PrivySigner::new(
    env::var("PRIVY_APP_ID").unwrap(),
    env::var("PRIVY_APP_SECRET").unwrap(),
    env::var("PRIVY_WALLET_ID").unwrap(),
    String::new(), // Reserved for future use
    env::var("PRIVY_PUBLIC_KEY").unwrap(),
)?;
```

### General Purpose Signing

```rust
let message = b"Transaction data...";
let signature = signer.sign(message).await?;
```

### Solana Transaction Signing

```rust
let transaction_bytes = b"Solana transaction data...";
let signature = signer.sign_solana(transaction_bytes).await?;

// Get the associated Solana public key
let pubkey = signer.solana_pubkey();
```

## API Reference

### `PrivySigner`

The main struct for interacting with the Privy API to sign transactions using embedded wallets.

#### Methods

- `new(app_id: String, app_secret: String, wallet_id: String, _unused: String, public_key: String) -> Result<Self, anyhow::Error>`
  - Creates a new PrivySigner instance with the provided Privy credentials.

- `async sign(&self, message: &[u8]) -> Result<Vec<u8>, anyhow::Error>`
  - Signs a message using Privy's RPC endpoint and returns the signature as a byte vector.

- `async sign_solana(&self, message: &[u8]) -> Result<solana_sdk::signature::Signature, anyhow::Error>`
  - Signs a Solana transaction and returns a Solana-compatible signature.

- `solana_pubkey(&self) -> solana_sdk::pubkey::Pubkey`
  - Returns the Solana public key associated with the Privy wallet.

## Environment Variables

Create a `.env` file with your Privy credentials:

```env
PRIVY_APP_ID=your_app_id
PRIVY_APP_SECRET=your_app_secret
PRIVY_WALLET_ID=your_wallet_id
PRIVY_PUBLIC_KEY=your_solana_public_key
```

## License

This project is licensed under the MIT License.