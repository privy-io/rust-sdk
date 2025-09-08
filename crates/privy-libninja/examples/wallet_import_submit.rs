#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let privy_app_id = "your privy app id";
    let wallet = serde_json::json!({});
    let response = client
        .wallet_import_submit(privy_app_id, wallet)
        .additional_signers(vec![serde_json::json!({})])
        .owner(serde_json::json!({}))
        .owner_id("your owner id")
        .policy_ids(&["your policy ids"])
        .await
        .unwrap();
    println!("{:#?}", response);
}
