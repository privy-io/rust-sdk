#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let body = serde_json::json!({});
    let privy_app_id = "your privy app id";
    let response = client.wallet_import_init(body, privy_app_id).await.unwrap();
    println!("{:#?}", response);
}
