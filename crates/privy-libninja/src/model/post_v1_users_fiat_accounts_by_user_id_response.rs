use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostV1UsersFiatAccountsByUserIdResponse {
    pub account_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    pub currency: String,
    pub id: String,
    #[serde(rename = "last_4")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
}
impl std::fmt::Display for PostV1UsersFiatAccountsByUserIdResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
