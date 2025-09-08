use serde::{Serialize, Deserialize};
use super::HpkeEncryption;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletExportResponse {
    ///The encrypted private key.
    pub ciphertext: String,
    ///The base64-encoded encapsulated key that was generated during encryption, for use during decryption.
    pub encapsulated_key: String,
    ///The encryption type of the wallet to import. Currently only supports `HPKE`.
    pub encryption_type: HpkeEncryption,
}
impl std::fmt::Display for WalletExportResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
