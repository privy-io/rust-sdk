#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
use privy_libninja::request::post_v_1_users_accounts_unlink_by_user_id::PostV1UsersAccountsUnlinkByUserIdRequired;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let handle = "your handle";
    let privy_app_id = "your privy app id";
    let type_ = "your type";
    let user_id = "your user id";
    let response = client
        .post_v1_users_accounts_unlink_by_user_id(PostV1UsersAccountsUnlinkByUserIdRequired {
            handle,
            privy_app_id,
            type_,
            user_id,
        })
        .provider("your provider")
        .await
        .unwrap();
    println!("{:#?}", response);
}
