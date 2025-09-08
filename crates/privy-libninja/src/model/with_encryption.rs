use serde::{Serialize, Deserialize};
use super::Wallet;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WithEncryption {
    ///The encrypted authorization key data.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub encrypted_authorization_key: serde_json::Value,
    ///The expiration time of the authorization key in seconds since the epoch.
    pub expires_at: f64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wallets: Vec<Wallet>,
}
impl std::fmt::Display for WithEncryption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
