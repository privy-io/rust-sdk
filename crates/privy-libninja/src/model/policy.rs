use serde::{Serialize, Deserialize};
use super::PolicyChainType;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    ///The chain type the policy applies to.
    pub chain_type: PolicyChainType,
    ///Unix timestamp of when the policy was created in milliseconds.
    pub created_at: f64,
    ///Unique ID of the created policy. This will be the primary identifier when using the policy in the future.
    pub id: String,
    ///Name to assign to policy.
    pub name: String,
    ///The key quorum ID of the owner of the policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<serde_json::Value>,
    ///Version of the policy. Currently, 1.0 is the only version.
    pub version: String,
}
impl std::fmt::Display for Policy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
