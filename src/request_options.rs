//! Per-request options for signed requests.

/// Per-request options for signed requests.
///
/// Pass these to the `*_with_options` methods, for example
/// [`WalletsClient::rpc_with_options`](crate::subclients::WalletsClient::rpc_with_options).
///
/// ```rust
/// use std::time::Duration;
///
/// use privy_rs::{RequestOptions, request_expiry_from_now};
///
/// // Expire this request one minute from now.
/// let options =
///     RequestOptions::new().with_request_expiry(request_expiry_from_now(Duration::from_secs(60)));
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct RequestOptions {
    /// Absolute expiry for this request, as a Unix timestamp in milliseconds.
    ///
    /// Sent as-is in the `privy-request-expiry` header and included in the
    /// authorization signature. It takes precedence over the client's default,
    /// and is sent even when request expiry is disabled on the client. Use
    /// [`request_expiry_from_now`](crate::request_expiry_from_now) to compute one.
    pub request_expiry: Option<u64>,
}

impl RequestOptions {
    /// Creates `RequestOptions` with no overrides.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the absolute request expiry (Unix timestamp in milliseconds).
    #[must_use]
    pub fn with_request_expiry(mut self, request_expiry: u64) -> Self {
        self.request_expiry = Some(request_expiry);
        self
    }
}
