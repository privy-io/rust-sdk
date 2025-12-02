# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.1.0-alpha.5](https://github.com/privy-io/rust-sdk/compare/privy-rs-v0.1.0-alpha.4...privy-rs-v0.1.0-alpha.5) - 2025-12-02

### Added

- base introduction for alloy signer implementation

### Fixed

- fix renamed items in tests
- made it harder to enable vulnerable log statements
- clippy, naming
- feature flag in example

### Other

- Generate latest changes from OpenApi spec
- ensure nightly rustup for gen-openapi
- Ignore send transaction tests
- Generate latest changes from OpenApi spec
- add new opids to patch
- add set -e to bail when script fails
- point progenitor CLI to main
- mark more tests as flaky
- BREAKING CHANGE: improve private key memory hygiene

## [0.1.0-alpha.4](https://github.com/privy-io/rust-sdk/compare/privy-rs-v0.1.0-alpha.3...privy-rs-v0.1.0-alpha.4) - 2025-11-06

### Fixed

- fix renames

### Other

- add patches and run gen-openapi
- move to patch-based approach for openapi spec
- Merge pull request #56 from privy-io/arlyon/missing-tests
- add ignored hpke tests
- add privy client header test

## [0.1.0-alpha.3](https://github.com/privy-io/rust-sdk/compare/privy-rs-v0.1.0-alpha.2...privy-rs-v0.1.0-alpha.3) - 2025-10-02

### Fixed

- make sure to thread idempotency key into sig calc for wallet ops

## [0.1.0-alpha.2](https://github.com/privy-io/rust-sdk/compare/privy-rs-v0.1.0-alpha.1...privy-rs-v0.1.0-alpha.2) - 2025-09-30

### Fixed

- fix clippy lints

### Other

- Merge pull request #49 from privy-io/arlyon/tuple
- Merge pull request #47 from privy-io/arlyon/tracing
- Merge pull request #44 from privy-io/arlyon/base-url
- add a function to access the base url
- more race condition resolution
- stainless -> allowlist
- Merge pull request #43 from privy-io/arlyon/lint
- resolve race condition
- shorter readme + contributing doc

## [0.1.0-alpha.1](https://github.com/privy-io/rust-sdk/compare/privy-rs-v0.1.0-alpha.0...privy-rs-v0.1.0-alpha.1) - 2025-09-26

### Other

- change the version range in the readme and add badges
