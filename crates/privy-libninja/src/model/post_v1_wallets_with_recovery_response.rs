use serde::{Serialize, Deserialize};
use super::Wallet;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostV1WalletsWithRecoveryResponse {
    ///The ID of the created user.
    pub recovery_user_id: String,
    ///The wallets that were created.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wallets: Vec<Wallet>,
}
impl std::fmt::Display for PostV1WalletsWithRecoveryResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
