use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct PrivySigner {
    pub app_id: String,
    pub app_secret: String,
    pub wallet_id: String,
    pub api_base_url: String,
    pub client: Client,
    pub public_key: String,
}

// API request/response types for Privy
#[derive(Serialize)]
pub struct SignTransactionRequest {
    pub method: &'static str,
    pub caip2: &'static str,
    pub params: SignTransactionParams,
}

#[derive(Serialize)]
pub struct SignTransactionParams {
    pub transaction: String,
    pub encoding: &'static str,
}

#[derive(Deserialize, Debug)]
pub struct SignTransactionResponse {
    pub method: String,
    pub data: SignTransactionData,
}

#[derive(Deserialize, Debug)]
pub struct SignTransactionData {
    pub signature: String,
}

// Wallet info response
#[derive(Deserialize, Debug)]
pub struct WalletResponse {
    pub id: String,
    pub address: String,
    pub chain_type: String,
    pub wallet_client_type: String,
    pub connector_type: Option<String>,
    pub imported: bool,
    pub delegated: bool,
    pub hd_path: Option<String>,
    pub public_key: Option<String>,
}

