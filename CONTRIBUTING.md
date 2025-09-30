Thank you for your interest in contributing to `privy-rs`! We're excited to see your contributions. This guide will help you get set up and follow the development practices of this project.

***

## Development Setup

This project uses [mise](https://mise.jdx.dev/) to manage the development environment, including the Rust toolchain version and environment variables defined in the `rust-toolchain.toml` file.

### Initial Setup

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-username/privy-rs.git
    cd privy-rs
    ```

2.  **Install mise:** Follow the instructions on the [mise website](https://mise.jdx.dev/getting-started.html).

3.  **Install dependencies:** Run `mise install` in the project root to install the required Rust toolchain and other tools.

4.  **Activate environment:** Run `mise trust` in the project root to approve the configuration. `mise` will automatically pick up the Rust version from the `rust-toolchain.toml` file.

5.  **Build the project:** Build the project to ensure everything is set up correctly. This will also trigger the initial code generation step.
    ```bash
    cargo build
    ```

The `.mise.toml` file contains placeholders for the required environment variables. You can set them there or in a `.env` file for `mise` to load.

### Environment Variables

Copy the example environment file and fill in your Privy credentials:

```bash
cp .env.example .env
```

Then edit `.env` with your actual Privy credentials:

```env
PRIVY_APP_ID=your_app_id
PRIVY_APP_SECRET=your_app_secret
PRIVY_WALLET_ID=your_wallet_id
```

See `.env.example` for all available environment variables including staging/testing configurations.

### OpenAPI Spec Management

This project uses Privy's OpenAPI specification to generate client code. To update the OpenAPI spec:

```bash
mise run pull-openapi

# this automatically runs pull first
mise run gen-openapi
```

This command:

1. Downloads the latest OpenAPI spec from `https://api.privy.io/v1/openapi.json`
2. Applies local overlay from `openapi.overlay.json` to add operation IDs and other fixes
3. Processes the spec to remove `privy-app-id` from operation parameters
4. Replaces `anyOf` with `oneOf` for better code generation
5. Saves the processed spec to `openapi.json`
6. Generates the Rust client code

***

## Testing

This project includes comprehensive test coverage across multiple levels:

### Test Categories

- **Module Tests (mod tests):** Unit tests located inside the source files they are testing (e.g., within `src/keys.rs`). They are meant to test specific, isolated functionality in a "white-box" manner. They should be fast and have no external dependencies.

- **Integration Tests (`/tests` directory):** "Black-box" tests located in the `tests/` directory. They test the public API of the crate as an external user would, ensuring different parts of the library work together correctly.

  > Important: These tests make real API calls and require environment variables to be set (e.g., PRIVY_APP_ID, PRIVY_APP_SECRET). Create a `.env` file in the root of the project to manage these secrets.

- **Doctests:** Tests written directly inside the documentation comments (`///`) of a function or module. They serve as verifiable examples, ensuring that the documentation is always correct and up-to-date. They are automatically run with the main test suite.

- **Examples (`/examples` directory):** Runnable programs that demonstrate how to use the library. While not run by `cargo test`, they serve as end-to-end integration tests and document real-world usage patterns. You can run an example with:

  ```bash
  # This also requires environment variables to be set
  cargo run --example get_wallets
  ```

### Environment Setup for Testing

#### Staging Environment Variables

For end-to-end tests, configure these environment variables:

```bash
# Required for all E2E tests
export PRIVY_TEST_APP_ID="your_staging_app_id"
export PRIVY_TEST_APP_SECRET="your_staging_app_secret"
export PRIVY_TEST_URL="https://api.staging.privy.io"  # Optional, defaults to production
export PRIVY_TEST_JWT_PRIVATE_KEY="your_test_jwt_token" # JWT authentication tests
```

### Running Tests

To run all module tests, integration tests, and doctests, use:

```bash
cargo test --all

# Run specific module tests
cargo test keys
cargo test client
```

***

## Understanding the Codebase

A significant portion of the client code in this repository is auto-generated to ensure it stays in sync with the Privy OpenAPI specification.

### Progenitor

We use `cargo-progenitor` to generate the base client and subclients. This tool is a wrapper around the `progenitor` crate, which is responsible for generating the typed subclients that provide a convenient, resource-oriented interface. The resulting code lives in `./crates/privy-openapi`.

### Code Generation (`build.rs`)

The `build.rs` script is responsible for generating the typed subclients that provide a convenient, resource-oriented interface. The process works as follows:

1.  **Base Client Generation:** The script parses code with `progenitor` and gets an AST.
2.  **Configuration Parsing:** It reads `allowlist.yml` to understand the desired structure of the final client. This YAML file defines resources (like `wallets`, `users`), maps method names to API endpoints, and defines nested subclients (e.g., `wallets().rpc()`).
3.  **Subclient Generation:** The script then parses the AST from progenitor, pulls the function signatures, and stamps out delegatory methods that forward the call onto the raw method on the base client, following the `allowlist.yml` configuration, to generate specialized **subclients** (e.g., `WalletsClient`, `UsersClient`). These subclients wrap the base `Client` and provide a more ergonomic API surface consistent with the other SDKs.
4.  **Main Client Extension:** Finally, it adds accessor methods to the main `PrivyClient` (e.g., `privy_client.wallets()`) that return instances of the generated subclients.

If you make changes to either `openapi.json` or `allowlist.yml`, `cargo` will automatically re-run this build script to regenerate the clients.

> Note: while we technically run progenitor _twice_, the build.rs file never
actually writes those files anywhere. It is only used to pull out the AST
so that we can automatically generate the sub-clients with the appropriate
function signatures.

### Rest

Otherwise, this is just your standard (well-documented) rust crate. We provide
a few manual impls for functions in the sub-clients which is managed in the
subclients folder.

***

## Contribution Workflow

To ensure code quality and consistency, please follow these steps before submitting a pull request.

### 1. Formatting

This project uses `rustfmt` to maintain a consistent code style. The configuration is defined in `rustfmt.toml`. Before committing, please format your code:

```bash
cargo fmt --all
```

While not required, you can also format the rustdoc code using `rustfmt` but only on rust nightly:

```bash
cargo +nightly fmt --all
```

### 2. Linting

We use clippy to catch common mistakes and improve code quality. Run clippy with the following command:

```bash
cargo clippy --all-targets
```

### 3. Testing

Ensure all tests pass before submitting:

```bash
cargo test --all
```

### 4. Committing Your Changes

This project uses release-plz to automate releases and changelog generation based on commit messages. Please follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification to help categorize changes in the changelog and release notes. Anything that is not a conventional commit will be just included under the 'other' header.

- `feat:` A new feature (e.g., `feat: add support for fiat on-ramping`)
- `fix:` A bug fix (e.g., `fix: correct serialization of wallet update requests`)
- `docs:` Changes to documentation only
- `style:` Formatting changes, no code logic changes
- `refactor:` Code changes that neither fix a bug nor add a feature
- `test:` Adding or refactoring tests
- `chore:` Build process or tooling changes

### 5. Submitting a Pull Request

Once your changes are ready, push them to your fork and open a pull request. Please provide a clear description of the changes and link any relevant issues. Ensure that all checks (formatting, linting, testing) pass in CI. Note that all PRs submitted are assumed to be licensed under the same conditions as the rest of the repository.

***

## Development Policies

### Rust Version Policy (MSRV)

The Rust toolchain version for development and CI will not be less than two major versions behind the current stable release. This ensures a balance between modern language features and ecosystem stability.

### `Cargo.lock` Policy

> **Note:** The convention for this has recently changed in Rust.

Previously, many recommended not committing Cargo.lock into the repo for libraries since potential version mismatches would be highlighted earlier, however this opinion is now changing. For that reason, this project **tracks the `Cargo.lock` file**. This ensures that all developers and the CI environment are using the exact same dependency versions, leading to more reproducible builds.

For more, see [this GitHub issue](https://github.com/rust-lang/cargo/issues/315).
