use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Transaction {
    pub caip2: String,
    pub created_at: f64,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sponsored: Option<bool>,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<String>,
    pub wallet_id: String,
}
impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
