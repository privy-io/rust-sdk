use serde::{Serialize, Deserialize};
///The rules that apply to each method the policy covers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyRule {
    ///Action to take if the conditions are true.
    pub action: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<serde_json::Value>,
    ///Method the rule applies to.
    pub method: String,
    pub name: String,
}
impl std::fmt::Display for PolicyRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
