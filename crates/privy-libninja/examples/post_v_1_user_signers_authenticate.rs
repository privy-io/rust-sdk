#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let privy_app_id = "your privy app id";
    let user_jwt = "your user jwt";
    let response = client
        .post_v1_user_signers_authenticate(privy_app_id, user_jwt)
        .encryption_type("your encryption type")
        .recipient_public_key("your recipient public key")
        .await
        .unwrap();
    println!("{:#?}", response);
}
