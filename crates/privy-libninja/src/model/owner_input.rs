use serde::{Serialize, Deserialize};
use super::{PublicKeyOwner, UserOwner};
///The owner of the resource. If you provide this, do not specify an owner_id as it will be generated automatically. When updating a wallet, you can set the owner to null to remove the owner.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum OwnerInput {
    #[serde(rename = "Public key owner")]
    PublicKeyOwner(PublicKeyOwner),
    #[serde(rename = "User owner")]
    UserOwner(UserOwner),
}
