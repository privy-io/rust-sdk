//! Privy client implementations.
//!
//! This module contains the `PrivyClient` with typed wallet support.

use std::time::Duration;

use delegate::delegate;
use progenitor_client::{ByteStream, ResponseValue};
use reqwest::header::{CONTENT_TYPE, HeaderValue};
use serde::Serialize;

use crate::{
    AuthorizationContext, Method, PrivyCreateError, WalletApiRequestSignatureInput,
    generated::{
        Client,
        types::{
            AuthenticateBody, AuthenticateResponse, CreateUserBody, CreateWalletBody,
            GetWalletsChainType, GetWalletsCursor, GetWalletsResponse, UpdateWalletBody, User,
            Wallet,
        },
    },
    get_auth_header,
    middleware::MiddlewareState,
};

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
    /// # Usage
    /// ```no_run
    /// # use privy_rust::{PrivyClient, PrivyCreateError, PrivateKeyFromFile, AuthorizationContext};
    /// # async fn foo() -> Result<(), PrivyCreateError> {
    /// let ctx = AuthorizationContext::new();
    /// let client = PrivyClient::new("app_id".into(), "app_secret".into(), ctx.clone())?;
    /// ctx.push(PrivateKeyFromFile("private_key.pem".into()));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// This can fail for two reasons, either the `app_id` or `app_secret` are not
    /// valid headers, or that the underlying http client could not be created.
    pub fn new(
        app_id: String,
        app_secret: String,
        ctx: AuthorizationContext,
    ) -> Result<Self, PrivyCreateError> {
        Self::new_with_url(app_id, app_secret, ctx, "https://api.privy.io")
    }

    /// Create a new `PrivyClient` with a custom url
    ///
    /// # Errors
    /// This can fail for two reasons, either the `app_id` or `app_secret` are not
    /// valid headers, or that the underlying http client could not be created.
    pub fn new_with_url(
        app_id: String,
        app_secret: String,
        ctx: AuthorizationContext,
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
            app_id: app_id.clone(),
            app_secret,
            client: Client::new_with_client(
                url,
                client_with_custom_defaults,
                MiddlewareState {
                    app_id,
                    ctx: ctx.clone(),
                },
            ),
        })
    }

    /// Update a wallet
    #[must_use]
    pub fn update_wallet<'a>(
        &'a self,
        wallet_id: &'a str,
        body: &'a UpdateWalletBody,
    ) -> impl Future<Output = Result<ResponseValue<Wallet>, crate::generated::Error>> + 'a {
        // NOTE: this is handled in the middleware
        self.client.update_wallet(wallet_id, None, body)
    }

    // this is the crux of the impl, a handy macro that delegates all the
    // unexciting methods to the inner client automatically. we can do nice
    // things like auto-populating items on the builders

    delegate! {
        to self.client {
            /// Authenticate a user using a JWT
            #[must_use] pub fn authenticate<'a>(&'a self, body: &'a AuthenticateBody) -> impl Future<Output =  Result<ResponseValue<AuthenticateResponse>, crate::generated::Error>> + 'a;

            /// Get a wallet
            #[must_use] pub fn get_wallet<'a>(&'a self, wallet_id: &'a str) -> impl Future<Output = Result<ResponseValue<Wallet>, crate::generated::Error>> + 'a;


            /// Get a list of wallets
            #[must_use] pub fn get_wallets<'a>(&'a self, chain_type: Option<&'a GetWalletsChainType>, cursor: Option<&'a GetWalletsCursor>, limit: Option<f64>, user_id: Option<&'a str>) -> impl Future<Output = Result<ResponseValue<GetWalletsResponse>, crate::generated::Error>> + 'a;

            /// Create a new wallet
            #[must_use] pub fn create_wallet<'a>(&'a self, privy_idempotency_key: Option<&'a str>, body: &'a CreateWalletBody) -> impl Future<Output = Result<ResponseValue<Wallet>, crate::generated::Error>> + 'a;


            /// Create a new user
            #[must_use] pub fn create_user<'a>(&'a self, body: &'a CreateUserBody) -> impl Future<Output = Result<ResponseValue<User>, crate::generated::Error>> + 'a;

            // /// Get a user
            // #[must_use] pub fn get_user(&self) -> crate::generated::types::GetUser<'_>;

            // /// Get a list of users
            // #[must_use] pub fn get_users(&self) -> crate::generated::types::GetUsers<'_>;

            /// Delete a user
            #[must_use] pub fn delete_user<'a>(&'a self, user_id: &'a str) -> impl Future<Output = Result<ResponseValue<()>, crate::generated::Error<ByteStream>>> + 'a;

            // /// Search for users
            // #[must_use] pub fn search_users(&self) -> crate::generated::types::SearchUsers<'_>;

            // /// Create a new user wallet
            // #[must_use] pub fn create_user_wallet(&self) -> CreateUserWallet<'_>;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        IntoKey, PrivateKeyFromFile,
        generated::types::{OwnerInput, PublicKeyOwner},
    };

    #[tokio::test]
    async fn test_build_canonical_request() {
        let client = PrivyClient::new(
            "cmf418pa801bxl40b5rcgjvd9".into(),
            "app_secret".into(),
            AuthorizationContext::new(),
        )
        .unwrap();
        let wallet_id = "o5zuf7fbygwze9l9gaxyc0bm";

        let key = PrivateKeyFromFile("private_key.pem".into());
        let public_key = key.get_key().await.unwrap().public_key();

        // Create the request body that will be sent using the generated privy-api type
        let update_wallet_body = UpdateWalletBody {
            owner: Some(OwnerInput {
                subtype_0: Some(PublicKeyOwner {
                    public_key: public_key.to_string(),
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        // Build the canonical request data for signing using the serialized body
        let canonical_data = client
            .build_update_wallet_canonical_request(
                wallet_id,
                update_wallet_body.clone(),
                // Some(idempotency_key.clone()),
                None,
            )
            .unwrap();

        assert_eq!(
            canonical_data,
            "{\"body\":{\"owner\":{\"public_key\":\"-----BEGIN PUBLIC KEY-----\\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAESYrvEwooR33jt/8Up0lWdDNAcxmg\\nNZrCX23OThCPA+WxDx+dHYrjRlfPmHX0/aMTopp1PdKAtlQjRJDHSNd8XA==\\n-----END PUBLIC KEY-----\\n\"}},\"headers\":{\"privy-app-id\":\"cmf418pa801bxl40b5rcgjvd9\"},\"method\":\"PATCH\",\"url\":\"https://api.privy.io/v1/wallets/o5zuf7fbygwze9l9gaxyc0bm\",\"version\":1}"
        );
    }
}
