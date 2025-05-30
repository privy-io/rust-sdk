pub mod types;
pub mod utils;

use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Signature, Signer},
};
use std::str::FromStr;

use crate::types::*;

impl PrivyConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            app_id: std::env::var("PRIVY_APP_ID").ok(),
            app_secret: std::env::var("PRIVY_APP_SECRET").ok(),
            wallet_id: std::env::var("PRIVY_WALLET_ID").ok(),
        }
    }
    
    /// Merge CLI arguments with existing config (CLI takes precedence)
    pub fn merge_with_cli(
        mut self,
        app_id: Option<String>,
        app_secret: Option<String>,
        wallet_id: Option<String>,
    ) -> Self {
        if app_id.is_some() {
            self.app_id = app_id;
        }
        if app_secret.is_some() {
            self.app_secret = app_secret;
        }
        if wallet_id.is_some() {
            self.wallet_id = wallet_id;
        }
        self
    }
    
    /// Build a PrivySigner from the config
    pub fn build(self) -> Result<PrivySigner, PrivyError> {
        Ok(PrivySigner {
            app_id: self.app_id.ok_or(PrivyError::MissingConfig("app_id"))?,
            app_secret: self.app_secret.ok_or(PrivyError::MissingConfig("app_secret"))?,
            wallet_id: self.wallet_id.ok_or(PrivyError::MissingConfig("wallet_id"))?,
            api_base_url: "https://api.privy.io/v1".to_string(),
            client: reqwest::Client::new(),
        })
    }
}

impl PrivySigner {
    /// Create a new PrivySigner
    pub fn new(app_id: String, app_secret: String, wallet_id: String) -> Self {
        Self {
            app_id,
            app_secret,
            wallet_id,
            api_base_url: "https://api.privy.io/v1".to_string(),
            client: reqwest::Client::new(),
        }
    }
    
    /// Get the Basic Auth header value
    fn get_auth_header(&self) -> String {
        let credentials = format!("{}:{}", self.app_id, self.app_secret);
        format!("Basic {}", STANDARD.encode(credentials))
    }
    
    /// Get the public key for this wallet
    pub async fn get_public_key(&self) -> Result<Pubkey, PrivyError> {
        let url = format!("{}/wallets/{}", self.api_base_url, self.wallet_id);
        
        let response = self.client
            .get(&url)
            .header("Authorization", self.get_auth_header())
            .header("privy-app-id", &self.app_id)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(PrivyError::ApiError(response.status().as_u16()));
        }
        
        let wallet_info: WalletResponse = response.json().await?;
        
        // For Solana wallets, the address is the public key
        Pubkey::from_str(&wallet_info.address)
            .map_err(|_| PrivyError::InvalidPublicKey)
    }
    
    /// Sign a transaction
    pub async fn sign_transaction(&self, transaction: &[u8]) -> Result<Signature, PrivyError> {
        let url = format!("{}/wallets/{}/rpc", self.api_base_url, self.wallet_id);
        
        let request = SignTransactionRequest {
            method: "signTransaction",
            caip2: "solana:EtWTRABZaYq6iMfeYKouRu166VU2xqa1", // Solana mainnet
            params: SignTransactionParams {
                transaction: STANDARD.encode(transaction),
                encoding: "base64",
            },
        };
        
        let response = self.client
            .post(&url)
            .header("Authorization", self.get_auth_header())
            .header("privy-app-id", &self.app_id)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
            
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_default();
            eprintln!("Privy API error {}: {}", status, error_text);
            return Err(PrivyError::ApiError(status));
        }
        
        let sign_response: SignTransactionResponse = response.json().await?;
        
        // Decode the signature from base64
        let sig_bytes = STANDARD.decode(&sign_response.data.signature)?;
        
        Signature::try_from(sig_bytes.as_slice())
            .map_err(|_| PrivyError::InvalidSignature)
    }
}

// For compatibility with code expecting a blocking Signer trait
pub struct PrivySignerBlocking {
    inner: PrivySigner,
    runtime: tokio::runtime::Runtime,
    cached_pubkey: Option<Pubkey>,
}

impl PrivySignerBlocking {
    pub fn new(signer: PrivySigner) -> Result<Self, PrivyError> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| PrivyError::ApiError(500))?; // Convert to appropriate error
            
        // Pre-fetch the public key
        let cached_pubkey = runtime.block_on(signer.get_public_key())?;
        
        Ok(Self {
            inner: signer,
            runtime,
            cached_pubkey: Some(cached_pubkey),
        })
    }
}

impl Signer for PrivySignerBlocking {
    fn pubkey(&self) -> Pubkey {
        self.cached_pubkey.expect("Public key not initialized")
    }
    
    fn try_pubkey(&self) -> Result<Pubkey, solana_sdk::signer::SignerError> {
        self.cached_pubkey.ok_or(solana_sdk::signer::SignerError::Custom(
            "Public key not initialized".to_string()
        ))
    }
    
    fn sign_message(&self, message: &[u8]) -> Signature {
        self.runtime
            .block_on(self.inner.sign_transaction(message))
            .expect("Failed to sign message")
    }
    
    fn try_sign_message(&self, message: &[u8]) -> Result<Signature, solana_sdk::signer::SignerError> {
        self.runtime
            .block_on(self.inner.sign_transaction(message))
            .map_err(|e| solana_sdk::signer::SignerError::Custom(e.to_string()))
    }
    
    fn is_interactive(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_from_env() {
        // Save existing env vars
        let saved_app_id = std::env::var("PRIVY_APP_ID").ok();
        let saved_app_secret = std::env::var("PRIVY_APP_SECRET").ok();
        let saved_wallet_id = std::env::var("PRIVY_WALLET_ID").ok();
        
        // Set test env vars
        std::env::set_var("PRIVY_APP_ID", "test_app_id");
        std::env::set_var("PRIVY_APP_SECRET", "test_secret");
        std::env::set_var("PRIVY_WALLET_ID", "test_wallet");
        
        let config = PrivyConfig::from_env();
        assert_eq!(config.app_id, Some("test_app_id".to_string()));
        assert_eq!(config.app_secret, Some("test_secret".to_string()));
        assert_eq!(config.wallet_id, Some("test_wallet".to_string()));
        
        // Restore original env vars
        match saved_app_id {
            Some(val) => std::env::set_var("PRIVY_APP_ID", val),
            None => std::env::remove_var("PRIVY_APP_ID"),
        }
        match saved_app_secret {
            Some(val) => std::env::set_var("PRIVY_APP_SECRET", val),
            None => std::env::remove_var("PRIVY_APP_SECRET"),
        }
        match saved_wallet_id {
            Some(val) => std::env::set_var("PRIVY_WALLET_ID", val),
            None => std::env::remove_var("PRIVY_WALLET_ID"),
        }
    }
    
    #[test]
    fn test_config_merge() {
        let config = PrivyConfig {
            app_id: Some("env_id".to_string()),
            app_secret: Some("env_secret".to_string()),
            wallet_id: None,
        };
        
        let merged = config.merge_with_cli(
            Some("cli_id".to_string()),
            None,
            Some("cli_wallet".to_string()),
        );
        
        assert_eq!(merged.app_id, Some("cli_id".to_string()));
        assert_eq!(merged.app_secret, Some("env_secret".to_string()));
        assert_eq!(merged.wallet_id, Some("cli_wallet".to_string()));
    }
}
