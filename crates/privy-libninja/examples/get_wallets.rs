#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let privy_app_id = "your privy app id";
    let response = client
        .get_wallets(privy_app_id)
        .chain_type(serde_json::json!({}))
        .cursor("your cursor")
        .limit(1.0)
        .user_id("your user id")
        .await
        .unwrap();
    println!("{:#?}", response);
}
