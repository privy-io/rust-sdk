#!/usr/bin/env jq -f

# Set operation IDs for API endpoints that don't already have them
# This patch assigns meaningful operationId values to legacy endpoints that are missing them.
# The code generator uses these to create properly named Rust methods.
#
# Note: Newer endpoints (policies/{policy_id}/rules, condition_sets, passkeys, kraken_embed,
# and wallets/{wallet_id}/export) already have operationIds in the raw spec and are not
# included here to avoid conflicts.
#
# Operation IDs are organized by resource:
# - Users: CRUD operations and lookups by various identifiers
# - Wallets: CRUD, RPC calls, signing, transactions, import
# - Policies: CRUD for policies only (rules already have IDs)
# - Key Quorums: CRUD operations
# - Fiat: On/off-ramping, KYC, accounts, and status checks

.paths = .paths * {
  "/v1/users": {
    "post": { "operationId": "createUser" },
    "get": { "operationId": "getUsers" }
  },
  "/v1/users/{user_id}": {
    "get": { "operationId": "getUser" },
    "delete": { "operationId": "deleteUser" }
  },
  "/v1/users/search": {
    "post": { "operationId": "searchUsers" }
  },
  "/v1/users/{user_id}/wallets": {
    "post": { "operationId": "createUserWallet" }
  },
  "/v1/users/{user_id}/custom_metadata": {
    "post": { "operationId": "updateUserCustomMetadata" }
  },
  "/v1/wallets": {
    "post": { "operationId": "createWallet" },
    "get": { "operationId": "getWallets" }
  },
  "/v1/wallets/{wallet_id}/rpc": {
    "post": { "operationId": "walletRpc" }
  },
  "/v1/wallets/{wallet_id}": {
    "get": { "operationId": "getWallet" },
    "patch": { "operationId": "updateWallet" }
  },
  "/v1/wallets/{wallet_id}/raw_sign": {
    "post": { "operationId": "rawSign" }
  },
  "/v1/wallets/{wallet_id}/balance": {
    "get": { "operationId": "getWalletBalance" }
  },
  "/v1/wallets/{wallet_id}/transactions": {
    "get": { "operationId": "walletTransactions" }
  },
  "/v1/wallets/authenticate": {
    "post": { "operationId": "authenticate" }
  },
  "/v1/wallets/import/init": {
    "post": { "operationId": "walletImportInit" }
  },
  "/v1/wallets/import/submit": {
    "post": { "operationId": "walletImportSubmit" }
  },
  "/v1/transactions/{transaction_id}": {
    "get": { "operationId": "getTransaction" }
  },
  "/v1/policies": {
    "post": { "operationId": "createPolicy" }
  },
  "/v1/policies/{policy_id}": {
    "get": { "operationId": "getPolicy" },
    "patch": { "operationId": "updatePolicy" },
    "delete": { "operationId": "deletePolicy" }
  },
  "/v1/key_quorums/{key_quorum_id}": {
    "get": { "operationId": "getKeyQuorum" },
    "patch": { "operationId": "updateKeyQuorum" },
    "delete": { "operationId": "deleteKeyQuorum" }
  },
  "/v1/key_quorums": {
    "post": { "operationId": "createKeyQuorum" }
  },
  "/v1/users/{user_id}/fiat/status": {
    "post": { "operationId": "userFiatStatuses" }
  },
  "/v1/users/{user_id}/fiat/tos": {
    "post": { "operationId": "createUserFiatTos" }
  },
  "/v1/users/{user_id}/fiat/kyc": {
    "post": { "operationId": "initiateUserFiatKyc" },
    "get": { "operationId": "getUserFiatKycStatus" },
    "patch": { "operationId": "updateUserFiatKycStatus" }
  },
  "/v1/users/{user_id}/fiat/kyc_link": {
    "post": { "operationId": "getUserFiatKycLink" }
  },
  "/v1/users/{user_id}/fiat/accounts": {
    "get": { "operationId": "getUserFiatAccounts" },
    "post": { "operationId": "createUserFiatAccount" }
  },
  "/v1/users/{user_id}/fiat/offramp": {
    "post": { "operationId": "initiateUserFiatOfframp" }
  },
  "/v1/users/{user_id}/fiat/onramp": {
    "post": { "operationId": "initiateUserFiatOnramp" }
  },
  "/v1/apps/{app_id}/fiat": {
    "post": { "operationId": "configureAppForFiatOnOffRamping" }
  },
  "/v1/users/email/address": {
    "post": { "operationId": "lookUpUserByEmail" }
  },
  "/v1/users/custom_auth/id": {
    "post": { "operationId": "lookUpUserByCustomAuthId" }
  },
  "/v1/users/wallet/address": {
    "post": { "operationId": "lookUpUserByWalletAddress" }
  },
  "/v1/users/farcaster/fid": {
    "post": { "operationId": "lookUpUserByFarcasterId" }
  },
  "/v1/users/phone/number": {
    "post": { "operationId": "lookUpUserByPhoneNumber" }
  },
  "/v1/users/smart_wallet/address": {
    "post": { "operationId": "lookUpUserBySmartWalletAddress" }
  },
  "/v1/users/discord/username": {
    "post": { "operationId": "lookUpUserByDiscordUsername" }
  },
  "/v1/users/github/username": {
    "post": { "operationId": "lookUpUserByGithubUsername" }
  },
  "/v1/users/twitter/username": {
    "post": { "operationId": "lookUpUserByTwitterUsername" }
  },
  "/v1/users/twitter/subject": {
    "post": { "operationId": "lookUpUserByTwitterSubject" }
  },
  "/v1/users/telegram/telegram_user_id": {
    "post": { "operationId": "lookUpUserByTelegramUserId" }
  },
  "/v1/users/telegram/username": {
    "post": { "operationId": "lookUpUserByTelegramUsername" }
  },
  "/v1/users/{user_id}/accounts": {
    "post": { "operationId": "addOrUpdateUserLinkedAccount" }
  },
  "/v1/users/{user_id}/accounts/unlink": {
    "post": { "operationId": "unlinkUserLinkedAccount" }
  },
  "/v1/custodial_wallets": {
    "post": { "operationId": "getCustodialWallets" }
  },
  "/v1/kraken_embed/assets/{asset_id}": {
    "get": { "operationId": "getKrakenEmbedAssets" }
  }
}
