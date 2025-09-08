#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let linked_accounts = vec![LinkedAccountInput(serde_json::json!({}))];
    let privy_app_id = "your privy app id";
    let response = client
        .create_user(linked_accounts, privy_app_id)
        .custom_metadata(std::collections::HashMap::new())
        .wallets(vec![serde_json::json!({})])
        .await
        .unwrap();
    println!("{:#?}", response);
}
