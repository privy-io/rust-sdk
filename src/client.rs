//! Privy client implementations.
//!
//! This module contains the `PrivyClient` with typed wallet support.

use std::{num::NonZeroUsize, time::Duration};

use reqwest::header::{CONTENT_TYPE, HeaderValue};

use crate::{PrivyCreateError, generated::Client, get_auth_header, jwt_exchange::JwtExchange};

/// Privy client for interacting with the Privy API.
///
/// This provides access to global operations like user and wallet management.
/// For wallet-specific operations, use `TypedWallet<T>` instances created via
/// the `wallet()` method.
///
/// # Errors
///
/// The api calls that require a signature to run will return a `PrivySignedApiError`
/// while the others will return a normal `PrivyApiError`.
#[derive(Clone, Debug)]
pub struct PrivyClient {
    pub(crate) app_id: String,
    #[allow(dead_code)]
    pub(crate) app_secret: String,
    pub(crate) base_url: String,
    pub(crate) client: Client,

    /// A store of all jwt operations for this client
    pub jwt_exchange: JwtExchange,
}

/// Options for configuring a `PrivyClient`
pub struct PrivyClientOptions {
    /// The maximum number of cached JWT secret keys to store
    pub cache_size: NonZeroUsize,
    /// The base url to use when making requests
    pub base_url: String,
}

impl Default for PrivyClientOptions {
    fn default() -> Self {
        Self {
            cache_size: NonZeroUsize::new(1000).expect("non-zero"),
            base_url: String::from("https://api.privy.com"),
        }
    }
}

impl PrivyClient {
    /// Create a new `PrivyClient`
    ///
    /// # Usage
    /// ```no_run
    /// # use privy_rs::{PrivyCreateError, PrivateKey, AuthorizationContext};
    /// # async fn foo() -> Result<(), PrivyCreateError> {
    /// # let my_key = include_str!("../tests/test_private_key.pem").to_string();
    /// let ctx = AuthorizationContext::new();
    /// ctx.push(PrivateKey(my_key));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// This can fail for two reasons, either the `app_id` or `app_secret` are not
    /// valid headers, or that the underlying http client could not be created.
    pub fn new(app_id: String, app_secret: String) -> Result<Self, PrivyCreateError> {
        Self::new_with_options(app_id, app_secret, PrivyClientOptions::default())
    }

    /// Create a new `PrivyClient` with a custom url
    ///
    /// # Errors
    /// This can fail for two reasons, either the `app_id` or `app_secret` are not
    /// valid headers, or that the underlying http client could not be created.
    pub fn new_with_options(
        app_id: String,
        app_secret: String,
        options: PrivyClientOptions,
    ) -> Result<Self, PrivyCreateError> {
        let client_version = concat!("rust:", env!("CARGO_PKG_VERSION"));

        tracing::debug!("Privy client version: {}", client_version);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&get_auth_header(&app_id, &app_secret))?,
        );
        headers.insert("privy-app-id", HeaderValue::from_str(&app_id)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("privy-client", HeaderValue::from_static(client_version));

        tracing::info!("Privy client headers: {:?}", headers);

        let client_with_custom_defaults = reqwest::ClientBuilder::new()
            .connect_timeout(Duration::from_secs(15))
            .timeout(Duration::from_secs(15))
            .default_headers(headers)
            .build()?;

        Ok(Self {
            app_id,
            app_secret,
            client: Client::new_with_client(&options.base_url, client_with_custom_defaults),
            base_url: options.base_url,
            jwt_exchange: JwtExchange::new(options.cache_size),
        })
    }

    /// Returns a new [`Utils`] instance
    #[must_use]
    pub fn utils(&self) -> crate::utils::Utils {
        crate::utils::Utils(self.app_id.clone())
    }
}
