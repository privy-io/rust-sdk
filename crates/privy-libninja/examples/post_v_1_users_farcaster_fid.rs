#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let fid = 1.0;
    let privy_app_id = "your privy app id";
    let response = client.post_v1_users_farcaster_fid(fid, privy_app_id).await.unwrap();
    println!("{:#?}", response);
}
