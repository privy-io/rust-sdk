use serde::{Serialize, Deserialize};
use super::Wallet;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetWalletsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<Wallet>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}
impl std::fmt::Display for GetWalletsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
