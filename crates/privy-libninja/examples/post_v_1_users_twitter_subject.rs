#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let privy_app_id = "your privy app id";
    let subject = "your subject";
    let response = client
        .post_v1_users_twitter_subject(privy_app_id, subject)
        .await
        .unwrap();
    println!("{:#?}", response);
}
