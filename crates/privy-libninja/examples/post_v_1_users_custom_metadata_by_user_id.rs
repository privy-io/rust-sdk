#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let custom_metadata = std::collections::HashMap::new();
    let privy_app_id = "your privy app id";
    let user_id = "your user id";
    let response = client
        .post_v1_users_custom_metadata_by_user_id(custom_metadata, privy_app_id, user_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}
