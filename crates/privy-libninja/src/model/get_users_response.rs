use serde::{Serialize, Deserialize};
use super::User;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetUsersResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<User>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}
impl std::fmt::Display for GetUsersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
