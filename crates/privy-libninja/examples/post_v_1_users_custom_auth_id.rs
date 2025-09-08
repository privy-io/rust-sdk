#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let custom_user_id = "your custom user id";
    let privy_app_id = "your privy app id";
    let response = client
        .post_v1_users_custom_auth_id(custom_user_id, privy_app_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}
