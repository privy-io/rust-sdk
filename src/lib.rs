#![deny(clippy::unwrap_used)]
#![warn(clippy::pedantic)]

use std::{str::FromStr, time::Duration};

use base64::{Engine, engine::general_purpose::STANDARD};
use delegate::delegate;
use privy_api::{
    Client,
    types::{
        SolanaSignMessageRpcInputChainType, SolanaSignMessageRpcInputMethod,
        SolanaSignMessageRpcInputParamsEncoding, WalletRpcBody,
        builder::{SolanaSignMessageRpcInput, SolanaSignMessageRpcInputParams},
    },
};
use reqwest::header::{CONTENT_TYPE, HeaderValue};

pub(crate) mod errors;
pub(crate) mod keys;
pub(crate) mod types;

pub use errors::*;
pub use keys::*;
pub use types::*;

impl PrivySigner {
    /// Create a new `PrivySigner`
    ///
    /// # Errors
    /// This can fail for two reasons, either the `app_id` or `app_secret` are not
    /// valid headers, or that the underlying http client could not be created.
    pub fn new(
        app_id: String,
        app_secret: String,
        wallet_id: String,
        public_key: String,
    ) -> Result<Self, PrivyCreateError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&get_auth_header(&app_id, &app_secret))?,
        );
        headers.insert("privy-app-id", HeaderValue::from_str(&app_id)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("privy-client", HeaderValue::from_static("rust-sdk"));

        let client_with_custom_defaults = reqwest::ClientBuilder::new()
            .connect_timeout(Duration::from_secs(15))
            .timeout(Duration::from_secs(15))
            .default_headers(headers)
            .build()?;

        Ok(Self {
            app_id,
            app_secret,
            wallet_id,
            client: Client::new_with_client("https://api.privy.io", client_with_custom_defaults),
            public_key,
        })
    }

    // this is the crux of the impl, a handy macro that delegates all the
    // unexciting methods to the inner client automatically. we can do nice
    // things like auto-populating items on the builders
    delegate! {
        to self.client {
            #[expr($.privy_app_id(&self.app_id))]
            #[must_use] pub fn authenticate(&self) -> privy_api::builder::Authenticate<'_>;

            #[expr($.privy_app_id(&self.app_id))]
            #[must_use] pub fn get_wallet(&self) -> privy_api::builder::GetWallet<'_>;

            #[expr($.privy_app_id(&self.app_id))]
            #[must_use] pub fn get_wallets(&self) -> privy_api::builder::GetWallets<'_>;

            #[expr($.privy_app_id(&self.app_id))]
            #[must_use] pub fn create_wallet(&self) -> privy_api::builder::CreateWallet<'_>;

            #[expr($.privy_app_id(&self.app_id))]
            #[must_use] pub fn update_wallet(&self) -> privy_api::builder::UpdateWallet<'_>;

            #[expr($.privy_app_id(&self.app_id))]
            #[must_use] pub fn create_user(&self) -> privy_api::builder::CreateUser<'_>;

            #[expr($.privy_app_id(&self.app_id))]
            #[must_use] pub fn get_user(&self) -> privy_api::builder::GetUser<'_>;

            #[expr($.privy_app_id(&self.app_id))]
            #[must_use] pub fn get_users(&self) -> privy_api::builder::GetUsers<'_>;
        }
    }

    /// Sign a message on the solana blockchain
    ///
    /// # Errors
    /// This can fail if the underlying http request fails
    ///
    /// # Panics
    /// If the server returns a mismatched RPC response
    pub async fn sign(
        &self,
        message: &[u8],
    ) -> Result<solana_sdk::signature::Signature, PrivyError> {
        let input = SolanaSignMessageRpcInput::default()
            .method(SolanaSignMessageRpcInputMethod::SignMessage)
            .chain_type(Some(SolanaSignMessageRpcInputChainType::Solana))
            .params(
                SolanaSignMessageRpcInputParams::default()
                    .encoding(SolanaSignMessageRpcInputParamsEncoding::Base64)
                    .message(STANDARD.encode(message)),
            )
            .try_into()?;

        let response = self
            .client
            .wallet_rpc()
            .wallet_id(&self.wallet_id)
            .privy_app_id(&self.app_id)
            .body(WalletRpcBody::SolanaSignMessageRpcInput(input))
            .send()
            .await?;

        let privy_api::types::WalletRpcResponse::SolanaSignMessageRpcResponse(sign_response) =
            response.into_inner()
        else {
            panic!("invalid response type");
        };

        let mut sig_bytes: [u8; 64] = [0; 64];
        STANDARD
            .decode_slice(&sign_response.data.signature, &mut sig_bytes)
            .expect("exactly 64 bytes");

        Ok(solana_sdk::signature::Signature::from(sig_bytes))
    }

    /// Get the public key of the solana wallet
    ///
    /// # Errors
    /// This can fail if the public key is not a valid solana pubkey
    pub fn solana_pubkey(&self) -> Result<solana_sdk::pubkey::Pubkey, ParsePubkeyError> {
        tracing::debug!("solana pubkey: {}", self.public_key);
        solana_sdk::pubkey::Pubkey::from_str(&self.public_key)
    }

    /// Create canonical request data for signing
    ///
    /// # Errors
    /// This can fail if JSON serialization fails
    pub fn build_canonical_request(
        &self,
        method: Method,
        url: String,
        body: Option<serde_json::Value>,
        headers: Option<serde_json::Value>,
    ) -> Result<String, serde_json::Error> {
        let builder = PrivySignerBuilder::new(method, url)
            .body(body.unwrap_or_default())
            .headers(headers.unwrap_or_default());

        builder.canonicalize()
    }

    /// Create canonical request data for wallet update operations
    ///
    /// # Errors
    /// This can fail if JSON serialization fails
    pub fn build_update_wallet_canonical_request(
        &self,
        wallet_id: &str,
        body: serde_json::Value,
        idempotency_key: Option<String>,
    ) -> Result<String, serde_json::Error> {
        let url = format!("https://api.privy.io/v1/wallets/{wallet_id}");

        let mut headers = serde_json::Map::new();
        headers.insert(
            "privy-app-id".into(),
            serde_json::Value::String(self.app_id.clone()),
        );
        if let Some(key) = idempotency_key {
            headers.insert(
                "privy-idempotency-key".to_string(),
                serde_json::Value::String(key),
            );
        }

        self.build_canonical_request(
            Method::PATCH,
            url,
            Some(body),
            Some(serde_json::Value::Object(headers)),
        )
    }
}

fn get_auth_header(app_id: &str, app_secret: &str) -> String {
    let credentials = format!("{app_id}:{app_secret}");
    format!("Basic {}", STANDARD.encode(credentials))
}
