#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let params = serde_json::json!({});
    let privy_app_id = "your privy app id";
    let wallet_id = "your wallet id";
    let response = client
        .raw_sign(params, privy_app_id, wallet_id)
        .privy_authorization_signature("your privy authorization signature")
        .privy_idempotency_key("your privy idempotency key")
        .await
        .unwrap();
    println!("{:#?}", response);
}
