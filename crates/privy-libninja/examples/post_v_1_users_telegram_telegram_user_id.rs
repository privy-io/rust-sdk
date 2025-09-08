#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let privy_app_id = "your privy app id";
    let telegram_user_id = "your telegram user id";
    let response = client
        .post_v1_users_telegram_telegram_user_id(privy_app_id, telegram_user_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}
