use solana_sdk::pubkey::ParsePubkeyError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PrivyError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Base64 decoding failed: {0}")]
    Base64(#[from] base64::DecodeError),

    #[error("Hex parsing failed: {0}")]
    HexParsing(#[from] std::num::ParseIntError),

    #[error("Solana pubkey parsing failed: {0}")]
    SolanaPubkey(#[from] ParsePubkeyError),

    #[error("Privy API error {status}: {message}")]
    Api { status: u16, message: String },

    #[error("Invalid signature length: expected 64 bytes")]
    InvalidSignatureLength,

    #[error("Configuration error: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, PrivyError>;
