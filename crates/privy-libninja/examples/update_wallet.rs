#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let privy_app_id = "your privy app id";
    let wallet_id = "your wallet id";
    let response = client
        .update_wallet(privy_app_id, wallet_id)
        .additional_signers(WalletAdditionalSigner {
            override_policy_ids: vec!["your override policy ids".to_owned()],
            signer_id: "your signer id".to_owned(),
        })
        .owner(OwnerInput::PublicKeyOwner)
        .owner_id(serde_json::json!({}))
        .policy_ids(&["your policy ids"])
        .privy_authorization_signature("your privy authorization signature")
        .await
        .unwrap();
    println!("{:#?}", response);
}
