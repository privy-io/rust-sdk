#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let key_quorum_id = "your key quorum id";
    let privy_app_id = "your privy app id";
    let response = client
        .patch_v1_key_quorums_by_key_quorum_id(key_quorum_id, privy_app_id)
        .authorization_threshold(1.0)
        .display_name("your display name")
        .privy_authorization_signature("your privy authorization signature")
        .public_keys(&["your public keys"])
        .user_ids(&["your user ids"])
        .await
        .unwrap();
    println!("{:#?}", response);
}
