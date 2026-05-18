# Testing

## Structure

```
tests/
├── common/mod.rs         # Shared helpers (client setup, wallet/user/policy creation, JWT minting)
├── wallets.rs            # Wallet CRUD, RPC signing (Ethereum + Solana), import, export
├── users.rs              # User CRUD and lookup by various identifiers
├── policies.rs           # Policy and rule CRUD with authorization
├── key_quorums.rs        # Key quorum management with authorization
├── transactions.rs       # Transaction queries
├── e2e.rs                # End-to-end quorum wallet workflow (2-of-3 signing)
├── client_behavior.rs    # Client config tests (uses httpmock, no real API calls)
├── alloy_integration.rs  # Alloy signer adapter tests
├── fiat.rs               # Fiat on/off-ramp operations
├── test_private_key.pem  # P-256 test key for authorization tests
└── test_public_key.pem
```

## Running Tests

```sh
# All tests (requires env vars for API access)
cargo test

# Single test file
cargo test --test wallets

# Single test function
cargo test --test wallets test_wallets_ethereum_sign_message

# With debug logging
cargo test --test wallets test_wallets_export -- --nocapture

# Include ignored tests (fund-dependent or broken endpoints)
cargo test -- --ignored

# All features (includes alloy integration)
cargo test --all-features
```

## Environment Variables

Tests hit a real staging API. Required secrets:

| Variable | Purpose |
|----------|---------|
| `PRIVY_TEST_APP_ID` | App identifier (fallback: `PRIVY_APP_ID`) |
| `PRIVY_TEST_APP_SECRET` | App secret (fallback: `PRIVY_APP_SECRET`) |
| `PRIVY_TEST_JWT_PRIVATE_KEY` | RSA PEM key for minting test JWTs |
| `PRIVY_TEST_URL` | Staging API URL (fallback: `PRIVY_URL`, or production default) |

Set these in `.env.local` (loaded by mise) or export them directly.

## Test Helpers (`tests/common/mod.rs`)

| Helper | Purpose |
|--------|---------|
| `get_test_client()` | Creates `PrivyClient` from env vars |
| `get_test_wallet_id_by_type(client, chain, owner)` | Creates a wallet and returns its ID |
| `get_test_wallet_by_type(client, chain, owner)` | Creates a wallet and returns the full object |
| `ensure_test_user(client)` | Creates a user with email + custom JWT linked accounts |
| `ensure_test_policy(client, rules)` | Creates a policy with given rules |
| `ensure_test_policy_with_user(client, rules, user)` | Creates an owned policy |
| `mint_staging_jwt(sub)` | Mints a JWT signed with `PRIVY_TEST_JWT_PRIVATE_KEY` |

## The `debug_response!` Macro

Wraps async API calls to provide better error output on failure. On `UnexpectedResponse`, it reads and prints the response body before panicking:

```rust
let wallet = debug_response!(client.wallets().get(&wallet_id)).await?;
```

Use this for any API call where you want visibility into unexpected failures.

## Writing New Tests

1. Create a new file in `tests/` or add to an existing one.
2. Include the common module: `mod common;`
3. Use `#[tokio::test]` for async tests.
4. Use `get_test_client()?` to get a configured client.
5. Wrap API calls with `debug_response!()` for better error output.
6. Use `#[traced_test]` (from `tracing_test`) when you need log output.

```rust
use anyhow::Result;
use common::get_test_client;
use privy_rs::{AuthorizationContext, generated::types::*};

mod common;

#[tokio::test]
async fn test_my_feature() -> Result<()> {
    let client = get_test_client()?;
    let wallet_id = common::get_test_wallet_id_by_type(
        &client, WalletChainType::Ethereum, None
    ).await?;

    // Test your feature...
    let result = debug_response!(client.wallets().get(&wallet_id)).await?;
    assert_eq!(result.chain_type, WalletChainType::Ethereum);

    Ok(())
}
```

## Test Annotations

- `#[ignore = "reason"]` — skipped by default (fund-dependent or broken upstream)
- `#[mark_flaky_tests::flaky]` — retried up to `MARK_FLAKY_TESTS_RETRIES` times in CI (default 20)
- `#[traced_test]` — captures tracing output for the test

## Mock Tests

For tests that don't need a real API (client config, header validation), use `httpmock`:

```rust
use httpmock::prelude::*;
use privy_rs::{PrivyClient, client::PrivyClientOptions};

let server = MockServer::start();
let mock = server.mock(|when, then| {
    when.method(GET).path("/v1/users");
    then.status(200).json_body(serde_json::json!({"data": []}));
});

let client = PrivyClient::new_with_options(
    "app_id".into(), "secret".into(),
    PrivyClientOptions { base_url: server.base_url(), ..Default::default() },
).unwrap();
```
