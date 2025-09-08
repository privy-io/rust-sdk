use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetV1UsersFiatAccountsByUserIdResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<serde_json::Value>,
}
impl std::fmt::Display for GetV1UsersFiatAccountsByUserIdResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
