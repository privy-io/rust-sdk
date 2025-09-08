#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
use privy_libninja::request::post_v_1_policies::PostV1PoliciesRequired;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let chain_type = PolicyChainType::Ethereum;
    let name = "your name";
    let privy_app_id = "your privy app id";
    let rules = vec![
        PolicyRule { action : "your action".to_owned(), conditions :
        vec![serde_json::json!({})], method : "your method".to_owned(), name :
        "your name".to_owned() }
    ];
    let version = "your version";
    let response = client
        .post_v1_policies(PostV1PoliciesRequired {
            chain_type,
            name,
            privy_app_id,
            rules,
            version,
        })
        .owner(OwnerInput::PublicKeyOwner)
        .owner_id(serde_json::json!({}))
        .privy_idempotency_key("your privy idempotency key")
        .await
        .unwrap();
    println!("{:#?}", response);
}
