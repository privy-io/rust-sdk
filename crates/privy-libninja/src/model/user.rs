use serde::{Serialize, Deserialize};
use super::CustomMetadata;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct User {
    ///Unix timestamp of when the user was created in milliseconds.
    pub created_at: f64,
    ///Custom metadata associated with the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<CustomMetadata>,
    ///Indicates if the user has accepted the terms of service.
    pub has_accepted_terms: bool,
    pub id: String,
    ///Indicates if the user is a guest account user.
    pub is_guest: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linked_accounts: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mfa_methods: Vec<serde_json::Value>,
}
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
