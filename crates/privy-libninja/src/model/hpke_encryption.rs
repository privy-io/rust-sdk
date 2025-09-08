use serde::{Serialize, Deserialize};
///The encryption type of the wallet to import. Currently only supports `HPKE`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum HpkeEncryption {
    #[serde(rename = "HPKE")]
    Hpke,
}
