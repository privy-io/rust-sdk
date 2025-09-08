use serde::{Serialize, Deserialize};
///The user ID of the owner of the resource. The user must already exist, and this value must start with "did:privy:". If you provide this, do not specify an owner_id as it will be generated automatically.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserOwner {
    pub user_id: String,
}
impl std::fmt::Display for UserOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
