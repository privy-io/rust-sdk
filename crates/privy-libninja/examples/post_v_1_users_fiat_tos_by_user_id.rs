#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let privy_app_id = "your privy app id";
    let provider = "your provider";
    let user_id = "your user id";
    let response = client
        .post_v1_users_fiat_tos_by_user_id(privy_app_id, provider, user_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}
