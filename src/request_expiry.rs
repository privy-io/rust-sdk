//! Request expiry for signed requests.
//!
//! The Privy API accepts a `privy-request-expiry` header on signed requests: an
//! absolute Unix timestamp in milliseconds after which the API rejects the request.
//! The SDK computes this value for every signed request (see [`RequestExpiryOptions`])
//! and includes the same value in the authorization signature.
//!
//! ```rust
//! use std::time::Duration;
//!
//! use privy_rs::{PrivyClient, client::PrivyClientOptions, request_expiry::RequestExpiryOptions};
//!
//! # fn main() -> Result<(), privy_rs::PrivyCreateError> {
//! let client = PrivyClient::new_with_options(
//!     "app_id".to_string(),
//!     "app_secret".to_string(),
//!     PrivyClientOptions {
//!         request_expiry: RequestExpiryOptions::new()
//!             .with_default_expiry(Duration::from_secs(5 * 60)),
//!         ..Default::default()
//!     },
//! )?;
//! # Ok(())
//! # }
//! ```

use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Default request expiry for signed requests: 15 minutes after the request is made.
pub const DEFAULT_REQUEST_EXPIRY: Duration = Duration::from_secs(15 * 60);

/// Default request expiry for intent requests: 72 hours after the request is made.
///
/// The SDK does not expose intents endpoints yet, so no request uses this value today.
pub const DEFAULT_INTENT_REQUEST_EXPIRY: Duration = Duration::from_secs(72 * 60 * 60);

/// Returns the Unix timestamp in milliseconds `delta` from now.
///
/// Use this to compute an absolute per-request expiry, for example
/// `request_expiry_from_now(Duration::from_secs(60))` for one minute from now.
#[must_use]
pub fn request_expiry_from_now(delta: Duration) -> u64 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    u64::try_from(now.saturating_add(delta).as_millis()).unwrap_or(u64::MAX)
}

/// Client-level request expiry settings.
///
/// By default every signed request expires [`DEFAULT_REQUEST_EXPIRY`] after it is
/// made. The expiry is computed when each request is sent, not when the client is
/// created.
///
/// ```rust
/// use std::time::Duration;
///
/// use privy_rs::request_expiry::RequestExpiryOptions;
///
/// // Expire signed requests 5 minutes after they are made.
/// let custom = RequestExpiryOptions::new().with_default_expiry(Duration::from_secs(5 * 60));
///
/// // Do not send a request expiry unless one is set on the request itself.
/// let disabled = RequestExpiryOptions::new().with_disabled(true);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct RequestExpiryOptions {
    /// When `true`, the SDK does not add a default request expiry. An expiry set
    /// explicitly on a request is still sent.
    pub disabled: bool,
    /// How long after a request is made it expires, when the request does not set
    /// its own expiry. Defaults to [`DEFAULT_REQUEST_EXPIRY`].
    pub default_expiry: Duration,
}

impl Default for RequestExpiryOptions {
    fn default() -> Self {
        Self {
            disabled: false,
            default_expiry: DEFAULT_REQUEST_EXPIRY,
        }
    }
}

impl RequestExpiryOptions {
    /// Creates `RequestExpiryOptions` with the defaults.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether the default request expiry is disabled.
    #[must_use]
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets how long after a request is made it expires by default.
    #[must_use]
    pub fn with_default_expiry(mut self, default_expiry: Duration) -> Self {
        self.default_expiry = default_expiry;
        self
    }

    /// Resolves the expiry (Unix ms) to send for a single request.
    ///
    /// Precedence: per-request value, then `disabled` (no expiry), then the
    /// configured default computed from the current time.
    pub(crate) fn resolve(&self, per_request: Option<u64>) -> Option<u64> {
        if per_request.is_some() {
            return per_request;
        }
        if self.disabled {
            return None;
        }
        Some(request_expiry_from_now(self.default_expiry))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn now_ms() -> u64 {
        request_expiry_from_now(Duration::ZERO)
    }

    #[test]
    fn test_default_constants() {
        assert_eq!(DEFAULT_REQUEST_EXPIRY, Duration::from_millis(900_000));
        assert_eq!(
            DEFAULT_INTENT_REQUEST_EXPIRY,
            Duration::from_millis(259_200_000)
        );
    }

    #[test]
    fn test_request_expiry_from_now() {
        let before = now_ms();
        let expiry = request_expiry_from_now(Duration::from_secs(60));
        let after = now_ms();
        assert!(expiry >= before + 60_000);
        assert!(expiry <= after + 60_000);
    }

    #[test]
    fn test_request_expiry_from_now_saturates() {
        assert_eq!(request_expiry_from_now(Duration::MAX), u64::MAX);
    }

    #[test]
    fn test_resolve_uses_default() {
        let before = now_ms();
        let expiry = RequestExpiryOptions::new().resolve(None).unwrap();
        let after = now_ms();
        assert!(expiry >= before + 900_000);
        assert!(expiry <= after + 900_000);
    }

    #[test]
    fn test_resolve_uses_custom_default() {
        let options = RequestExpiryOptions::new().with_default_expiry(Duration::from_secs(30));
        let before = now_ms();
        let expiry = options.resolve(None).unwrap();
        let after = now_ms();
        assert!(expiry >= before + 30_000);
        assert!(expiry <= after + 30_000);
    }

    #[test]
    fn test_resolve_disabled() {
        let options = RequestExpiryOptions::new().with_disabled(true);
        assert_eq!(options.resolve(None), None);
    }

    #[test]
    fn test_resolve_per_request_wins() {
        let enabled = RequestExpiryOptions::new();
        let disabled = RequestExpiryOptions::new().with_disabled(true);
        assert_eq!(enabled.resolve(Some(1_234)), Some(1_234));
        assert_eq!(disabled.resolve(Some(1_234)), Some(1_234));
        assert_eq!(enabled.resolve(Some(0)), Some(0));
        assert_eq!(disabled.resolve(Some(0)), Some(0));
    }

    #[test]
    fn test_resolve_computes_now_per_call() {
        let options = RequestExpiryOptions::new();
        let first = options.resolve(None).unwrap();
        std::thread::sleep(Duration::from_millis(5));
        let second = options.resolve(None).unwrap();
        assert!(second > first);
    }
}
