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
