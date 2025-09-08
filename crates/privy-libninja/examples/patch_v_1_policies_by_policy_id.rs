#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let policy_id = "your policy id";
    let privy_app_id = "your privy app id";
    let response = client
        .patch_v1_policies_by_policy_id(policy_id, privy_app_id)
        .name("your name")
        .owner(OwnerInput::PublicKeyOwner)
        .owner_id(serde_json::json!({}))
        .privy_authorization_signature("your privy authorization signature")
        .rules(
            vec![
                PolicyRule { action : "your action".to_owned(), conditions :
                vec![serde_json::json!({})], method : "your method".to_owned(), name :
                "your name".to_owned() }
            ],
        )
        .await
        .unwrap();
    println!("{:#?}", response);
}
