#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
use privy_libninja::request::post_v_1_users_fiat_kyc_link_by_user_id::PostV1UsersFiatKycLinkByUserIdRequired;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let email = "your email";
    let privy_app_id = "your privy app id";
    let provider = "your provider";
    let user_id = "your user id";
    let response = client
        .post_v1_users_fiat_kyc_link_by_user_id(PostV1UsersFiatKycLinkByUserIdRequired {
            email,
            privy_app_id,
            provider,
            user_id,
        })
        .endorsements(&["your endorsements"])
        .full_name("your full name")
        .redirect_uri("your redirect uri")
        .type_("your type")
        .await
        .unwrap();
    println!("{:#?}", response);
}
