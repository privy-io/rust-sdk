use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostV1UsersFiatStatusByUserIdResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transactions: Vec<serde_json::Value>,
}
impl std::fmt::Display for PostV1UsersFiatStatusByUserIdResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
