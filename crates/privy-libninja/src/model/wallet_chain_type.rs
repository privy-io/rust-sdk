use serde::{Serialize, Deserialize};
///Chain type of the wallet
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WalletChainType {
    #[serde(rename = "solana")]
    Solana,
    #[serde(rename = "ethereum")]
    Ethereum,
    #[serde(rename = "cosmos")]
    Cosmos,
    #[serde(rename = "stellar")]
    Stellar,
    #[serde(rename = "sui")]
    Sui,
    #[serde(rename = "tron")]
    Tron,
    #[serde(rename = "bitcoin-segwit")]
    BitcoinSegwit,
    #[serde(rename = "near")]
    Near,
    #[serde(rename = "spark")]
    Spark,
    #[serde(rename = "ton")]
    Ton,
    #[serde(rename = "starknet")]
    Starknet,
}
