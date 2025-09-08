use serde::{Serialize, Deserialize};
///The P-256 public key of the owner of the resource. If you provide this, do not specify an owner_id as it will be generated automatically.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PublicKeyOwner {
    pub public_key: String,
}
impl std::fmt::Display for PublicKeyOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
