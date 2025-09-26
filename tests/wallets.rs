use std::str::FromStr;

use anyhow::Result;
use base64::{Engine, engine::general_purpose::STANDARD};
use common::{ensure_test_user, get_test_client, get_test_wallet_id_by_type, mint_staging_jwt};
use hex::ToHex;
use p256::elliptic_curve::SecretKey;
use privy_rs::{
    AuthorizationContext, IntoKey, JwtUser, PrivateKey, PrivyHpke, generated::types::*,
};
use sha2::Digest;
use solana_sdk::{pubkey::Pubkey, transaction::Transaction};
use tracing_test::traced_test;

mod common;

#[tokio::test]
async fn test_wallets_list() -> Result<()> {
    let client = get_test_client()?;
    let wallets = client.wallets().list(None, None, Some(10.0), None).await?;

    println!("Retrieved {} wallets", wallets.data.len());

    Ok(())
}

#[tokio::test]
async fn test_wallets_create() -> Result<()> {
    let client = get_test_client()?;

    let create_body = CreateWalletBody {
        chain_type: WalletChainType::Solana,
        additional_signers: None,
        owner: None,
        owner_id: None,
        policy_ids: vec![],
    };

    let wallet = client.wallets().create(None, &create_body).await?;

    assert_eq!(wallet.chain_type, WalletChainType::Solana);
    println!("Created wallet with ID: {:?}", wallet.id);

    Ok(())
}

#[tokio::test]
async fn test_wallets_get() -> Result<()> {
    let client = get_test_client()?;
    let wallet_id = get_test_wallet_id_by_type(&client, WalletChainType::Solana, None).await?;

    let wallet = debug_response!(client.wallets().get(&wallet_id)).await?;

    assert_eq!(wallet.id, wallet_id);
    assert_eq!(wallet.chain_type, WalletChainType::Solana);

    println!("Retrieved wallet: {:?}", wallet.id);

    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_wallets_authenticate_with_jwt() -> Result<()> {
    let client = get_test_client()?;
    let user = ensure_test_user(&client).await?;

    // authenticate requires that the user own at least one wallet
    let _wallet =
        get_test_wallet_id_by_type(&client, WalletChainType::Solana, Some(&user.id)).await?;

    let custom_auth = user
        .linked_accounts
        .iter()
        .find_map(|la| match la {
            UserLinkedAccountsItem::CustomJwt(jwt) => Some(jwt.custom_user_id.to_owned()),
            _ => None,
        })
        .unwrap();

    let jwt_token = mint_staging_jwt(&custom_auth)?;

    tracing::info!("JWT token: {:?}", jwt_token);

    let privy_hpke = PrivyHpke::new();
    let auth_body = AuthenticateBody {
        encryption_type: Some(AuthenticateBodyEncryptionType::Hpke),
        user_jwt: jwt_token,
        recipient_public_key: Some(privy_hpke.public_key().unwrap()),
    };

    let result = debug_response!(client.wallets().authenticate_with_jwt(&auth_body)).await?;

    match result.into_inner() {
        AuthenticateResponse::WithEncryption { .. } => {
            println!("JWT authentication successful for user: {}", user.id);
        }
        _ => panic!("Unexpected response type"),
    }

    Ok(())
}

#[tokio::test]
async fn test_wallets_raw_sign_with_auth_context() -> Result<()> {
    let client = get_test_client()?;
    let wallet_id = get_test_wallet_id_by_type(&client, WalletChainType::Tron, None).await?;

    let raw_sign_body = privy_rs::generated::types::RawSign {
        params: RawSignParams {
            hash: Some(
                "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".to_string(),
            ),
        },
    };

    let ctx = AuthorizationContext::new();

    let result = debug_response!(
        client
            .wallets()
            .raw_sign(&wallet_id, &ctx, None, &raw_sign_body)
    )
    .await?;

    let sig = result.into_inner().data.signature;

    assert_ne!(sig.len(), 0);
    println!("Raw signature created: {sig:?}");

    Ok(())
}

#[tokio::test]
async fn test_wallets_update_with_auth_context() -> Result<()> {
    let client = get_test_client()?;
    let wallet_id = get_test_wallet_id_by_type(&client, WalletChainType::Solana, None).await?;

    let private_key = include_str!("./test_private_key.pem");

    // Get a public key for the update (use existing or generate one)
    let private_key = PrivateKey(private_key.to_string());
    let public_key = private_key.get_key().await?.public_key();

    let update_body = UpdateWalletBody {
        owner: Some(OwnerInput::PublicKey(public_key.to_string())),
        ..Default::default()
    };

    let ctx = AuthorizationContext::new();

    // add an owner to the wallet
    let _wallet = debug_response!(client.wallets().update(&wallet_id, &ctx, &update_body)).await?;

    let mut rng = rand::thread_rng();
    let other = SecretKey::<p256::NistP256>::random(&mut rng);

    let update_body = UpdateWalletBody {
        owner: Some(OwnerInput::PublicKey(other.public_key().to_string())),
        ..Default::default()
    };

    // we now have an owner, lets try to update, and expect an error
    if client
        .wallets()
        .update(&wallet_id, &ctx, &update_body)
        .await
        .is_ok()
    {
        panic!("Expected an error when removing an owner");
    }

    let ctx = ctx.push(private_key);

    let wallet = debug_response!(client.wallets().update(&wallet_id, &ctx, &update_body)).await?;

    assert_eq!(wallet.id, wallet_id);
    println!("Updated wallet owner for: {wallet_id}");

    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_wallets_export() -> Result<()> {
    let client = get_test_client()?;
    let user = ensure_test_user(&client).await?;
    let wallet_id =
        get_test_wallet_id_by_type(&client, WalletChainType::Solana, Some(&user.id)).await?;

    let sub = user
        .linked_accounts
        .iter()
        .find_map(|u| match u {
            UserLinkedAccountsItem::CustomJwt(jwt) => Some(&jwt.custom_user_id),
            _ => None,
        })
        .unwrap();

    let jwt = mint_staging_jwt(sub)?;
    let ctx = AuthorizationContext::new().push(JwtUser(client.clone(), jwt));

    let exported = debug_response!(client.wallets().export(&wallet_id, &ctx)).await?;

    println!("Wallet exported successfully {exported:?}");

    Ok(())
}

#[tokio::test]
#[ignore = "openapi reports base as only chain type which is not supported"]
async fn test_wallets_transactions_get() -> Result<()> {
    let client = get_test_client()?;
    let wallet_id = get_test_wallet_id_by_type(&client, WalletChainType::Ethereum, None).await?;

    let asset = WalletTransactionsAsset::String(WalletTransactionsAssetString::Eth);
    let chain = WalletTransactionsChain::Base;

    let transactions = debug_response!(client.wallets().transactions().get(
        &wallet_id,
        &asset,
        chain,
        None,
        Some(10.0)
    ))
    .await?;

    println!(
        "Retrieved {} transactions",
        transactions.into_inner().transactions.len()
    );

    Ok(())
}

#[tokio::test]
async fn test_wallets_balance_get() -> Result<()> {
    let client = get_test_client()?;
    let wallet_id = get_test_wallet_id_by_type(&client, WalletChainType::Solana, None).await?;

    let asset = GetWalletBalanceAsset::String(GetWalletBalanceAssetString::Sol);
    let chain = GetWalletBalanceChain::String(GetWalletBalanceChainString::Solana);

    let balance = debug_response!(
        client
            .wallets()
            .balance()
            .get(&wallet_id, &asset, &chain, None)
    )
    .await?;

    let balance = balance.into_inner();

    println!("Wallet balance retrieved: {balance:?}");

    // new wallet, must be 0
    match balance.balances.as_slice() {
        [GetWalletBalanceResponseBalancesItem { raw_value, .. }] => {
            assert_eq!(*raw_value, "0");
        }
        resp => panic!("Unexpected balance response {resp:?}"),
    }

    Ok(())
}

// Chain-specific signing tests - these would need actual chain-specific implementations
// For now, these are placeholders that demonstrate the testing structure

#[tokio::test]
async fn test_wallets_solana_sign_message() -> Result<()> {
    let client = get_test_client()?;
    let wallet_id = get_test_wallet_id_by_type(&client, WalletChainType::Solana, None).await?;

    let message = "Hello, Solana!";
    let rpc_body = WalletRpcBody::SolanaSignMessageRpcInput(SolanaSignMessageRpcInput {
        address: None,
        chain_type: None,
        method: SolanaSignMessageRpcInputMethod::SignMessage,
        params: SolanaSignMessageRpcInputParams {
            encoding: SolanaSignMessageRpcInputParamsEncoding::Base64,
            message: STANDARD.encode(message),
        },
    });

    let result = debug_response!(client.wallets().rpc(
        &wallet_id,
        &AuthorizationContext::new(),
        None,
        &rpc_body
    ))
    .await?;

    println!("Solana message signed successfully: {result:?}");

    Ok(())
}

#[tokio::test]
async fn test_wallets_solana_sign_transaction() -> Result<()> {
    let client = get_test_client()?;
    let wallet_id = get_test_wallet_id_by_type(&client, WalletChainType::Solana, None).await?;

    // Example base64-encoded Solana transaction (simplified for testing)
    let transaction = "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAEDArczbMia1tLmq7zz4DinMNN0pJ1JtLdqIJPUw3YrGCzYAMHBsgN27lcgB6H2WQvFgyZuJYHa46puOQo9yQ8CVQbd9uHXZaGT2cvhRs7reawctIXtX1s3kTqM9YV+/wCp20C7Wj2aiuk5TReAXo+VTVg8QTHjs0UjNMMKCvpzZ+ABAgEBARU=";

    let rpc_body = WalletRpcBody::SolanaSignTransactionRpcInput(SolanaSignTransactionRpcInput {
        address: None,
        chain_type: None,
        method: SolanaSignTransactionRpcInputMethod::SignTransaction,
        params: SolanaSignTransactionRpcInputParams {
            encoding: SolanaSignTransactionRpcInputParamsEncoding::Base64,
            transaction: transaction.to_string(),
        },
    });

    let result = debug_response!(client.wallets().rpc(
        &wallet_id,
        &AuthorizationContext::new(),
        None,
        &rpc_body
    ))
    .await?;

    println!("Solana transaction signed successfully: {result:?}");

    Ok(())
}

#[tokio::test]
#[traced_test]
#[mark_flaky_tests::flaky]
async fn test_wallets_solana_sign_and_send_transaction() -> Result<()> {
    let client = get_test_client()?;

    let funded_solana_wallet_id =
        std::env::var("FUNDED_SOLANA_WALLET_ID").unwrap_or("av73ge0tfe2xqborg9vjushr".to_string());
    let funded_solana_wallet_address = std::env::var("FUNDED_SOLANA_WALLET_ADDRESS")
        .unwrap_or("DTeASnDsQ1z9Le77MjuiPH4MyqLDWa9vB6R3ZZKRd8d3".to_string());

    // re-using the wallet the java-sdk uses rather than maintaining a parallel wallet
    let funded_solana_wallet_owner_subject_id =
        std::env::var("FUNDED_SOLANA_WALLET_OWNER_SUBJECT_ID")
            .unwrap_or("java-sdk-sub-id".to_string());

    let from_pubkey = Pubkey::from_str(&funded_solana_wallet_address).unwrap();
    let to_pubkey = Pubkey::from_str(&funded_solana_wallet_address).unwrap();
    let lamports = 100;

    let transaction = Transaction::new_with_payer(
        &[solana_system_interface::instruction::transfer(
            &from_pubkey,
            &to_pubkey,
            lamports,
        )],
        None,
    );

    // bincode then base64 encode
    let transaction = STANDARD.encode(bincode::serialize(&transaction).unwrap());

    let rpc_body =
        WalletRpcBody::SolanaSignAndSendTransactionRpcInput(SolanaSignAndSendTransactionRpcInput {
            address: None,
            caip2: SolanaSignAndSendTransactionRpcInputCaip2::from_str(
                "solana:EtWTRABZaYq6iMfeYKouRu166VU2xqa1", // devnet
            )
            .unwrap(),
            chain_type: None,
            method: SolanaSignAndSendTransactionRpcInputMethod::SignAndSendTransaction,
            params: SolanaSignAndSendTransactionRpcInputParams {
                encoding: SolanaSignAndSendTransactionRpcInputParamsEncoding::Base64,
                transaction,
            },
            sponsor: Some(false),
        });

    let ctx = AuthorizationContext::new().push(JwtUser(
        client.clone(),
        mint_staging_jwt(&funded_solana_wallet_owner_subject_id)?,
    ));

    let result = debug_response!(client.wallets().rpc(
        &funded_solana_wallet_id,
        &ctx,
        None,
        &rpc_body
    ))
    .await?;

    println!("Solana transaction signed and sent successfully: {result:?}");

    Ok(())
}

#[tokio::test]
#[traced_test]
#[mark_flaky_tests::flaky]
async fn test_wallets_ethereum_sign_message() -> Result<()> {
    let client = get_test_client()?;
    let wallet_id = get_test_wallet_id_by_type(&client, WalletChainType::Ethereum, None).await?;

    let rpc_body: WalletRpcBody =
        WalletRpcBody::EthereumPersonalSignRpcInput(EthereumPersonalSignRpcInput {
            address: Some("0xdadB0d80178819F2319190D340ce9A924f783711".to_string()),
            chain_type: Some(EthereumPersonalSignRpcInputChainType::Ethereum),
            method: EthereumPersonalSignRpcInputMethod::PersonalSign,
            params: EthereumPersonalSignRpcInputParams {
                encoding: EthereumPersonalSignRpcInputParamsEncoding::Utf8,
                message: "Hello world".to_string(),
            },
        });

    let result = debug_response!(client.wallets().rpc(
        &wallet_id,
        &AuthorizationContext::new(),
        None,
        &rpc_body
    ))
    .await?;

    // Just verify we got a valid response
    println!("Ethereum message signed via RPC: {result:?}");

    Ok(())
}

#[tokio::test]
async fn test_wallets_ethereum_sign_typed_data() -> Result<()> {
    let client = get_test_client()?;
    let wallet_id = get_test_wallet_id_by_type(&client, WalletChainType::Ethereum, None).await?;

    let rpc_body = WalletRpcBody::EthereumSignTypedDataRpcInput(EthereumSignTypedDataRpcInput {
        address: None,
        chain_type: None,
        method: EthereumSignTypedDataRpcInputMethod::EthSignTypedDataV4,
        params: EthereumSignTypedDataRpcInputParams {
            typed_data: EthereumSignTypedDataRpcInputParamsTypedData {
                domain: serde_json::Map::from_iter([
                    ("name".to_string(), serde_json::json!("DApp Mail")),
                    ("version".to_string(), serde_json::json!("1")),
                    ("chainId".to_string(), serde_json::json!("0x3e8")),
                    (
                        "verifyingContract".to_string(),
                        serde_json::json!("0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"),
                    ),
                ]),
                message: serde_json::Map::from_iter([
                    (
                        "from".to_string(),
                        serde_json::json!({
                            "name": "Alice",
                            "wallet": "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
                        }),
                    ),
                    (
                        "to".to_string(),
                        serde_json::json!({
                            "name": "Bob",
                            "wallet": "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
                        }),
                    ),
                    ("contents".to_string(), serde_json::json!("Hello, Bob!")),
                ]),
                primary_type: "Mail".to_string(),
                types: [
                    (
                        "EIP712Domain".to_string(),
                        vec![
                            EthereumSignTypedDataRpcInputParamsTypedDataTypesValueItem {
                                name: "name".to_string(),
                                type_: "string".to_string(),
                            },
                            EthereumSignTypedDataRpcInputParamsTypedDataTypesValueItem {
                                name: "version".to_string(),
                                type_: "string".to_string(),
                            },
                            EthereumSignTypedDataRpcInputParamsTypedDataTypesValueItem {
                                name: "chainId".to_string(),
                                type_: "uint256".to_string(),
                            },
                            EthereumSignTypedDataRpcInputParamsTypedDataTypesValueItem {
                                name: "verifyingContract".to_string(),
                                type_: "address".to_string(),
                            },
                        ],
                    ),
                    (
                        "Person".to_string(),
                        vec![
                            EthereumSignTypedDataRpcInputParamsTypedDataTypesValueItem {
                                name: "name".to_string(),
                                type_: "string".to_string(),
                            },
                            EthereumSignTypedDataRpcInputParamsTypedDataTypesValueItem {
                                name: "wallet".to_string(),
                                type_: "address".to_string(),
                            },
                        ],
                    ),
                    (
                        "Mail".to_string(),
                        vec![
                            EthereumSignTypedDataRpcInputParamsTypedDataTypesValueItem {
                                name: "from".to_string(),
                                type_: "Person".to_string(),
                            },
                            EthereumSignTypedDataRpcInputParamsTypedDataTypesValueItem {
                                name: "to".to_string(),
                                type_: "Person".to_string(),
                            },
                            EthereumSignTypedDataRpcInputParamsTypedDataTypesValueItem {
                                name: "contents".to_string(),
                                type_: "string".to_string(),
                            },
                        ],
                    ),
                ]
                .into(),
            },
        },
    });

    let result = debug_response!(client.wallets().rpc(
        &wallet_id,
        &AuthorizationContext::new(),
        None,
        &rpc_body
    ))
    .await?;

    // Just verify we got a valid response
    println!("Ethereum typed data signed via RPC: {result:?}");

    Ok(())
}

#[tokio::test]
async fn test_wallets_ethereum_sign_secp256k1() -> Result<()> {
    let client = get_test_client()?;
    let wallet_id = get_test_wallet_id_by_type(&client, WalletChainType::Ethereum, None).await?;

    let rpc_body = WalletRpcBody::EthereumSecp256k1SignRpcInput(EthereumSecp256k1SignRpcInput {
        address: None,
        chain_type: None,
        method: privy_rs::generated::types::EthereumSecp256k1SignRpcInputMethod::Secp256k1Sign,
        params: privy_rs::generated::types::EthereumSecp256k1SignRpcInputParams {
            hash: "0x12345678".to_string(),
        },
    });

    let result = debug_response!(client.wallets().rpc(
        &wallet_id,
        &AuthorizationContext::new(),
        None,
        &rpc_body
    ))
    .await?;

    // Just verify we got a valid response
    println!("Ethereum secp256k1 signature via RPC: {result:?}");

    Ok(())
}

#[tokio::test]
async fn test_wallets_ethereum_sign_7702_authorization() -> Result<()> {
    let client = get_test_client()?;
    let wallet_id = get_test_wallet_id_by_type(&client, WalletChainType::Ethereum, None).await?;

    let rpc_body = WalletRpcBody::EthereumSign7702AuthorizationRpcInput(
        EthereumSign7702AuthorizationRpcInput {
            address: None,
            chain_type: None,
            method: EthereumSign7702AuthorizationRpcInputMethod::EthSign7702Authorization,
            params: EthereumSign7702AuthorizationRpcInputParams {
                chain_id: EthereumSign7702AuthorizationRpcInputParamsChainId::Integer(1),
                contract: "0x1234567890abcdef1234567890abcdef12345678".into(),
                nonce: None,
            },
        },
    );

    let result = debug_response!(client.wallets().rpc(
        &wallet_id,
        &AuthorizationContext::new(),
        None,
        &rpc_body
    ))
    .await?;

    // Just verify we got a valid response
    println!("Ethereum 7702 authorization signed via RPC: {result:?}");

    Ok(())
}

#[tokio::test]
#[ignore = "failing with 'unexpected error occurred, please try again later'"]
async fn test_wallets_ethereum_sign_transaction() -> Result<()> {
    let client = get_test_client()?;
    let wallet_id = get_test_wallet_id_by_type(&client, WalletChainType::Ethereum, None).await?;

    let rpc_body = WalletRpcBody::EthereumSignTransactionRpcInput(
        privy_rs::generated::types::EthereumSignTransactionRpcInput {
            address: Some("0xdadB0d80178819F2319190D340ce9A924f783711".to_string()),
            chain_type: Some(privy_rs::generated::types::EthereumSignTransactionRpcInputChainType::Ethereum),
            method: privy_rs::generated::types::EthereumSignTransactionRpcInputMethod::EthSignTransaction,
            params: privy_rs::generated::types::EthereumSignTransactionRpcInputParams {
                transaction: privy_rs::generated::types::EthereumSignTransactionRpcInputParamsTransaction {
                    chain_id: None, data: None, from: None, gas_limit: None, gas_price: None, max_fee_per_gas: None, max_priority_fee_per_gas: None, nonce: None, to: None, type_: None, value: None
                }
            },
        },
    );

    let result = debug_response!(client.wallets().rpc(
        &wallet_id,
        &AuthorizationContext::new(),
        None,
        &rpc_body
    ))
    .await?;

    // Just verify we got a valid response
    println!("Ethereum transaction signed via RPC: {result:?}");

    Ok(())
}

#[tokio::test]
#[traced_test]
// #[mark_flaky_tests::flaky]
async fn test_wallets_ethereum_send_transaction() -> Result<()> {
    let client = get_test_client()?;

    let funded_ethereum_wallet_id = std::env::var("FUNDED_ETHEREUM_WALLET_ID")
        .unwrap_or("xdeor1731y8gme1utsldxynv".to_string());
    let funded_ethereum_wallet_address = std::env::var("FUNDED_ETHEREUM_WALLET_ADDRESS")
        .unwrap_or("0xd606c61A275328395E15375A0139Ef92DA9cC280".to_string());
    let funded_ethereum_wallet_owner_subject_id =
        std::env::var("FUNDED_WALLETS_OWNER_SUBJECT_ID").unwrap_or("java-sdk-sub-id".to_string());

    let rpc_body = WalletRpcBody::EthereumSendTransactionRpcInput(
        privy_rs::generated::types::EthereumSendTransactionRpcInput {
            address: None,
            caip2: "eip155:11155111".parse().unwrap(),
            chain_type: Some(EthereumSendTransactionRpcInputChainType::Ethereum),
            method: EthereumSendTransactionRpcInputMethod::EthSendTransaction,
            params: EthereumSendTransactionRpcInputParams {
                transaction: EthereumSendTransactionRpcInputParamsTransaction {
                    to: Some("0xa8BABAc2A6d66Db720D8271e7a85fB0CB78A5377".to_string()),
                    value: Some(
                        EthereumSendTransactionRpcInputParamsTransactionValue::Integer(100),
                    ),
                    chain_id: None,
                    from: Some(funded_ethereum_wallet_address.clone()),
                    max_fee_per_gas: None,
                    max_priority_fee_per_gas: None,
                    nonce: None,
                    type_: None,
                    data: Some("0x".to_string()),
                    // data: None,
                    gas_limit: None,
                    gas_price: None,
                },
            },
            sponsor: Some(false),
        },
    );

    let ctx = AuthorizationContext::new().push(JwtUser(
        client.clone(),
        mint_staging_jwt(&funded_ethereum_wallet_owner_subject_id)?,
    ));

    let result = debug_response!(client.wallets().rpc(
        &funded_ethereum_wallet_id,
        &ctx,
        None,
        &rpc_body
    ))
    .await?;

    // Just verify we got a valid response
    println!("Ethereum transaction sent via RPC: {result:?}");

    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_ethereum_wallet_import() -> Result<()> {
    let client = get_test_client()?;

    let mut rng = rand::thread_rng();
    let (secret, public) = secp256k1::generate_keypair(&mut rng);

    let secret: String = (&secret[..]).encode_hex();

    // strip 04 from the start
    let public = &public.serialize_uncompressed()[1..];

    // address is the last 20 bytes of the keccak256 hash of the public key
    let address: String = format!(
        "0x{}",
        sha3::Keccak256::digest(public)[12..]
            .iter()
            .encode_hex::<String>()
    );

    tracing::info!("Generated secret key: {:?}", secret);
    tracing::info!("Generated public key: {:?}", public);
    tracing::info!("Generated address: {:?}", address);

    // Initialize wallet import for Ethereum
    let imported_wallet = client
        .wallets()
        .import(
            address.clone(),
            &secret,
            WalletImportSupportedChains::Ethereum,
            None,   // owner
            vec![], // policy_ids
            vec![], // additional_signers
        )
        .await?
        .into_inner();

    // Verify the imported wallet has the correct address and chain type
    assert_eq!(imported_wallet.address.to_lowercase(), address);
    assert_eq!(imported_wallet.chain_type, WalletChainType::Ethereum);

    println!(
        "Successfully imported Ethereum wallet with ID: {}",
        imported_wallet.id
    );
    println!("Wallet address: {}", imported_wallet.address);

    Ok(())
}
