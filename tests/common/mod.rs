use std::{
    env,
    time::{Duration, SystemTime},
};

use anyhow::Result;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use privy_rs::{
    PrivyClient,
    client::PrivyClientOptions,
    generated::types::{
        CreateUserBody, CreateWalletBody, LinkedAccountCustomJwtInput, LinkedAccountEmailInput,
        LinkedAccountInput, OwnerInput, SearchUsersBody, User, WalletChainType,
    },
};
use serde::Serialize;

#[macro_export]
macro_rules! debug_response {
    ($future_expr:expr) => {
        async {
            // Await the future provided by the user
            let result = $future_expr.await;

            // Use the full path for robustness within the macro
            use progenitor_client::Error as ProgenitorError;

            match result {
                // On success, pass the value through wrapped in Ok.
                Ok(value) => Ok(value),

                // On failure, inspect the error.
                Err(err) => {
                    match err {
                        // This is the specific error we want to debug and panic on.
                        ProgenitorError::UnexpectedResponse(resp) => {
                            let body = resp
                                .text()
                                .await
                                .unwrap_or_else(|_| "[Could not decode body]".to_string());
                            println!("\n--- UNEXPECTED API RESPONSE ---");
                            println!("Body: {:?}", body);
                            println!("-----------------------------\n");
                            panic!("Panicking due to unexpected API response.");
                        }
                        // For any other error, pass it through wrapped in Err.
                        // This allows the `?` operator to work correctly.
                        other_err => Err(other_err),
                    }
                }
            }
        }
    };
}

pub fn get_test_client() -> Result<PrivyClient> {
    let app_id = env::var("PRIVY_TEST_APP_ID")
        .or_else(|_| env::var("PRIVY_APP_ID"))
        .expect("PRIVY_TEST_APP_ID or PRIVY_APP_ID environment variable not set");
    let app_secret = env::var("PRIVY_TEST_APP_SECRET")
        .or_else(|_| env::var("PRIVY_APP_SECRET"))
        .expect("PRIVY_TEST_APP_SECRET or PRIVY_APP_SECRET environment variable not set");
    let url = env::var("PRIVY_TEST_URL")
        .or_else(|_| env::var("PRIVY_URL"))
        .ok();

    tracing::debug!(
        "Starting client against {} on {}",
        app_id,
        url.as_deref().unwrap_or("default")
    );

    let client = url
        .map(|url| {
            PrivyClient::new_with_options(
                app_id.clone(),
                app_secret.clone(),
                PrivyClientOptions {
                    base_url: url,
                    ..Default::default()
                },
            )
        })
        .unwrap_or_else(|| PrivyClient::new(app_id, app_secret))
        .unwrap();

    Ok(client)
}

pub async fn get_test_wallet_id_by_type(
    client: &PrivyClient,
    chain_type: WalletChainType,
    owner: Option<&str>,
) -> Result<String> {
    let wallet_id = match chain_type {
        WalletChainType::Solana => env::var("PRIVY_TEST_SOLANA_WALLET_ID")
            .or_else(|_| env::var("PRIVY_TEST_WALLET_ID"))
            .ok(),
        WalletChainType::Ethereum => env::var("PRIVY_TEST_ETH_WALLET_ID")
            .or_else(|_| env::var("PRIVY_TEST_WALLET_ID"))
            .ok(),
        _ => env::var("PRIVY_TEST_WALLET_ID").ok(),
    };

    if let Some(id) = wallet_id {
        return Ok(id);
    }

    tracing::info!(
        "No wallet ID found for {:?}, creating new wallet...",
        chain_type
    );

    let wallet = client
        .wallets()
        .create(
            None,
            &CreateWalletBody {
                chain_type,
                additional_signers: None,
                owner: owner.map(|o| OwnerInput::UserId(o.to_string())),
                owner_id: None,
                policy_ids: vec![],
            },
        )
        .await?;

    tracing::info!("Created new {:?} wallet with ID: {}", chain_type, wallet.id);
    tracing::debug!("Wallet: {:?}", wallet);
    Ok(wallet.into_inner().id)
}

/// Create a test user with a linked email address (for use with JWT authentication)
pub async fn ensure_test_user(client: &PrivyClient) -> Result<User> {
    // we don't need a whole uuid, just the last 12 chars
    let test_user_id = uuid::Uuid::new_v4().to_string().split_off(12);
    let test_user_id = format!("rust-sdk-{}@privy.io", test_user_id);

    let user = client
        .users()
        .search(&SearchUsersBody::Variant1 {
            emails: vec![test_user_id.clone()],
            phone_numbers: vec![],
            wallet_addresses: vec![],
        })
        .await;

    if let Ok(user) = user {
        return Ok(user.into_inner());
    }

    let user = debug_response!(client.users().create(&CreateUserBody {
        linked_accounts: vec![
            LinkedAccountInput::EmailInput(LinkedAccountEmailInput {
                address: test_user_id.clone(),
                type_: privy_rs::generated::types::LinkedAccountEmailInputType::Email,
            }),
            LinkedAccountInput::CustomJwtInput(LinkedAccountCustomJwtInput {
                custom_user_id: test_user_id.parse().unwrap(),
                type_: privy_rs::generated::types::LinkedAccountCustomJwtInputType::CustomAuth,
            }),
        ],
        custom_metadata: None,
        wallets: vec![],
    }))
    .await
    .map(|r| r.into_inner())?;

    Ok(user)
}

pub fn mint_staging_jwt(sub: &str) -> Result<String> {
    // pem-formatted private key
    let private_key = env::var("PRIVY_TEST_JWT_PRIVATE_KEY").expect("jwt secret needed");

    #[derive(Serialize)]
    struct PrivyClaims<'a> {
        sub: &'a str,
        exp: u64,
    }

    let claims = PrivyClaims {
        sub,
        exp: (SystemTime::now() + Duration::from_secs(60 * 60))
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    let token = jsonwebtoken::encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_rsa_pem(private_key.as_bytes())?,
    )?;

    Ok(token)
}
