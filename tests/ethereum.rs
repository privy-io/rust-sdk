#![allow(unnameable_test_items)]

use anyhow::Result;
use common::{get_test_client, get_test_wallet_id_by_type, mint_staging_jwt};
use hex::ToHex;
use privy_rs::{AuthorizationContext, JwtUser, generated::types::*};
use sha2::Digest;
use tracing_test::traced_test;

mod common;

#[tokio::test]
#[traced_test]
#[mark_flaky_tests::flaky]
async fn test_ethereum_sign_message() -> Result<()> {
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

    println!("Ethereum message signed via RPC: {result:?}");

    match result.into_inner() {
        WalletRpcResponse::EthereumPersonalSignRpcResponse(_) => {}
        _ => panic!("Expected EthereumPersonalSignRpcResponse"),
    }

    Ok(())
}

#[tokio::test]
async fn test_ethereum_sign_typed_data() -> Result<()> {
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

    println!("Ethereum typed data signed via RPC: {result:?}");

    match result.into_inner() {
        WalletRpcResponse::EthereumSignTypedDataRpcResponse(_) => {}
        _ => panic!("Expected EthereumSignTypedDataRpcResponse"),
    }

    Ok(())
}

#[tokio::test]
async fn test_ethereum_sign_secp256k1() -> Result<()> {
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

    println!("Ethereum secp256k1 signature via RPC: {result:?}");

    match result.into_inner() {
        WalletRpcResponse::EthereumSecp256k1SignRpcResponse(_) => {}
        _ => panic!("Expected EthereumSecp256k1SignRpcResponse"),
    }

    Ok(())
}

#[tokio::test]
async fn test_ethereum_sign_7702_authorization() -> Result<()> {
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

    println!("Ethereum 7702 authorization signed via RPC: {result:?}");

    match result.into_inner() {
        WalletRpcResponse::EthereumSign7702AuthorizationRpcResponse(_) => {}
        _ => panic!("Expected EthereumSign7702AuthorizationRpcResponse"),
    }

    Ok(())
}

#[tokio::test]
#[ignore = "failing with 'unexpected error occurred, please try again later'"]
async fn test_ethereum_sign_transaction() -> Result<()> {
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

    println!("Ethereum transaction signed via RPC: {result:?}");

    match result.into_inner() {
        WalletRpcResponse::EthereumSignTransactionRpcResponse(_) => {}
        _ => panic!("Expected EthereumSignTransactionRpcResponse"),
    }

    Ok(())
}

#[tokio::test]
#[traced_test]
#[ignore = "ignore tests that attempt to move funds as wallets are not funded"]
async fn test_ethereum_send_transaction() -> Result<()> {
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

    println!("Ethereum transaction sent via RPC: {result:?}");

    match result.into_inner() {
        WalletRpcResponse::EthereumSendTransactionRpcResponse(_) => {}
        _ => panic!("Expected EthereumSendTransactionRpcResponse"),
    }

    Ok(())
}

#[tokio::test]
#[traced_test]
#[ignore = "ignore tests that attempt to move funds as wallets are not funded"]
async fn test_ethereum_send_transaction_with_options_sponsored() -> Result<()> {
    let client = get_test_client()?;

    let funded_ethereum_wallet_id =
        std::env::var("FUNDED_ETHEREUM_WALLET_ID").expect("FUNDED_ETHEREUM_WALLET_ID must be set");
    let funded_ethereum_wallet_address = std::env::var("FUNDED_ETHEREUM_WALLET_ADDRESS")
        .expect("FUNDED_ETHEREUM_WALLET_ADDRESS must be set");
    let funded_ethereum_wallet_owner_subject_id = std::env::var("FUNDED_WALLETS_OWNER_SUBJECT_ID")
        .expect("FUNDED_WALLETS_OWNER_SUBJECT_ID must be set");
    let recipient_address = std::env::var("ETHEREUM_RECIPIENT_ADDRESS")
        .expect("ETHEREUM_RECIPIENT_ADDRESS must be set");

    let transaction = EthereumSendTransactionRpcInputParamsTransaction {
        to: Some(recipient_address),
        value: Some(EthereumSendTransactionRpcInputParamsTransactionValue::Integer(100)),
        chain_id: None,
        from: Some(funded_ethereum_wallet_address.clone()),
        max_fee_per_gas: None,
        max_priority_fee_per_gas: None,
        nonce: None,
        type_: None,
        data: Some("0x".to_string()),
        gas_limit: None,
        gas_price: None,
    };

    let options = privy_rs::SendTransactionOptions::new().with_sponsor(true);

    let ctx = AuthorizationContext::new().push(JwtUser(
        client.clone(),
        mint_staging_jwt(&funded_ethereum_wallet_owner_subject_id)?,
    ));

    let result = debug_response!(client.wallets().ethereum().send_transaction_with_options(
        &funded_ethereum_wallet_id,
        "eip155:11155111",
        transaction,
        &ctx,
        None,
        &options,
    ))
    .await?;

    println!("Ethereum sponsored transaction sent: {result:?}");

    match result.into_inner() {
        WalletRpcResponse::EthereumSendTransactionRpcResponse(_) => {}
        _ => panic!("Expected EthereumSendTransactionRpcResponse"),
    }

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

    let imported_wallet = client
        .wallets()
        .import(
            address.clone(),
            &secret,
            WalletImportSupportedChains::Ethereum,
            None,
            vec![],
            vec![],
        )
        .await?
        .into_inner();

    assert_eq!(imported_wallet.address.to_lowercase(), address);
    assert_eq!(imported_wallet.chain_type, WalletChainType::Ethereum);

    println!(
        "Successfully imported Ethereum wallet with ID: {}",
        imported_wallet.id
    );

    Ok(())
}
