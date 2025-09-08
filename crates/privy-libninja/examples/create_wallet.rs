#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let body = vec![
        WalletAdditionalSigner { override_policy_ids : vec!["your override policy ids"
        .to_owned()], signer_id : "your signer id".to_owned() }
    ];
    let privy_app_id = "your privy app id";
    let response = client
        .create_wallet(body, privy_app_id)
        .privy_idempotency_key("your privy idempotency key")
        .await
        .unwrap();
    println!("{:#?}", response);
}
