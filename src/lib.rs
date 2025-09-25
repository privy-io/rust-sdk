//! Privy SDK for Rust

#![deny(clippy::unwrap_used)]
// #![warn(clippy::pedantic)]
#![warn(missing_docs)]

use base64::{Engine, engine::general_purpose::STANDARD};

pub mod client;
pub mod ethereum;
pub mod privy_hpke;
pub mod solana;

/// Generated types from privy's openapi spec
pub mod generated {
    pub use privy_openapi::*;
}

pub mod subclients;

pub(crate) mod errors;
pub(crate) mod jwt_exchange;
pub(crate) mod keys;
pub(crate) mod utils;

pub use client::PrivyClient;
pub use errors::*;
pub use keys::*;
pub use privy_hpke::PrivyHpke;
pub use utils::{
    Method, Utils, WalletApiRequestSignatureInput, format_request_for_authorization_signature,
    generate_authorization_signatures,
};

pub(crate) fn get_auth_header(app_id: &str, app_secret: &str) -> String {
    let credentials = format!("{app_id}:{app_secret}");
    format!("Basic {}", STANDARD.encode(credentials))
}
