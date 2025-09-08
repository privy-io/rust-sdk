//! Privy client implementations.
//!
//! This module contains the `PrivyClient` with typed wallet support.

use std::{fmt, sync::Arc, time::Duration};

use privy_libninja::{
    FluentRequest, PrivyLibninjaAuth, PrivyLibninjaClient, default_http_client,
    request::{AuthenticateRequest, GetWalletRequest, UpdateWalletRequest},
};
use reqwest::header::{CONTENT_TYPE, HeaderValue};
use serde::Serialize;

use crate::{Method, PrivyCreateError, WalletApiRequestSignatureInput, get_auth_header};

/// Privy client for interacting with the Privy API.
///
/// This provides access to global operations like user and wallet management.
/// For wallet-specific operations, use `TypedWallet<T>` instances created via
/// the `wallet()` method.
#[derive(Clone)]
pub struct PrivyClient {
    pub(crate) app_id: String,
    #[allow(dead_code)]
    pub(crate) app_secret: String,
    pub(crate) client: Arc<PrivyLibninjaClient>,
}

impl fmt::Debug for PrivyClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        #[derive(Debug)]
        struct PrivyClient<'a> {
            pub(crate) app_id: &'a str,
            pub(crate) app_secret: &'a str,
        }

        fmt::Debug::fmt(
            &PrivyClient {
                app_id: &self.app_id,
                app_secret: &self.app_secret,
            },
            f,
        )
    }
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
    /// NOTE: libninja keeps a static global client, so subsequent
    ///       calls to new will return the same client (and middleware)
    ///
    /// # Errors
    /// This can fail for two reasons, either the `app_id` or `app_secret` are not
    /// valid headers, or that the underlying http client could not be created.
    pub fn new_with_url(
        app_id: String,
        app_secret: String,
        url: &str,
    ) -> Result<Self, PrivyCreateError> {
        let client =
            default_http_client().base_url(url).default_headers(
                [
                    (
                        "Authorization",
                        get_auth_header(&app_id, &app_secret).as_str(),
                    ),
                    ("Privy-App-Id", &app_id),
                    ("Privy-Client", "rust-sdk"),
                    ("Content-Type", "application/json"),
                ]
                .into_iter(),
            )
            //.middleware()
            ;

        let auth = PrivyLibninjaAuth::AppId {
            privy_app_id: app_id.to_string(),
        };

        Ok(Self {
            app_id,
            app_secret,
            client: Arc::new(PrivyLibninjaClient::new(client, auth)),
        })
    }

    pub fn authenticate(&self, jwt: &str) -> FluentRequest<AuthenticateRequest> {
        self.client.authenticate(&self.app_id, jwt)
    }

    pub fn update_wallet(&self, wallet_id: &str) -> FluentRequest<UpdateWalletRequest> {
        self.client.update_wallet(&self.app_id, wallet_id)
    }

    pub fn get_wallet(&self, wallet_id: &str) -> FluentRequest<GetWalletRequest> {
        self.client.get_wallet(&self.app_id, wallet_id)
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

    use privy_libninja::model::{OwnerInput, PublicKeyOwner};

    use crate::{IntoKey, PrivateKeyFromFile};

    use super::*;

    #[tokio::test]
    async fn test_build_canonical_request() {
        let client =
            PrivyClient::new("cmf418pa801bxl40b5rcgjvd9".into(), "app_secret".into()).unwrap();
        let wallet_id = "o5zuf7fbygwze9l9gaxyc0bm";

        let key = PrivateKeyFromFile("private_key.pem".into());
        let public_key = key.get_key().await.unwrap().public_key();

        let request = client.update_wallet(&wallet_id).owner(
            OwnerInput::PublicKeyOwner(PublicKeyOwner {
                public_key: public_key.to_string(),
            }),
            // OwnerInput::UserOwner(UserOwner {
            //     user_id: "did:privy:cmf5wqe2l0005k10blt7x5dq2".to_string(),
            // }),
        );

        // Build the canonical request data for signing using the serialized body
        let canonical_data = client
            .build_update_wallet_canonical_request(
                wallet_id,
                &request.params,
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
