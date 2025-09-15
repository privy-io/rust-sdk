#![allow(missing_docs)]

use thiserror::Error;

pub use crate::generated::{Error as PrivyApiError, types::error::ConversionError};

#[derive(Error, Debug)]
pub enum PrivyCreateError {
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Unable to create client: {0}")]
    Client(#[from] reqwest::Error),
}

#[derive(Error, Debug)]
pub enum PrivyError {
    #[error("Base64 decoding failed: {0}")]
    Base64(#[from] base64::DecodeError),

    #[error("Hex parsing failed: {0}")]
    HexParsing(#[from] std::num::ParseIntError),

    #[error("Invalid signature length: expected 64 bytes")]
    InvalidSignatureLength,

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Unable to convert fields: {0}")]
    Conversion(#[from] ConversionError),

    #[error("Error while accessing API: {0}")]
    Api(#[from] PrivyApiError),
    #[error("Unknown error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum KeyError {
    #[error("Invalid key")]
    Unknown,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid key format: {0}")]
    InvalidFormat(String),
    #[error("Base64 decoding failed: {0}")]
    Base64(#[from] base64::DecodeError),
    #[error("HPKE decryption failed: {0}")]
    HpkeDecryption(String),
    #[error("PKCS#8 parsing failed")]
    Pkcs8Parsing,
    #[error("Unsupported encryption type")]
    UnsupportedEncryption,
}

#[derive(Error, Debug)]
pub enum SigningError {
    #[error("Invalid key: {0}")]
    Key(#[from] KeyError),
    #[error("Invalid signature")]
    Unknown,
}
