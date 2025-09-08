use serde::{Serialize, Deserialize};
use super::Wallet;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WithoutEncryption {
    ///The raw authorization key data.
    pub authorization_key: String,
    ///The expiration time of the authorization key in seconds since the epoch.
    pub expires_at: f64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wallets: Vec<Wallet>,
}
impl std::fmt::Display for WithoutEncryption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
