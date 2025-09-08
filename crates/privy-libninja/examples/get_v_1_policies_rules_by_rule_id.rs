#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let policy_id = "your policy id";
    let rule_id = "your rule id";
    let response = client
        .get_v1_policies_rules_by_rule_id(policy_id, rule_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}
