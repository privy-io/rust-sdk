use serde::{Serialize, Deserialize};
use super::{WithEncryption, WithoutEncryption};
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum PostV1UserSignersAuthenticateResponse {
    #[serde(rename = "With encryption")]
    WithEncryption(WithEncryption),
    #[serde(rename = "Without encryption")]
    WithoutEncryption(WithoutEncryption),
}
