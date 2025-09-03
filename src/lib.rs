use std::str::FromStr;

use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::Client;

mod types;

pub use types::*;

impl PrivySigner {
    pub fn new(
        app_id: String,
        app_secret: String,
        wallet_id: String,
        _unused: String, // To match tk-rs signature
        public_key: String,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            app_id,
            app_secret,
            wallet_id,
            api_base_url: "https://api.privy.io/v1".to_string(),
            client: Client::new(),
            public_key,
        })
    }

    pub async fn sign(&self, message: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
        let url = format!("{}/wallets/{}/rpc", self.api_base_url, self.wallet_id);

        let request = SignTransactionRequest {
            method: "signTransaction",
            caip2: "solana:EtWTRABZaYq6iMfeYKouRu166VU2xqa1",
            params: SignTransactionParams {
                transaction: STANDARD.encode(message),
                encoding: "base64",
            },
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", self.get_auth_header())
            .header("privy-app-id", &self.app_id)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Privy API error {}: {}",
                status,
                error_text
            ));
        }

        let sign_response: SignTransactionResponse = response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let sig_bytes = STANDARD
            .decode(&sign_response.data.signature)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(sig_bytes)
    }

    pub async fn sign_solana(
        &self,
        message: &[u8],
    ) -> Result<solana_sdk::signature::Signature, anyhow::Error> {
        let sig = self.sign(message).await?;
        let sig_bytes: [u8; 64] = sig
            .try_into()
            .map_err(|_| anyhow::anyhow!("Invalid signature length"))?;
        Ok(solana_sdk::signature::Signature::from(sig_bytes))
    }

    fn get_auth_header(&self) -> String {
        let credentials = format!("{}:{}", self.app_id, self.app_secret);
        format!("Basic {}", STANDARD.encode(credentials))
    }

    pub fn solana_pubkey(&self) -> solana_sdk::pubkey::Pubkey {
        solana_sdk::pubkey::Pubkey::from_str(&self.public_key).unwrap()
    }
}
