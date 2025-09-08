#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let privy_app_id = "your privy app id";
    let response = client
        .post_v1_key_quorums(privy_app_id)
        .authorization_threshold(1.0)
        .display_name("your display name")
        .public_keys(&["your public keys"])
        .user_ids(&["your user ids"])
        .await
        .unwrap();
    println!("{:#?}", response);
}
