//! Privy client implementations.
//!
//! This module contains the `PrivyClient` with typed wallet support.

use std::time::Duration;

use reqwest::header::{CONTENT_TYPE, HeaderValue};

use crate::{PrivyCreateError, generated::Client, get_auth_header};

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
    pub(crate) base_url: String,
    pub(crate) client: Client,
}

impl PrivyClient {
    /// Create a new `PrivyClient`
    ///
    /// # Usage
    /// ```no_run
    /// # use privy_rust::{PrivyCreateError, PrivateKeyFromFile, AuthorizationContext};
    /// # async fn foo() -> Result<(), PrivyCreateError> {
    /// let ctx = AuthorizationContext::new();
    /// ctx.push(PrivateKeyFromFile("private_key.pem".into()));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// This can fail for two reasons, either the `app_id` or `app_secret` are not
    /// valid headers, or that the underlying http client could not be created.
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
            app_id: app_id.clone(),
            app_secret,
            base_url: url.to_string(),
            client: Client::new_with_client(url, client_with_custom_defaults),
        })
    }

    pub fn utils(&self) -> crate::utils::Utils {
        crate::utils::Utils(self.app_id.clone())
    }
}
