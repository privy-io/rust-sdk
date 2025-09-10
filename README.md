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
use privy_rust::PrivyClient;

let client = PrivyClient::new(
    env::var("PRIVY_APP_ID").unwrap(),
    env::var("PRIVY_APP_SECRET").unwrap(),
)?;
```

### General Purpose Signing

```rust
let message = b"Transaction data...";
let signature = client.wallet<Ethereum>("wallet_id").sign_message(message).await?;
```

### Solana Transaction Signing

```rust
let transaction_bytes = b"Solana transaction data...";
let wallet = client.wallet<Solana>("wallet_id");
let signature = wallet.sign_message(transaction_bytes).await?;

// Get the associated Solana public key
let pubkey = wallet.pubkey().await?;
```

## API Reference

### `PrivyClient`

The main struct for interacting with the Privy API to sign transactions using embedded wallets.

#### Methods

- `new(app_id: String, app_secret: String, wallet_id: String, _unused: String, public_key: String) -> Result<Self, anyhow::Error>`
  - Creates a new PrivyClient instance with the provided Privy credentials.

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
```

## Testing

This project includes comprehensive test coverage across multiple levels:

### Test Categories

- **Unit Tests**: Located in `src/` files using `#[cfg(test)]` modules
- **Integration Tests**: End-to-end tests in `tests/` directory
- **Example Tests**: Runnable examples in `examples/` directory

### Environment Setup for Testing

#### Staging Environment Variables

For end-to-end tests, configure these environment variables:

```bash
# Required for all E2E tests
export STAGING_APP_ID="your_staging_app_id"
export STAGING_APP_SECRET="your_staging_app_secret"
export STAGING_URL="https://api.privy.io"  # Optional, defaults to production

# Optional for specific test types
export STAGING_JWT="your_test_jwt_token"           # JWT authentication tests
export STAGING_WALLET_ID="your_test_wallet_id"    # Wallet operation tests
export STAGING_PRIVATE_KEY_PATH="./private_key.pem"  # Signature auth tests
```

#### Key Generation for Signature Authorization

Generate P-256 key pairs for signature authorization tests:

```bash
./scripts/generate-keys.sh
```

This creates:
- `private_key.pem` (SEC1 format for Rust)
- `public_key.pem` (standard PEM format)

### Running Tests

#### Unit Tests

```bash
# Run all unit tests
cargo test --lib

# Run specific module tests
cargo test --lib keys::tests
cargo test --lib client::tests
cargo test --lib types::tests
```

#### End-to-End Tests

```bash
# Run all E2E tests (requires staging environment)
cargo test --test "*"

# Run specific E2E test suites
cargo test --test wallet_operations
cargo test --test jwt_authentication
cargo test --test signature_authorization

# Run with detailed logging
RUST_LOG=info cargo test --test wallet_operations
```

#### Test Coverage

The test suite covers:

**Authentication Methods (`src/keys.rs`)**:
- Private key loading (PEM format, file-based)
- Authorization contexts with multiple keys
- JWT user authentication and key caching
- ECDSA P-256 signature generation
- Concurrent signing operations

**Client Operations (`src/client.rs`)**:
- Canonical request generation (RFC 8785 compliance)
- Multi-key signature authorization
- HTTP client configuration and error handling
- Idempotency key support

**Wallet Operations (E2E)**:
- Wallet creation, retrieval, and deletion
- Balance checking and transaction history
- Raw message signing with authorization
- Wallet export functionality

**JWT Authentication (E2E)**:
- HPKE encryption/decryption workflows
- Encrypted and unencrypted authentication flows
- Authorization key caching and expiration
- Invalid/expired JWT handling

**Signature Authorization (E2E)**:
- Canonical request generation and signing
- Multi-key authorization scenarios
- Deterministic signing validation
- Wallet owner updates with signature auth

### Test Structure

```
tests/
├── common/
│   └── mod.rs              # Shared test utilities and environment setup
├── wallet_operations.rs    # Complete wallet lifecycle tests
├── jwt_authentication.rs   # JWT authentication and HPKE tests
└── signature_authorization.rs  # Signature-based authorization tests
```

## Development

### Environment Setup

This project uses [mise](https://mise.jdx.dev/) to manage the development environment, including the Rust toolchain version and environment variables.

1.  **Install mise:** Follow the instructions on the [mise website](https://mise.jdx.dev/getting-started.html).
2.  **Activate Environment:** Run `mise trust` in the project root to approve the configuration. `mise` will automatically pick up the Rust version from the `rust-toolchain.toml` file.

The `.mise.toml` file contains placeholders for the required environment variables. You can set them there or in a `.env` file for `mise` to load.

### Rust Version Policy

The Rust toolchain version for development and CI will not be less than two major versions behind the current stable release. This ensures a balance between modern language features and ecosystem stability.

### `Cargo.lock` Policy

**Note:** The convention for this has recently changed in Rust.

Previously, many recommended not committing Cargo.lock into the repo for libraries since potential version mismatches would be highlighted earlier, however this opinion is now changing. For that reason, this project **tracks the `Cargo.lock` file**. This ensures that all developers and the CI environment are using the exact same dependency versions, leading to more reproducible builds.

For more, see [this GH issue](https://github.com/rust-lang/cargo/issues/315)

## License

This project is dual-licensed under MIT and Apache-2.0.
