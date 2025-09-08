use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostV1UsersFiatOnrampByUserIdResponse {
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub deposit_instructions: serde_json::Value,
    pub id: String,
    pub status: String,
}
impl std::fmt::Display for PostV1UsersFiatOnrampByUserIdResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
