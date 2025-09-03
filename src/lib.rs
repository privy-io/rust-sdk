#![deny(clippy::unwrap_used)]

use privy_api::{
    Client,
    types::{
        SolanaSignMessageRpcInputChainType, SolanaSignMessageRpcInputMethod,
        SolanaSignMessageRpcInputParamsEncoding, WalletRpcBody,
        builder::{SolanaSignMessageRpcInput, SolanaSignMessageRpcInputParams},
    },
};
use reqwest::header::{CONTENT_TYPE, HeaderValue};

use std::{str::FromStr, time::Duration};

use base64::{Engine, engine::general_purpose::STANDARD};

pub(crate) mod errors;
pub(crate) mod keys;
pub(crate) mod types;

pub use errors::*;
pub use types::*;

impl PrivySigner {
    pub fn new(app_id: String, app_secret: String, wallet_id: String, public_key: String) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&get_auth_header(&app_id, &app_secret)).unwrap(),
        );
        headers.insert("privy-app-id", HeaderValue::from_str(&app_id).unwrap());
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("privy-client", HeaderValue::from_static("rust-sdk"));

        let client_with_custom_defaults = reqwest::ClientBuilder::new()
            .connect_timeout(Duration::from_secs(15))
            .timeout(Duration::from_secs(15))
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            app_id,
            app_secret,
            wallet_id,
            client: Client::new_with_client("https://api.privy.io", client_with_custom_defaults),
            public_key,
        }
    }

    pub async fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
        let input = SolanaSignMessageRpcInput::default()
            .method(SolanaSignMessageRpcInputMethod::SignMessage)
            .chain_type(Some(SolanaSignMessageRpcInputChainType::Solana))
            .params(
                SolanaSignMessageRpcInputParams::default()
                    .encoding(SolanaSignMessageRpcInputParamsEncoding::Base64)
                    .message(STANDARD.encode(message)),
            )
            .try_into()
            .unwrap();

        let response = self
            .client
            .wallet_rpc()
            .wallet_id(&self.wallet_id)
            .privy_app_id(&self.app_id)
            .body(WalletRpcBody::SolanaSignMessageRpcInput(input))
            .send()
            .await
            .unwrap();

        let sign_response = match response.into_inner() {
            privy_api::types::WalletRpcResponse::SolanaSignMessageRpcResponse(response) => response,
            _ => panic!("error"),
        };

        let sig_bytes = STANDARD.decode(&sign_response.data.signature)?;

        Ok(sig_bytes)
    }

    pub async fn sign_solana(&self, message: &[u8]) -> Result<solana_sdk::signature::Signature> {
        let sig = self.sign(message).await?;
        let sig_bytes: [u8; 64] = sig
            .try_into()
            .map_err(|_| PrivyError::InvalidSignatureLength)?;
        Ok(solana_sdk::signature::Signature::from(sig_bytes))
    }

    pub fn solana_pubkey(&self) -> Result<solana_sdk::pubkey::Pubkey> {
        tracing::info!("Solana pubkey: {}", self.public_key);
        Ok(solana_sdk::pubkey::Pubkey::from_str(&self.public_key)?)
    }
}

fn get_auth_header(app_id: &str, app_secret: &str) -> String {
    let credentials = format!("{}:{}", app_id, app_secret);
    format!("Basic {}", STANDARD.encode(credentials))
}
