#![allow(missing_docs)]

use thiserror::Error;

pub use crate::generated::{Error as PrivyApiError, types::error::ConversionError};

/// Errors that can occur during `PrivyClient` initialization.
#[derive(Error, Debug)]
pub enum PrivyCreateError {
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Unable to create client: {0}")]
    Client(#[from] reqwest::Error),
    #[error("Invalid app id")]
    InvalidAppId,
    #[error("Invalid app secret")]
    InvalidAppSecret,
}

/// The primary error type for the Privy SDK.
///
/// This enum consolidates all possible failures that can occur during client setup,
/// API interaction, or cryptographic operations into a single, easy-to-handle type.
#[derive(Error, Debug)]
pub enum PrivySignedApiError {
    /// An error returned by the Privy API (e.g., 4xx or 5xx HTTP status codes).
    /// Contains the raw response for further inspection.
    #[error("API request failed")]
    Api(#[from] PrivyApiError),

    /// An error occurred during the signing process.
    #[error("Signature generation failed: {0}")]
    SignatureGeneration(#[from] SignatureGenerationError),
}

/// Errors that can appear during wallet export.
#[derive(Error, Debug)]
pub enum PrivyExportError {
    /// An error returned by the Privy API (e.g., 4xx or 5xx HTTP status codes).
    /// Contains the raw response for further inspection.
    #[error("API request failed")]
    Api(#[from] PrivyApiError),

    /// An error occurred during the signing process.
    #[error("Signature generation failed: {0}")]
    SignatureGeneration(#[from] SignatureGenerationError),

    /// An error occurred during the decryption process.
    #[error("Unable to decrypt key: {0}")]
    Key(#[from] KeyError),
}

/// Errors related to cryptographic keys and operations.
#[derive(Error, Debug)]
pub enum CryptoError {
    /// A failure occurred during the signing process.
    #[error("Signing failed: {0}")]
    Signing(#[from] SigningError),

    /// A failure occurred while parsing, loading, or exchanging a key.
    #[error("Key handling failed: {0}")]
    Key(#[from] KeyError),
}

/// Errors related to handling cryptographic keys.
#[derive(Error, Debug)]
pub enum KeyError {
    /// Failed to read a key from a file or other I/O source.
    #[error("Key I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The key data is malformed (e.g., invalid PEM, DER, or Base64 format).
    #[error("Invalid key format: {0}")]
    InvalidFormat(String),

    /// Failed to decrypt an HPKE-encrypted payload.
    #[error("HPKE decryption failed: {0}")]
    HpkeDecryption(#[from] hpke::HpkeError),

    /// An unknown error occurred.
    #[error(transparent)]
    Other(Box<dyn std::error::Error + Send + Sync>),
}

/// Errors that occur specifically during a digital signature operation.
#[derive(Error, Debug)]
pub enum SigningError {
    /// The key required for signing could not be obtained or is invalid.
    #[error("Invalid key for signing: {0}")]
    Key(#[from] KeyError),

    /// The underlying cryptographic library failed to produce a signature.
    #[error("Signature creation failed: {0}")]
    Signature(#[from] p256::ecdsa::Error),

    /// An unknown error occurred.
    #[error(transparent)]
    Other(Box<dyn std::error::Error + Send + Sync>),
}

/// Errors from the authorization signature generation process. This can
/// very rarely occur from serialization (either the request could not
/// be serialized or the serialized data can not be converted to base64),
/// or (more likely) there was an error when undergoing the signing process.
#[derive(Debug, Error)]
pub enum SignatureGenerationError {
    #[error("Unable to serialize request for signing: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Error when signing request: {0}")]
    Signing(#[from] SigningError),
}

/// Error codes the Privy API documents; each variant's doc is Privy's description.
/// <https://docs.privy.io/basics/troubleshooting/error-handling/api-errors>
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PrivyApiErrorCode {
    /// `policy_violation` — RPC request denied due to policy violation
    PolicyViolation,
    /// `insufficient_funds` — Wallet has insufficient funds to complete the transaction
    InsufficientFunds,
    /// `transaction_broadcast_failure` — Transaction failed to broadcast to the network
    TransactionBroadcastFailure,
    /// `missing_or_empty_authorization_header` — Missing `privy-authorization-signature` header or no signatures provided
    MissingOrEmptyAuthorizationHeader,
    /// `zero_correct_authorization_signatures` — No valid authorization signatures were provided
    ZeroCorrectAuthorizationSignatures,
    /// `insufficient_correct_authorization_signatures` — Not enough valid authorization signatures provided
    InsufficientCorrectAuthorizationSignatures,
    /// `incorrect_quantity_of_authorization_signatures` — Number of signatures does not match the wallet's authorization threshold
    IncorrectQuantityOfAuthorizationSignatures,
    /// `request_expired` — The request has expired based on the `privy-request-expiry` header
    RequestExpired,
    /// `no_valid_user_session_keys` — No valid user signing keys available
    NoValidUserSessionKeys,
    /// `user_session_keys_expired` — User signing key is expired
    UserSessionKeysExpired,
    /// `insufficient_app_secret_permissions` — The app secret does not have permission to make this request (HTTP 403)
    InsufficientAppSecretPermissions,
    /// A code this SDK version does not list; holds the raw `code` string.
    Unknown(String),
}

impl PrivyApiErrorCode {
    /// The `code` string as the API sends it (snake_case).
    pub fn as_str(&self) -> &str {
        match self {
            Self::PolicyViolation => "policy_violation",
            Self::InsufficientFunds => "insufficient_funds",
            Self::TransactionBroadcastFailure => "transaction_broadcast_failure",
            Self::MissingOrEmptyAuthorizationHeader => "missing_or_empty_authorization_header",
            Self::ZeroCorrectAuthorizationSignatures => "zero_correct_authorization_signatures",
            Self::InsufficientCorrectAuthorizationSignatures => {
                "insufficient_correct_authorization_signatures"
            }
            Self::IncorrectQuantityOfAuthorizationSignatures => {
                "incorrect_quantity_of_authorization_signatures"
            }
            Self::RequestExpired => "request_expired",
            Self::NoValidUserSessionKeys => "no_valid_user_session_keys",
            Self::UserSessionKeysExpired => "user_session_keys_expired",
            Self::InsufficientAppSecretPermissions => "insufficient_app_secret_permissions",
            Self::Unknown(code) => code,
        }
    }
}

impl From<&str> for PrivyApiErrorCode {
    fn from(code: &str) -> Self {
        match code {
            "policy_violation" => Self::PolicyViolation,
            "insufficient_funds" => Self::InsufficientFunds,
            "transaction_broadcast_failure" => Self::TransactionBroadcastFailure,
            "missing_or_empty_authorization_header" => Self::MissingOrEmptyAuthorizationHeader,
            "zero_correct_authorization_signatures" => Self::ZeroCorrectAuthorizationSignatures,
            "insufficient_correct_authorization_signatures" => {
                Self::InsufficientCorrectAuthorizationSignatures
            }
            "incorrect_quantity_of_authorization_signatures" => {
                Self::IncorrectQuantityOfAuthorizationSignatures
            }
            "request_expired" => Self::RequestExpired,
            "no_valid_user_session_keys" => Self::NoValidUserSessionKeys,
            "user_session_keys_expired" => Self::UserSessionKeysExpired,
            "insufficient_app_secret_permissions" => Self::InsufficientAppSecretPermissions,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl std::fmt::Display for PrivyApiErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// JSON body of a Privy API error as the API returns it: `error` (message), `code`
/// (see [`PrivyApiErrorCode`]) and an optional code-specific `data` object. The codes are
/// documented at <https://docs.privy.io/basics/troubleshooting/error-handling/api-errors>;
/// the body layout itself is observed, not part of that reference.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct PrivyApiErrorBody {
    /// Human-readable message.
    #[serde(default)]
    pub error: Option<String>,
    /// Machine-readable code; parsed by [`PrivyApiErrorBody::code`].
    #[serde(default)]
    pub code: Option<String>,
    /// Optional code-specific payload; the reference does not document its shape. Decode with [`PrivyApiErrorBody::data_as`].
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}

impl PrivyApiErrorBody {
    /// Parsed `code`; `None` when the body carries none.
    pub fn code(&self) -> Option<PrivyApiErrorCode> {
        self.code.as_deref().map(PrivyApiErrorCode::from)
    }

    /// Deserializes `data` into the caller's type; `None` when `data` is absent.
    pub fn data_as<T: serde::de::DeserializeOwned>(&self) -> Option<Result<T, serde_json::Error>> {
        self.data.as_ref().map(T::deserialize)
    }
}

/// Why a non-2xx body could not be turned into [`PrivyApiErrorBody`]; both keep their cause.
#[derive(Debug, thiserror::Error)]
pub enum PrivyApiErrorBodyError {
    /// The response body could not be read.
    #[error("failed to read error response body")]
    Read(#[source] reqwest::Error),
    /// The response body could not be decoded as a Privy error body.
    #[error("error response body could not be decoded as a Privy error body: {raw}")]
    Parse {
        /// The body as received (lossy UTF-8), for diagnostics.
        raw: String,
        /// The JSON decoding error.
        #[source]
        source: serde_json::Error,
    },
}

/// A non-2xx Privy API response with its body read.
#[derive(Debug)]
pub struct PrivyApiErrorResponse {
    /// HTTP status of the response.
    pub status: reqwest::StatusCode,
    /// Parsed body, or why it could not be parsed.
    pub body: Result<PrivyApiErrorBody, PrivyApiErrorBodyError>,
}

impl std::fmt::Display for PrivyApiErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.body {
            Ok(body) => write!(
                f,
                "Privy API error {} {}: {}",
                self.status,
                body.code.as_deref().unwrap_or("-"),
                body.error.as_deref().unwrap_or("-"),
            ),
            Err(e) => write!(f, "Privy API error {}: {e}", self.status),
        }
    }
}

impl std::error::Error for PrivyApiErrorResponse {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.body
            .as_ref()
            .err()
            .map(|e| e as &(dyn std::error::Error + 'static))
    }
}

impl PrivyApiErrorResponse {
    /// Reads the body of [`PrivyApiError::UnexpectedResponse`]. Consumes the error because a
    /// `reqwest::Response` body can be read once. Infallible for `UnexpectedResponse`; every other
    /// variant is handed back unchanged as `Err`.
    pub async fn from_api_error(error: PrivyApiError) -> Result<Self, PrivyApiError> {
        let PrivyApiError::UnexpectedResponse(response) = error else {
            return Err(error);
        };
        let status = response.status();
        let body = match response.bytes().await {
            Err(e) => Err(PrivyApiErrorBodyError::Read(e)),
            Ok(bytes) => serde_json::from_slice::<PrivyApiErrorBody>(&bytes).map_err(|source| {
                PrivyApiErrorBodyError::Parse {
                    raw: String::from_utf8_lossy(&bytes).into_owned(),
                    source,
                }
            }),
        };
        Ok(Self { status, body })
    }
}
