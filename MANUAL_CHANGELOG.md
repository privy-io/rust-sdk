# Manual Changelog

<!--
This file contains detailed change notes written during SDK regenerations and other
significant changes. It complements the auto-generated CHANGELOG.md (managed by
release-plz from commit messages) with context that commit messages alone can't
capture — such as breaking type renames, migration guidance, and new schema patches.
-->

## Unreleased — Request expiry

Signed requests now send `privy-request-expiry` (absolute Unix timestamp in
milliseconds), and the authorization signature covers it.

### Added

- `privy_rs::request_expiry` module: `DEFAULT_REQUEST_EXPIRY` (15 minutes),
  `DEFAULT_INTENT_REQUEST_EXPIRY` (72 hours, for intents once the SDK exposes them),
  `request_expiry_from_now(Duration) -> u64`, and `RequestExpiryOptions`
  (`disabled`, `default_expiry`).
- `PrivyClientOptions::request_expiry` to configure the client default.
- `RequestOptions { request_expiry }` and a `*_with_options` variant of every signed
  wrapper: `WalletsClient::{rpc, raw_sign, update, export}_with_options`,
  `PoliciesClient::{update, delete, create_rule, update_rule, delete_rule}_with_options`,
  `KeyQuorumsClient::{update, delete}_with_options`.
- `request_expiry` field and `with_request_expiry` on `SendTransactionOptions` and
  `SignAndSendTransactionOptions`.
- `AuthorizationSignatureHeaders`, `format_request_for_authorization_signature_with_headers`
  and `generate_authorization_signatures_with_headers` for signing requests that carry
  an idempotency key and/or request expiry.

### Changed

- All signed wrappers, and the Ethereum, Solana and alloy helpers built on `rpc`, send
  `privy-request-expiry` set to 15 minutes after the request is sent, unless the client
  disables it. A per-request value takes precedence and is sent even when disabled.

### Changed (Breaking)

- `PrivyClientOptions` has a new public field, `request_expiry`. Struct literals that
  list every field must add it or use `..Default::default()`.

## 2026-06-08 — OpenAPI Regeneration (3.0 → 3.1)

Regenerated from latest Privy OpenAPI spec. The upstream spec upgraded from OpenAPI 3.0 to 3.1.0, requiring three new schema patches to maintain progenitor compatibility.

### Added

- `PATCH /v1/users/{user_id}/custom_metadata` endpoint (`patchUserCustomMetadata`)
- `executor` field on `EthereumSign7702AuthorizationRpcInputParams`
- `wallet_id` field on all RPC input types
- `experimental_data_suffix`, `reference_id` fields on `EthereumSendTransactionRpcInput`
- `optimistic_broadcast`, `reference_id` fields on `SolanaSignAndSendTransactionRpcInput`
- `display_name`, `external_id` fields on `CreateWalletBody`
- `key_quorum_ids` field on `KeyQuorumCreateRequestBody`
- `hpke_config` field on `WalletImportSubmissionRequest`
- `Quantity` type for representing integer-or-hex values (chain_id, value, nonce, etc.)
- `Hex` newtype for hex-encoded strings
- `UnsignedStandardEthereumTransaction` struct with `authorization_list` field
- `UnsignedEthereumTransaction` enum wrapping standard and 7702 transaction types
- `TypedDataDomainInputParams`, `TypedDataTypesInputParams`, `TypedDataTypeFieldInput` types
- `EthereumTypedDataInput` type (replaces `EthereumSignTypedDataRpcInputParamsTypedData`)
- `WalletSolanaAsset`, `WalletEthereumAsset` enums for asset parameters
- `P256PublicKey` newtype wrapper
- Schema patches: `13-downgrade-openapi-version.jq`, `14-fix-responses.jq`, `15-fix-nullable-types.jq`

### Changed (Breaking)

- `WalletRpcBody` renamed to `WalletRpcRequestBody`
- `AuthenticateBody` renamed to `WalletAuthenticateRequestBody`
- `UpdateWalletBody` renamed to `WalletUpdateRequestBody`
- `CreateKeyQuorumBody` renamed to `KeyQuorumCreateRequestBody`
- `CreateKeyQuorumBodyDisplayName` renamed to `KeyQuorumCreateRequestBodyDisplayName`
- `WalletImportInitializationRequest` renamed to `WalletImportInitBody`
- `EthereumSignTypedDataRpcInputParamsTypedData` renamed to `EthereumTypedDataInput`
- `EthereumSignTransactionRpcInputParamsTransaction` replaced by `UnsignedEthereumTransaction`
- `EthereumSendTransactionRpcInputParamsTransaction` replaced by `UnsignedEthereumTransaction`
- `EthereumSign7702AuthorizationRpcInputParamsChainId` replaced by `Quantity`
- `RawSign` renamed to `RawSignInput`, `RawSignParams` renamed to `RawSignInputParams`
- `RawSignParams::Hash { hash }` changed to `RawSignInputParams::HashParams(RawSignHashParams { hash })`
- `OwnerInput::PublicKey(String)` changed to `OwnerInput::Variant1(OwnerInputPublicKey { public_key: P256PublicKey })`
- `WalletImportSubmissionRequestOwner` merged into `OwnerInput`
- `wallets().list()` now takes 6 parameters (added `chain_type` and `wallet_index`)
- `AuthenticateBody.encryption_type` changed from `Option<_>` to required field
- `AuthenticateBody.recipient_public_key` changed from `Option<String>` to required `String`
- `CreateWalletBody.policy_ids` changed from `Vec<String>` to `Option<Vec<String>>`
- `KeyQuorumCreateRequestBody.authorization_threshold` changed from `f64` to `Option<f64>`
- String fields that accept specific formats now use newtype wrappers (e.g. `message` fields require `.parse().unwrap()`)
- `GetWalletBalanceAsset` and `GetWalletBalanceChain` parameters changed from required to `Option<&_>`
- `WalletTransactionsAsset` parameter changed from required to `Option<&_>`
- `additional_signers` on wallet import changed from `Vec<_>` to `Option<AdditionalSignerInput>`
- `policy_ids` on wallet import wrapped in `PolicyInput` newtype

### Removed

- `EthereumSign7702AuthorizationRpcInputParamsChainId` enum (replaced by `Quantity`)
- `EthereumSignTransactionRpcInputParamsTransaction` struct (replaced by `UnsignedEthereumTransaction`)
- `EthereumSendTransactionRpcInputParamsTransaction` struct (replaced by `UnsignedEthereumTransaction`)
- `GetWalletBalanceAssetString` enum (replaced by chain-specific asset enums)
- `WalletTransactionsAssetString` enum (replaced by chain-specific asset enums)
- `WalletImportSubmissionRequestOwner` type (merged into `OwnerInput`)
