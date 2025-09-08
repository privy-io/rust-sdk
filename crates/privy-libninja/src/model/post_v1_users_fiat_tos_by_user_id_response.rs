use serde::{Serialize, Deserialize};
use super::{Variant0, Variant1};
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum PostV1UsersFiatTosByUserIdResponse {
    Variant0(Variant0),
    Variant1(Variant1),
}
