# Privy Rust SDK Examples

This directory contains comprehensive examples demonstrating how to use the Privy Rust SDK for various operations. Each example is self-contained and includes detailed documentation.

## Quick Start

1. Set up your environment variables (see individual example requirements)
2. Run any example using: `cargo run --example <example_name>`

## Examples Overview

### User Management
- **[create_user.rs](create_user.rs)** - Create a new user with linked accounts
- **[delete_user.rs](delete_user.rs)** - Delete a user by user ID
- **[search_users.rs](search_users.rs)** - Search for users by various criteria

### Wallet Operations
- **[get_wallets.rs](get_wallets.rs)** - List all wallets in your app
- **[get_wallet.rs](get_wallet.rs)** - Get a specific wallet by ID
- **[create_wallet.rs](create_wallet.rs)** - Create a new embedded wallet for a user
- **[update_wallet.rs](update_wallet.rs)** - Update wallet owner (requires signature authorization)
- **[wallet_export.rs](wallet_export.rs)** - Export wallet private key (requires signature authorization)

### Wallet Signing & RPC
- **[wallet_rpc.rs](wallet_rpc.rs)** - Sign transactions using wallet RPC
- **[wallet_raw_sign.rs](wallet_raw_sign.rs)** - Sign raw data using wallet
- **[jwt_authentication.rs](jwt_authentication.rs)** - JWT-based authentication for wallet access

### Wallet Data & History
- **[wallet_balance.rs](wallet_balance.rs)** - Get wallet balance for specific assets
- **[wallet_transactions.rs](wallet_transactions.rs)** - Get wallet transaction history

### Transaction Operations
- **[get_transaction.rs](get_transaction.rs)** - Get transaction details by transaction ID

## Environment Variables

Different examples require different environment variables. Here's a comprehensive list:

### Core Credentials (Required by most examples)
```bash
# Production credentials
export PRIVY_APP_ID="your_app_id"
export PRIVY_APP_SECRET="your_app_secret"

### Wallet-specific Variables
```bash
export PRIVY_WALLET_ID="your_wallet_id"
export PRIVY_PUBLIC_KEY="your_solana_public_key"
```

### User-specific Variables
```bash
export PRIVY_USER_ID="user_id_to_operate_on"
export PRIVY_USER_JWT="valid_jwt_token"
```

### Transaction-specific Variables
```bash
export PRIVY_TRANSACTION_ID="transaction_id_to_query"
```

### Authorization Variables
```bash
export PRIVY_AUTH_SIGNATURE="base64_encoded_signature"
export PRIVY_PRIVATE_KEY_PATH="path/to/private_key.pem"
```

### Search Variables
```bash
export PRIVY_SEARCH_TERM="search_query" # Optional, defaults to "alex@arlyon.dev"
```

## Security Notes

⚠️ **Important Security Considerations:**

1. **Private Keys**: Never commit private keys to version control
2. **Authorization Signatures**: Generate these dynamically, don't hardcode them
3. **Wallet Export**: The wallet export operation reveals private keys - handle with extreme care
4. **Environment Variables**: Use secure methods to set sensitive environment variables

## Signature Authorization

Some operations (like `update_wallet` and `wallet_export`) require signature authorization. This involves:

1. Creating a canonical JSON representation of the request
2. Signing it with a P-256 private key
3. Including the signature in the `privy-authorization-signature` header

See the [update_wallet.rs](update_wallet.rs) example for a complete implementation.

## Error Handling

All examples include basic error handling using `anyhow::Result`. In production code, you should implement more sophisticated error handling based on your specific needs.

## Logging

Examples use `tracing` for structured logging. Set the `RUST_LOG` environment variable to control log levels:

```bash
export RUST_LOG=debug  # For detailed debugging
export RUST_LOG=info   # For general information (default)
export RUST_LOG=warn   # For warnings only
```

## Running Tests

Some examples can be used as integration tests. Set `PRIVY_TEST_ENABLED=true` to enable real API integration tests during development.

## Support

For questions or issues with these examples, please refer to:
- [Privy Documentation](https://docs.privy.io/)
- [SDK Repository Issues](https://github.com/privy-io/privy-rust-sdk/issues)
