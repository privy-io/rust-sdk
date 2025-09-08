#![allow(unused_imports)]
use privy_libninja::model::*;
use privy_libninja::PrivyLibninjaClient;
use privy_libninja::request::wallet_export::WalletExportRequired;
#[tokio::main]
async fn main() {
    let client = PrivyLibninjaClient::from_env();
    let encryption_type = HpkeEncryption::Hpke;
    let privy_app_id = "your privy app id";
    let recipient_public_key = "your recipient public key";
    let wallet_id = "your wallet id";
    let response = client
        .wallet_export(WalletExportRequired {
            encryption_type,
            privy_app_id,
            recipient_public_key,
            wallet_id,
        })
        .privy_authorization_signature("your privy authorization signature")
        .await
        .unwrap();
    println!("{:#?}", response);
}
