use serde::{Serialize, Deserialize};
use super::{WalletAdditionalSigner, WalletChainType};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_signers: Vec<WalletAdditionalSigner>,
    ///Address of the wallet.
    pub address: String,
    ///Chain type of the wallet
    pub chain_type: WalletChainType,
    ///Unix timestamp of when the wallet was created in milliseconds.
    pub created_at: f64,
    ///Unix timestamp of when the wallet was exported in milliseconds, if the wallet was exported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exported_at: Option<f64>,
    ///Unique ID of the wallet. This will be the primary identifier when using the wallet in the future.
    pub id: String,
    ///Unix timestamp of when the wallet was imported in milliseconds, if the wallet was imported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_at: Option<f64>,
    ///The key quorum ID of the owner of the wallet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    ///List of policy IDs for policies that are enforced on the wallet.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_ids: Vec<String>,
    ///The compressed, raw public key for the wallet along the chain cryptographic curve.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}
impl std::fmt::Display for Wallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
