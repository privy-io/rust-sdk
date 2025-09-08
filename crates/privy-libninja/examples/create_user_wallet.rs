#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let privy_app_id = "your privy app id";
    let user_id = "your user id";
    let wallets = vec![serde_json::json!({})];
    let response = client
        .create_user_wallet(privy_app_id, user_id, wallets)
        .await
        .unwrap();
    println!("{:#?}", response);
}
