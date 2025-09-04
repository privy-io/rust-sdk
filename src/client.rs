//! Privy client implementations.
//!
//! This module contains the `PrivyClient` with typed wallet support.

use std::time::Duration;

use delegate::delegate;
use privy_api::Client;
use reqwest::header::{CONTENT_TYPE, HeaderValue};
use serde::Serialize;

use crate::{Method, PrivyCreateError, WalletApiRequestSignatureInput, get_auth_header};

/// Privy client for interacting with the Privy API.
///
/// This provides access to global operations like user and wallet management.
/// For wallet-specific operations, use `TypedWallet<T>` instances created via
/// the `wallet()` method.
#[derive(Clone, Debug)]
pub struct PrivyClient {
    pub(crate) app_id: String,
    #[allow(dead_code)]
    pub(crate) app_secret: String,
    pub(crate) client: Client,
}

impl PrivyClient {
    /// Create a new `PrivyClient`
    ///
    /// # Errors
    /// See [`PrivyClient::new_with_url`]
    pub fn new(app_id: String, app_secret: String) -> Result<Self, PrivyCreateError> {
        Self::new_with_url(app_id, app_secret, "https://api.privy.io")
    }

    /// Create a new `PrivyClient` with a custom url
    ///
    /// # Errors
    /// This can fail for two reasons, either the `app_id` or `app_secret` are not
    /// valid headers, or that the underlying http client could not be created.
    pub fn new_with_url(
        app_id: String,
        app_secret: String,
        url: &str,
    ) -> Result<Self, PrivyCreateError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&get_auth_header(&app_id, &app_secret))?,
        );
        headers.insert("privy-app-id", HeaderValue::from_str(&app_id)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("privy-client", HeaderValue::from_static("rust-sdk"));

        let client_with_custom_defaults = reqwest::ClientBuilder::new()
            .connect_timeout(Duration::from_secs(15))
            .timeout(Duration::from_secs(15))
            .default_headers(headers)
            .build()?;

        Ok(Self {
            app_id,
            app_secret,
            client: Client::new_with_client(url, client_with_custom_defaults),
        })
    }

    // this is the crux of the impl, a handy macro that delegates all the
    // unexciting methods to the inner client automatically. we can do nice
    // things like auto-populating items on the builders
    delegate! {
        to self.client {
            /// Authenticate a user using a JWT
            #[expr($.privy_app_id(&self.app_id))]
            #[must_use] pub fn authenticate(&self) -> privy_api::builder::Authenticate<'_>;

            /// Get a wallet
            #[expr($.privy_app_id(&self.app_id))]
            #[must_use] pub fn get_wallet(&self) -> privy_api::builder::GetWallet<'_>;

            /// Get a list of wallets
            #[expr($.privy_app_id(&self.app_id))]
            #[must_use] pub fn get_wallets(&self) -> privy_api::builder::GetWallets<'_>;

            /// Create a new wallet
            #[expr($.privy_app_id(&self.app_id))]
            #[must_use] pub fn create_wallet(&self) -> privy_api::builder::CreateWallet<'_>;

            /// Update a wallet
            #[expr($.privy_app_id(&self.app_id))]
            #[must_use] pub fn update_wallet(&self) -> privy_api::builder::UpdateWallet<'_>;

            /// Create a new user
            #[expr($.privy_app_id(&self.app_id))]
            #[must_use] pub fn create_user(&self) -> privy_api::builder::CreateUser<'_>;

            /// Get a user
            #[expr($.privy_app_id(&self.app_id))]
            #[must_use] pub fn get_user(&self) -> privy_api::builder::GetUser<'_>;

            /// Get a list of users
            #[expr($.privy_app_id(&self.app_id))]
            #[must_use] pub fn get_users(&self) -> privy_api::builder::GetUsers<'_>;

            /// Delete a user
            #[expr($.privy_app_id(&self.app_id))]
            #[must_use] pub fn delete_user(&self) -> privy_api::builder::DeleteUser<'_>;

            /// Search for users
            #[expr($.privy_app_id(&self.app_id))]
            #[must_use] pub fn search_users(&self) -> privy_api::builder::SearchUsers<'_>;

            /// Create a new user wallet
            #[expr($.privy_app_id(&self.app_id))]
            #[must_use] pub fn create_user_wallet(&self) -> privy_api::builder::CreateUserWallet<'_>;
        }
    }

    /// Create canonical request data for signing
    ///
    /// # Errors
    /// This can fail if JSON serialization fails
    pub fn build_canonical_request<S: Serialize>(
        &self,
        method: Method,
        url: String,
        body: S,
        idempotency_key: Option<String>,
    ) -> Result<String, serde_json::Error> {
        let mut headers = serde_json::Map::new();
        headers.insert(
            "privy-app-id".into(),
            serde_json::Value::String(self.app_id.clone()),
        );
        if let Some(key) = idempotency_key {
            headers.insert(
                "privy-idempotency-key".to_string(),
                serde_json::Value::String(key),
            );
        }

        WalletApiRequestSignatureInput::new(method, url)
            .headers(serde_json::Value::Object(headers))
            .body(body)
            .canonicalize()
    }

    /// Create canonical request data for wallet update operations
    ///
    /// # Errors
    /// This can fail if JSON serialization fails
    pub fn build_update_wallet_canonical_request<S: Serialize>(
        &self,
        wallet_id: &str,
        body: S,
        idempotency_key: Option<String>,
    ) -> Result<String, serde_json::Error> {
        let url = format!("https://api.privy.io/v1/wallets/{wallet_id}");
        self.build_canonical_request(Method::PATCH, url, Some(body), idempotency_key)
    }
}
