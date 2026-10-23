//! Privy SDK for Rust

#![deny(clippy::unwrap_used)]
// #![warn(clippy::pedantic)]
#![warn(missing_docs)]

use base64::{Engine, engine::general_purpose::STANDARD};

pub mod client;
pub mod ethereum;
pub mod privy_hpke;
pub mod request_expiry;
pub mod solana;

/// Generated types from privy's openapi spec
pub mod generated {
    pub use privy_openapi::*;
}

pub mod subclients;

#[cfg(feature = "alloy")]
pub mod alloy;

pub(crate) mod errors;
pub(crate) mod import;
pub(crate) mod jwt_exchange;
pub(crate) mod keys;
pub(crate) mod request_options;
pub(crate) mod utils;

pub use client::PrivyClient;
pub use errors::*;
pub use ethereum::SendTransactionOptions;
pub use keys::*;
pub use privy_hpke::PrivyHpke;
pub use request_expiry::{RequestExpiryOptions, request_expiry_from_now};
pub use request_options::RequestOptions;
pub use solana::SignAndSendTransactionOptions;
pub use utils::{
    AuthorizationSignatureHeaders, Method, Utils, WalletApiRequestSignatureInput,
    format_request_for_authorization_signature,
    format_request_for_authorization_signature_with_headers, generate_authorization_signatures,
    generate_authorization_signatures_with_headers,
};

#[cfg(feature = "alloy")]
pub use alloy::PrivyAlloyWallet;

pub(crate) fn get_auth_header(app_id: &str, app_secret: &str) -> String {
    let credentials = format!("{app_id}:{app_secret}");
    format!("Basic {}", STANDARD.encode(credentials))
}
