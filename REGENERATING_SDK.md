# Regenerating the Privy Rust SDK

This document explains how to regenerate the Privy Rust SDK from the OpenAPI specification.

## Overview

The Privy Rust SDK is generated using a combination of tools:

1. **OpenAPI Spec**: Downloaded from the Privy API
2. **Progenitor**: Rust code generator for OpenAPI specs
3. **Stainless Configuration**: Resource structure and method configuration
4. **Custom Build Script**: Combines everything into structured client code

## Prerequisites

- [mise](https://mise.jdx.dev/) installed and configured
- Rust toolchain (managed by mise)
- `jq` (managed by mise)

## Setup

1. Install dependencies:
   ```bash
   mise install
   ```

2. Activate the environment:
   ```bash
   mise trust
   ```

## Regeneration Process

### Step 1: Update OpenAPI Specification

Pull the latest OpenAPI spec from the Privy API:

```bash
mise run pull-openapi
```

This command:
- Downloads the spec from `https://api.privy.io/v1/openapi.json`
- Applies local overlay from `openapi.overlay.json` to add operation IDs
- Removes `privy-app-id` from operation parameters (handled via security scheme)
- Replaces `anyOf` with `oneOf` for better code generation
- Saves the processed spec to `openapi.json`

### Step 2: Configure Resource Structure

The SDK structure is defined in `stainless.yml`. This file contains:

- **Resources**: Top-level API resources (wallets, users, policies, etc.)
- **Methods**: HTTP operations mapped to client methods
- **Subresources**: Nested resources (e.g., `wallets.transactions`)
- **Language-specific rules**: Method inclusion/exclusion by language

Key configuration patterns:

```yaml
resources:
  wallets:
    methods:
      list: get /v1/wallets
      create: post /v1/wallets
      # Private methods prefixed with underscore
      _rpc:
        endpoint: post /v1/wallets/{wallet_id}/rpc
        only: [rust]  # Include only for Rust
    subresources:
      transactions:
        methods:
          get: get /v1/wallets/{wallet_id}/transactions
```

### Step 3: Build Process

The code generation happens automatically during `cargo build` via `build.rs`:

1. **Base Generation**: Progenitor generates core `Client` from `openapi.json`
2. **Configuration Parsing**: Reads `stainless.yml` for resource structure
3. **Method Mapping**: Maps YAML method names to OpenAPI operation IDs
4. **Subclient Generation**: Creates typed client structs for each resource
5. **Code Output**: Writes generated code to `$OUT_DIR/codegen.rs` and `$OUT_DIR/subclients.rs`

### Step 4: Rebuild the SDK

After updating the OpenAPI spec or configuration:

```bash
cargo clean
cargo build
```

This will regenerate all client code based on the latest spec and configuration.

## Generated Code Structure

The build process creates:

- **Base Client** (`codegen.rs`): Low-level HTTP client with all API operations
- **Subclients** (`subclients.rs`): High-level, resource-oriented clients:
  - `WalletsClient` - wallet operations
  - `UsersClient` - user management
  - `PoliciesClient` - policy management
  - `WalletsTransactionsClient` - wallet transaction history
  - etc.

## Manual Customization

Some methods are prefixed with `_` in the configuration to allow custom implementations:

- `_rpc` → custom `rpc` method with authorization context
- `_raw_sign` → custom `raw_sign` method with authorization
- `_update` → custom `update` method with signature authorization

These custom implementations are in `src/client.rs` and provide additional functionality like signature authorization.

## Testing Generated Code

After regeneration, run the test suite:

```bash
# Unit tests
cargo test --lib

# Integration tests (requires staging environment)
cargo test --test "*"
```

## Troubleshooting

### Missing Operation IDs

If progenitor fails due to missing operation IDs, update `openapi.overlay.json`:

```json
{
  "paths": {
    "/v1/some/endpoint": {
      "post": {
        "operationId": "createSomething"
      }
    }
  }
}
```

### Method Mapping Issues

If methods aren't generating correctly, check:

1. **Endpoint format** in `stainless.yml`: `"get /v1/resource"`
2. **Operation ID** in OpenAPI spec matches expected pattern
3. **Language rules** (`only`/`skip`) are correct for Rust

### Build Failures

For build script issues:

```bash
# See build script output
cargo build -v

# Clean and rebuild
cargo clean && cargo build
```

## Dependencies

The generation process uses these key dependencies:

- **progenitor**: OpenAPI to Rust code generation
- **openapiv3**: OpenAPI spec parsing
- **serde_yaml**: Configuration file parsing
- **syn/quote**: Rust AST manipulation
- **prettyplease**: Code formatting

Custom progenitor branch includes fixes for Privy-specific patterns.