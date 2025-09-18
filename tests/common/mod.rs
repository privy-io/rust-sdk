#![allow(dead_code)]

use std::{
    env,
    time::{Duration, SystemTime},
};

use anyhow::Result;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use privy_rs::{
    PrivyApiError, PrivyClient, PrivyExportError, PrivySignedApiError, client::PrivyClientOptions,
    generated::types::*,
};
use serde::Serialize;

// testing helper for the macro to handle both normal and signed errors
pub(crate) trait IntoApi: Sized {
    fn into_api(self) -> Result<PrivyApiError, Self>;
}

impl IntoApi for PrivySignedApiError {
    fn into_api(self) -> Result<PrivyApiError, Self> {
        match self {
            PrivySignedApiError::Api(e) => Ok(e),
            PrivySignedApiError::SignatureGeneration(_) => Err(self),
        }
    }
}

impl IntoApi for PrivyApiError {
    fn into_api(self) -> Result<PrivyApiError, Self> {
        Ok(self)
    }
}

impl IntoApi for PrivyExportError {
    fn into_api(self) -> Result<PrivyApiError, Self> {
        match self {
            PrivyExportError::Api(e) => Ok(e),
            PrivyExportError::SignatureGeneration(_) => Err(self),
            PrivyExportError::Key(_) => Err(self),
        }
    }
}

#[macro_export]
macro_rules! debug_response {
    ($future_expr:expr) => {
        async {
            // Await the future provided by the user
            let result = $future_expr.await;

            // Use the full path for robustness within the macro
            use privy_rs::PrivyApiError;
            use $crate::common::IntoApi;

            match result {
                // On success, pass the value through wrapped in Ok.
                Ok(value) => Ok(value),

                // On failure, inspect the error.
                Err(err) => {
                    let err = err.into_api();
                    match err {
                        // This is the specific error we want to debug and panic on.
                        Ok(PrivyApiError::UnexpectedResponse(resp)) => {
                            let body = resp
                                .text()
                                .await
                                .unwrap_or_else(|_| "[Could not decode body]".to_string());
                            println!("\n--- UNEXPECTED API RESPONSE ---");
                            println!("Body: {:?}", body);
                            println!("-----------------------------\n");
                            panic!("Panicking due to unexpected API response.");
                        }
                        Ok(other_err) => Err(other_err.into()),
                        // For any other error, pass it through wrapped in Err.
                        // This allows the `?` operator to work correctly.
                        Err(other_err) => Err(other_err),
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

pub async fn get_test_wallet_by_type(
    client: &PrivyClient,
    chain_type: WalletChainType,
    owner: Option<&str>,
) -> Result<Wallet> {
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
    Ok(wallet.into_inner())
}

pub async fn get_test_wallet_id_by_type(
    client: &PrivyClient,
    chain_type: WalletChainType,
    owner: Option<&str>,
) -> Result<String> {
    get_test_wallet_by_type(client, chain_type, owner)
        .await
        .map(|w| w.id)
}

/// Create a test user with a linked email address (for use with JWT authentication)
pub async fn ensure_test_user(client: &PrivyClient) -> Result<User> {
    // we don't need a whole uuid, just the last 12 chars
    let test_user_id = uuid::Uuid::new_v4().to_string().split_off(12);
    let test_user_id = format!("rust-sdk-{test_user_id}@privy.io");

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

pub async fn ensure_test_policy(
    client: &PrivyClient,
    rules: Vec<PolicyRuleRequestBody>,
) -> Result<Policy> {
    ensure_test_policy_with_user(client, rules, None).await
}

pub async fn ensure_test_policy_with_user(
    client: &PrivyClient,
    rules: Vec<PolicyRuleRequestBody>,
    user: Option<OwnerInput>,
) -> Result<Policy> {
    let unique_name = format!("test-policy-{}", chrono::Utc::now().timestamp());
    let policy_body = CreatePolicyBody {
        chain_type: PolicyChainType::Solana,
        name: CreatePolicyBodyName::try_from(unique_name).unwrap(),
        owner: user,
        owner_id: None,
        rules,
        version: CreatePolicyBodyVersion::try_from("1.0").unwrap(),
    };

    Ok(
        debug_response!(client.policies().create(None, &policy_body))
            .await
            .map(|r| r.into_inner())?,
    )
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
