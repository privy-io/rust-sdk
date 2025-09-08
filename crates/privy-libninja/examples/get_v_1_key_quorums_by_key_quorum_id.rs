#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let key_quorum_id = "your key quorum id";
    let privy_app_id = "your privy app id";
    let response = client
        .get_v1_key_quorums_by_key_quorum_id(key_quorum_id, privy_app_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}
