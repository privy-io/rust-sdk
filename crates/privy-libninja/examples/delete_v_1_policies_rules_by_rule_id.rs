#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let policy_id = "your policy id";
    let privy_app_id = "your privy app id";
    let rule_id = "your rule id";
    let response = client
        .delete_v1_policies_rules_by_rule_id(policy_id, privy_app_id, rule_id)
        .privy_authorization_signature("your privy authorization signature")
        .await
        .unwrap();
    println!("{:#?}", response);
}
