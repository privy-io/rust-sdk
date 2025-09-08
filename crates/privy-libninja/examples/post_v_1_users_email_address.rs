#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let address = "your address";
    let privy_app_id = "your privy app id";
    let response = client
        .post_v1_users_email_address(address, privy_app_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}
