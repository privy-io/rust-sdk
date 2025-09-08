use serde::{Serialize, Deserialize};
use super::PolicyRule;
///A rule that defines the conditions and action to take if the conditions are true.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuleResponse {
    ///The rules that apply to each method the policy covers.
    #[serde(flatten)]
    pub policy_rule: PolicyRule,
    pub id: String,
}
impl std::fmt::Display for RuleResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for RuleResponse {
    type Target = PolicyRule;
    fn deref(&self) -> &Self::Target {
        &self.policy_rule
    }
}
impl std::ops::DerefMut for RuleResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_rule
    }
}
