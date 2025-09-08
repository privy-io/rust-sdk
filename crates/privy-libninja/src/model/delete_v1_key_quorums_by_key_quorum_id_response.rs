use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteV1KeyQuorumsByKeyQuorumIdResponse {
    ///Whether the key quorum was deleted successfully.
    pub success: bool,
}
impl std::fmt::Display for DeleteV1KeyQuorumsByKeyQuorumIdResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
