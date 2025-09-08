#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
use privy_libninja::request::post_v_1_users_fiat_accounts_by_user_id::PostV1UsersFiatAccountsByUserIdRequired;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let account_owner_name = "your account owner name";
    let currency = "your currency";
    let privy_app_id = "your privy app id";
    let provider = "your provider";
    let user_id = "your user id";
    let response = client
        .post_v1_users_fiat_accounts_by_user_id(PostV1UsersFiatAccountsByUserIdRequired {
            account_owner_name,
            currency,
            privy_app_id,
            provider,
            user_id,
        })
        .account(serde_json::json!({}))
        .address(serde_json::json!({}))
        .bank_name("your bank name")
        .first_name("your first name")
        .iban(serde_json::json!({}))
        .last_name("your last name")
        .swift(serde_json::json!({}))
        .await
        .unwrap();
    println!("{:#?}", response);
}
