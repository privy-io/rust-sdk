use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostV1UsersFiatKycLinkByUserIdResponse {
    pub created_at: String,
    pub customer_id: String,
    pub email: String,
    pub full_name: String,
    pub id: String,
    pub kyc_link: String,
    pub kyc_status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persona_inquiry_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rejection_reasons: Vec<serde_json::Value>,
    pub tos_link: String,
    pub tos_status: String,
}
impl std::fmt::Display for PostV1UsersFiatKycLinkByUserIdResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
