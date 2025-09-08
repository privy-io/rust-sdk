use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetWalletBalanceResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub balances: Vec<serde_json::Value>,
}
impl std::fmt::Display for GetWalletBalanceResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
