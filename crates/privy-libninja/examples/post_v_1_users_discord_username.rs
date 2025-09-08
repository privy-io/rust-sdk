#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let privy_app_id = "your privy app id";
    let username = "your username";
    let response = client
        .post_v1_users_discord_username(privy_app_id, username)
        .await
        .unwrap();
    println!("{:#?}", response);
}
