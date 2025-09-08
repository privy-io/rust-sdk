use serde::{Serialize, Deserialize};
///Additional signers for the wallet.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletAdditionalSigner {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub override_policy_ids: Vec<String>,
    pub signer_id: String,
}
impl std::fmt::Display for WalletAdditionalSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
