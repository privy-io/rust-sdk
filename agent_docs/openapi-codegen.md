# OpenAPI Codegen & Release

## Pulling the Latest Spec

```sh
mise pull-openapi
```

Fetches `https://api.privy.io/v1/openapi.json` and pipes it through 12 jq patches in `scripts/schema-patches/`:

| Patch | Purpose |
|-------|---------|
| 01-operation-ids | Adds missing `operationId` fields to legacy endpoints |
| 02-owner-input | Fixes owner input schema structure |
| 03-solana-wallet-required-fields | Marks required fields on Solana wallet types |
| 04-remove-privy-app-id-param | Removes `privy-app-id` parameter (handled by client headers) |
| 05-remove-deprecated | Strips deprecated endpoints from the spec |
| 06-remove-additional-properties | Removes `additionalProperties` (Progenitor can't handle them) |
| 07-anyof-to-oneof | Converts `anyOf` to `oneOf` for discriminated union generation |
| 08-coinbase-onramp-assets | Fixes Coinbase onramp asset schema |
| 09-deduplicate-enums | Removes duplicate enum variants |
| 10-kraken-multipart | Fixes Kraken multipart request schema |
| 11-deduplicate-properties | Removes duplicate properties within objects |
| 12-remove-custom-oauth | Removes custom OAuth endpoints |

Output: `openapi.json` in the repo root.

## Regenerating the SDK

```sh
mise gen-openapi
```

This runs `pull-openapi` first, then:
1. Installs nightly Rust (Progenitor's formatter requires it)
2. Runs `cargo-progenitor` against `openapi.json` → regenerates `crates/privy-openapi/`

After regeneration, `cargo build` triggers `build.rs` which:
1. Runs Progenitor again to produce `$OUT_DIR/codegen.rs` (base client with all methods)
2. Parses `allowlist.yml` to determine which methods belong to which resource
3. Parses the generated AST to extract method signatures
4. Generates `$OUT_DIR/subclients.rs` with resource-oriented wrapper structs

## allowlist.yml

Stainless-compatible config that maps resources → methods → endpoints:

```yaml
resources:
  wallets:
    methods:
      list: get /v1/wallets           # Public method on WalletsClient
      _rpc:                            # Underscore = private (wrapped by hand-written code)
        endpoint: post /v1/wallets/{wallet_id}/rpc
        only: [python, typescript, node, rust]
```

- Simple string value: generates a public delegating method
- `_prefix`: generates a private method (hand-written wrapper provides the public API)
- `only: [rust]`: only generate for listed languages
- `skip: [rust]`: skip generation for listed languages
- `subresources`: nested clients (e.g., `wallets.transactions`)

## Fixing Issues After Spec Update

When the new spec breaks compilation:

### Schema-level issues → Add/modify a jq patch
- Duplicate enum variants, missing required fields, `anyOf` where `oneOf` is needed
- Create a new numbered file in `scripts/schema-patches/` and add it to the pipeline in `.mise.toml`

### New or changed endpoints → Update `allowlist.yml`
- Add new resources or methods
- Adjust `only`/`skip` rules if a method isn't relevant to Rust

### Generated type changes break hand-written code → Update extensions
- `src/subclients/wallets.rs`, `src/subclients/policies.rs`, `src/subclients/key_quorums.rs`
- `src/ethereum.rs`, `src/solana.rs`
- Common: field renames, type changes in request/response bodies

### Iteration loop
```sh
mise gen-openapi && cargo build 2>&1 | head -50
# Fix issues, repeat
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## CI Automation

### Weekly OpenAPI Update (`.github/workflows/openapi.yml`)
- Runs every Monday at 10am UTC (or manual dispatch)
- Executes `mise gen-openapi`
- Opens a PR automatically if the spec changed (reviewer: `arlyon`)

### CI (`.github/workflows/ci.yml`)
- Runs on every push/PR to `main`
- `cargo test --verbose`
- `cargo clippy --all-targets --all-features -- -D warnings`
- Uses secrets: `PRIVY_TEST_APP_ID`, `PRIVY_TEST_APP_SECRET`, `PRIVY_TEST_JWT_PRIVATE_KEY`, `PRIVY_TEST_URL`

## Release Process

Uses [release-plz](https://release-plz.ieni.dev/) (`.github/workflows/release-plz.yml`):

1. **On push to `main`**: `release-plz-pr` job creates/updates a release PR with:
   - Version bump (uses `cargo-semver-checks` to detect breaking changes)
   - Auto-generated changelog via git-cliff
   - Dependency updates

2. **When release PR merges**: `release-plz-release` job:
   - Publishes to crates.io
   - Creates a GitHub release with tag

Config in `release-plz.toml` enables semver checking, changelog generation, and dependency updates.

## Key Files

| File | Role |
|------|------|
| `openapi.json` | Patched OpenAPI spec (committed, regenerated) |
| `allowlist.yml` | Resource/method routing config |
| `build.rs` | Build-time code generation orchestrator |
| `crates/privy-openapi/` | Generated crate (types + base client) |
| `scripts/schema-patches/*.jq` | Spec fixup patches |
| `.mise.toml` | Task definitions (pull-openapi, gen-openapi) |
| `release-plz.toml` | Release configuration |
