#![allow(unnameable_test_items)]

use std::str::FromStr;

use anyhow::Result;
use base64::{Engine, engine::general_purpose::STANDARD};
use common::{get_test_client, get_test_wallet_id_by_type, mint_staging_jwt};
use privy_rs::{AuthorizationContext, JwtUser, generated::types::*};
use solana_sdk::{pubkey::Pubkey, transaction::Transaction};
use tracing_test::traced_test;

mod common;

#[tokio::test]
#[mark_flaky_tests::flaky]
async fn test_solana_sign_message() -> Result<()> {
    let client = get_test_client()?;
    let wallet_id = get_test_wallet_id_by_type(&client, WalletChainType::Solana, None).await?;

    let message = "Hello, Solana!";
    let rpc_body = WalletRpcRequestBody::SolanaSignMessageRpcInput(SolanaSignMessageRpcInput {
        address: None,
        chain_type: None,
        method: SolanaSignMessageRpcInputMethod::SignMessage,
        params: SolanaSignMessageRpcInputParams {
            encoding: SolanaSignMessageRpcInputParamsEncoding::Base64,
            message: STANDARD.encode(message).parse().unwrap(),
        },
        wallet_id: None,
    });

    let result = debug_response!(client.wallets().rpc(
        &wallet_id,
        &AuthorizationContext::new(),
        None,
        &rpc_body
    ))
    .await?;

    println!("Solana message signed successfully: {result:?}");

    match result.into_inner() {
        WalletRpcResponse::SolanaSignMessageRpcResponse(_) => {}
        _ => panic!("Expected SolanaSignMessageRpcResponse"),
    }

    Ok(())
}

#[tokio::test]
#[mark_flaky_tests::flaky]
async fn test_solana_sign_transaction() -> Result<()> {
    let client = get_test_client()?;
    let wallet_id = get_test_wallet_id_by_type(&client, WalletChainType::Solana, None).await?;

    let transaction = "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAEDArczbMia1tLmq7zz4DinMNN0pJ1JtLdqIJPUw3YrGCzYAMHBsgN27lcgB6H2WQvFgyZuJYHa46puOQo9yQ8CVQbd9uHXZaGT2cvhRs7reawctIXtX1s3kTqM9YV+/wCp20C7Wj2aiuk5TReAXo+VTVg8QTHjs0UjNMMKCvpzZ+ABAgEBARU=";

    let rpc_body = WalletRpcRequestBody::SolanaSignTransactionRpcInput(SolanaSignTransactionRpcInput {
        address: None,
        chain_type: None,
        method: SolanaSignTransactionRpcInputMethod::SignTransaction,
        params: SolanaSignTransactionRpcInputParams {
            encoding: SolanaSignTransactionRpcInputParamsEncoding::Base64,
            transaction: transaction.parse().unwrap(),
        },
        wallet_id: None,
    });

    let result = debug_response!(client.wallets().rpc(
        &wallet_id,
        &AuthorizationContext::new(),
        None,
        &rpc_body
    ))
    .await?;

    println!("Solana transaction signed successfully: {result:?}");

    match result.into_inner() {
        WalletRpcResponse::SolanaSignTransactionRpcResponse(_) => {}
        _ => panic!("Expected SolanaSignTransactionRpcResponse"),
    }

    Ok(())
}

#[tokio::test]
#[traced_test]
#[ignore = "ignore tests that attempt to move funds as wallets are not funded"]
async fn test_solana_sign_and_send_transaction() -> Result<()> {
    let client = get_test_client()?;

    let funded_solana_wallet_id =
        std::env::var("FUNDED_SOLANA_WALLET_ID").unwrap_or("av73ge0tfe2xqborg9vjushr".to_string());
    let funded_solana_wallet_address = std::env::var("FUNDED_SOLANA_WALLET_ADDRESS")
        .unwrap_or("DTeASnDsQ1z9Le77MjuiPH4MyqLDWa9vB6R3ZZKRd8d3".to_string());

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

    let transaction = STANDARD.encode(bincode::serialize(&transaction).unwrap());

    let rpc_body =
        WalletRpcRequestBody::SolanaSignAndSendTransactionRpcInput(SolanaSignAndSendTransactionRpcInput {
            address: None,
            caip2: "solana:EtWTRABZaYq6iMfeYKouRu166VU2xqa1".parse().unwrap(),
            chain_type: None,
            method: SolanaSignAndSendTransactionRpcInputMethod::SignAndSendTransaction,
            params: SolanaSignAndSendTransactionRpcInputParams {
                encoding: SolanaSignAndSendTransactionRpcInputParamsEncoding::Base64,
                transaction: transaction.parse().unwrap(),
            },
            sponsor: Some(false),
            wallet_id: None,
            optimistic_broadcast: None,
            reference_id: None,
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

    match result.into_inner() {
        WalletRpcResponse::SolanaSignAndSendTransactionRpcResponse(_) => {}
        _ => panic!("Expected SolanaSignAndSendTransactionRpcResponse"),
    }

    Ok(())
}

#[tokio::test]
#[traced_test]
#[ignore = "ignore tests that attempt to move funds as wallets are not funded"]
async fn test_solana_sign_and_send_transaction_with_options_sponsored() -> Result<()> {
    let client = get_test_client()?;

    let funded_solana_wallet_id =
        std::env::var("FUNDED_SOLANA_WALLET_ID").expect("FUNDED_SOLANA_WALLET_ID must be set");
    let funded_solana_wallet_address = std::env::var("FUNDED_SOLANA_WALLET_ADDRESS")
        .expect("FUNDED_SOLANA_WALLET_ADDRESS must be set");
    let funded_solana_wallet_owner_subject_id =
        std::env::var("FUNDED_SOLANA_WALLET_OWNER_SUBJECT_ID")
            .expect("FUNDED_SOLANA_WALLET_OWNER_SUBJECT_ID must be set");

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

    let transaction = STANDARD.encode(bincode::serialize(&transaction).unwrap());

    let options = privy_rs::SignAndSendTransactionOptions::new().with_sponsor(true);

    let ctx = AuthorizationContext::new().push(JwtUser(
        client.clone(),
        mint_staging_jwt(&funded_solana_wallet_owner_subject_id)?,
    ));

    let result = debug_response!(
        client.wallets().solana().sign_and_send_transaction_with_options(
            &funded_solana_wallet_id,
            "solana:EtWTRABZaYq6iMfeYKouRu166VU2xqa1",
            &transaction,
            &ctx,
            None,
            &options,
        )
    )
    .await?;

    println!("Solana sponsored transaction sent: {result:?}");

    match result.into_inner() {
        WalletRpcResponse::SolanaSignAndSendTransactionRpcResponse(_) => {}
        _ => panic!("Expected SolanaSignAndSendTransactionRpcResponse"),
    }

    Ok(())
}
