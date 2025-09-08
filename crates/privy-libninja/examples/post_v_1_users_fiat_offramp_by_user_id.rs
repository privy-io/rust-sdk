#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
use privy_libninja::request::post_v_1_users_fiat_offramp_by_user_id::PostV1UsersFiatOfframpByUserIdRequired;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let amount = "your amount";
    let destination = serde_json::json!({});
    let privy_app_id = "your privy app id";
    let provider = "your provider";
    let source = serde_json::json!({});
    let user_id = "your user id";
    let response = client
        .post_v1_users_fiat_offramp_by_user_id(PostV1UsersFiatOfframpByUserIdRequired {
            amount,
            destination,
            privy_app_id,
            provider,
            source,
            user_id,
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
