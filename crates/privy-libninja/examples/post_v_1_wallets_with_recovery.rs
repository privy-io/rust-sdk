#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
use privy_libninja::request::post_v_1_wallets_with_recovery::PostV1WalletsWithRecoveryRequired;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let primary_signer = serde_json::json!({});
    let privy_app_id = "your privy app id";
    let recovery_user = serde_json::json!({});
    let wallets = vec![serde_json::json!({})];
    let response = client
        .post_v1_wallets_with_recovery(PostV1WalletsWithRecoveryRequired {
            primary_signer,
            privy_app_id,
            recovery_user,
            wallets,
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
