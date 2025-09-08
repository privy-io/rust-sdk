#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
use privy_libninja::request::post_v_1_apps_fiat_by_app_id::PostV1AppsFiatByAppIdRequired;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let api_key = "your api key";
    let app_id = "your app id";
    let privy_app_id = "your privy app id";
    let provider = "your provider";
    let response = client
        .post_v1_apps_fiat_by_app_id(PostV1AppsFiatByAppIdRequired {
            api_key,
            app_id,
            privy_app_id,
            provider,
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
