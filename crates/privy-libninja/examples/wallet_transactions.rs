#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
use privy_libninja::request::wallet_transactions::WalletTransactionsRequired;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let asset = serde_json::json!({});
    let chain = "your chain";
    let privy_app_id = "your privy app id";
    let wallet_id = "your wallet id";
    let response = client
        .wallet_transactions(WalletTransactionsRequired {
            asset,
            chain,
            privy_app_id,
            wallet_id,
        })
        .cursor("your cursor")
        .limit(1.0)
        .await
        .unwrap();
    println!("{:#?}", response);
}
