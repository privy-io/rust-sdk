use serde::{Serialize, Deserialize};
use super::HpkeEncryption;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletImportInitResponse {
    ///The base64-encoded encryption public key to encrypt the wallet entropy with.
    pub encryption_public_key: String,
    ///The encryption type of the wallet to import. Currently only supports `HPKE`.
    pub encryption_type: HpkeEncryption,
}
impl std::fmt::Display for WalletImportInitResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
