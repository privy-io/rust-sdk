#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
use privy_libninja::request::get_wallet_balance::GetWalletBalanceRequired;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let asset = serde_json::json!({});
    let chain = serde_json::json!({});
    let privy_app_id = "your privy app id";
    let wallet_id = "your wallet id";
    let response = client
        .get_wallet_balance(GetWalletBalanceRequired {
            asset,
            chain,
            privy_app_id,
            wallet_id,
        })
        .include_currency("your include currency")
        .await
        .unwrap();
    println!("{:#?}", response);
}
