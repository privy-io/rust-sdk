# Publishing Guide

This document outlines the publishing process for the `privy-rs` crate.

## Automated Release Process

This project uses [release-plz](https://github.com/release-plz/release-plz) for automated versioning and publishing to crates.io.

### How It Works

1. **Automated PR Creation**: When changes are merged to the `main` branch, release-plz automatically:
   - Analyzes commit messages to determine version bumps
   - Runs `cargo-semver-checks` to detect breaking changes
   - Updates `Cargo.toml` version numbers
   - Generates/updates `CHANGELOG.md`
   - Creates a pull request with these changes

2. **Automated Publishing**: When the release PR is merged, release-plz automatically:
   - Creates a Git tag for the new version
   - Publishes the crate to crates.io
   - Creates a GitHub release

### Configuration

The release process is configured via GitHub Actions in `.github/workflows/release-plz.yml`:

- **Triggers**: Runs on pushes to `master` branch and manual workflow dispatch
- **Permissions**: Requires `REPO_SCOPED_TOKEN` and `CARGO_REGISTRY_TOKEN` secrets
- **Rust Version**: Uses minimum supported Rust version (1.88)
- **Semver Checking**: Uses `cargo-semver-checks` to detect API breaking changes

## Manual Publishing (Emergency Only)

In case the automated process fails, you can publish manually:

### Prerequisites

1. Ensure you have publishing rights to the `privy-rs` crate on crates.io
2. Authenticate with cargo: `cargo login <your-token>`
3. Verify all tests pass: `cargo test`
4. Check that the package builds: `cargo build --release`

### Steps

1. **Update Version**: Manually bump the version in `Cargo.toml`
2. **Update Changelog**: Add an entry to `CHANGELOG.md` (if it exists)
3. **Create Git Tag**: `git tag v<version>` (e.g., `git tag v0.2.0`)
4. **Publish**: `cargo publish`
5. **Push Tag**: `git push origin v<version>`

## Versioning Strategy

This project follows [Semantic Versioning](https://semver.org/):

- **MAJOR** version for incompatible API changes
- **MINOR** version for backward-compatible functionality additions
- **PATCH** version for backward-compatible bug fixes

### Conventional Commits

Use conventional commit messages to help release-plz determine version bumps:

- `fix:` - triggers a PATCH version bump
- `feat:` - triggers a MINOR version bump
- `BREAKING CHANGE:` or `!` suffix - triggers a MAJOR version bump

Examples:
```
fix: resolve authentication timeout issue
feat: add support for multiple wallet types
feat!: change client initialization API (BREAKING CHANGE)
```

## Pre-Release Checklist

Before any release (automated or manual):

- [ ] All tests pass locally and in CI
- [ ] Documentation is up to date
- [ ] Breaking changes are documented
- [ ] Environment variable requirements are documented
- [ ] Examples work with the new version

## OpenAPI Updates

This project automatically updates its OpenAPI client code weekly via GitHub Actions:

### Weekly OpenAPI Sync

The `.github/workflows/openapi.yml` workflow:

1. **Schedule**: Runs every Monday at 10:00 AM UTC
2. **Process**:
   - Pulls the latest OpenAPI specification
   - Regenerates client code using `mise pull-openapi`
   - Creates a pull request with any changes
3. **Review**: Automated PRs are assigned to maintainers for review

### Manual OpenAPI Updates

To manually update the OpenAPI client:

```bash
mise pull-openapi
```

This will regenerate the client code based on the latest API specification.

## Secrets Configuration

The automated release process requires these GitHub repository secrets:

- `REPO_SCOPED_TOKEN`: GitHub personal access token with repo permissions
- `CARGO_REGISTRY_TOKEN`: crates.io API token for publishing

## Related Documentation

- [Cargo Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [release-plz Documentation](https://release-plz.ieni.dev/)
- [Semantic Versioning](https://semver.org/)
