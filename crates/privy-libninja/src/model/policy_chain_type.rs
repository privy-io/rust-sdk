use serde::{Serialize, Deserialize};
///The chain type the policy applies to.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PolicyChainType {
    #[serde(rename = "ethereum")]
    Ethereum,
    #[serde(rename = "solana")]
    Solana,
}
