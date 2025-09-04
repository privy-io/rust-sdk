//! Privy SDK for Rust

#![deny(clippy::unwrap_used)]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]

use base64::{Engine, engine::general_purpose::STANDARD};

pub mod client;

pub(crate) mod errors;
pub(crate) mod keys;
pub(crate) mod types;

pub use client::PrivyClient;
pub use errors::*;
pub use keys::*;
pub use types::*;

pub(crate) fn get_auth_header(app_id: &str, app_secret: &str) -> String {
    let credentials = format!("{app_id}:{app_secret}");
    format!("Basic {}", STANDARD.encode(credentials))
}
