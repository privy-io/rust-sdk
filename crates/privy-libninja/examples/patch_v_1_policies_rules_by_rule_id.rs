#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
use privy_libninja::request::patch_v_1_policies_rules_by_rule_id::PatchV1PoliciesRulesByRuleIdRequired;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let action = "your action";
    let conditions = vec![serde_json::json!({})];
    let method = "your method";
    let name = "your name";
    let policy_id = "your policy id";
    let privy_app_id = "your privy app id";
    let rule_id = "your rule id";
    let response = client
        .patch_v1_policies_rules_by_rule_id(PatchV1PoliciesRulesByRuleIdRequired {
            action,
            conditions,
            method,
            name,
            policy_id,
            privy_app_id,
            rule_id,
        })
        .privy_authorization_signature("your privy authorization signature")
        .await
        .unwrap();
    println!("{:#?}", response);
}
