//! Privy SDK for Rust

#![deny(clippy::unwrap_used)]
// #![warn(clippy::pedantic)]
// #![warn(missing_docs)]

use base64::{Engine, engine::general_purpose::STANDARD};

pub mod client;
pub mod ethereum;
pub mod privy_hpke;
pub mod solana;

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

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize() {
        let body = r#"{"id":"kra9fvbs9vo47ge1xs5x97k5","name":"test-policy-1757686660","chain_type":"solana","rules":[{"name":"test-rule","method":"signTransaction","conditions":[{"field_source":"solana_system_program_instruction","field":"Transfer.lamports","operator":"lt","value":"1000000"}],"action":"ALLOW","id":"h7ugroh0uaotl5khnc8xftdo"}],"version":"1.0","created_at":1757686660877,"owner_id":null}"#;

        let jd = &mut serde_json::Deserializer::from_str(body);
        let result: Result<super::generated::types::Policy, _> =
            serde_path_to_error::deserialize(jd);

        println!("{:?}", result.unwrap());
    }
}
