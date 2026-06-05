#![allow(unnameable_test_items)] // flaky test annotation is causing a lint issue

use anyhow::Result;
use common::{ensure_test_user, get_test_client, get_test_wallet_id_by_type, mint_staging_jwt};
use p256::elliptic_curve::SecretKey;
use privy_rs::{
    AuthorizationContext, IntoKey, JwtUser, PrivateKey, PrivyHpke, generated::types::*,
};
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
            LinkedAccount::CustomJwt(jwt) => Some(jwt.custom_user_id.to_owned()),
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
        params: RawSignParams::Hash {
            hash: "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".to_string(),
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
    let private_key = PrivateKey::new(private_key.to_string());
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
            LinkedAccount::CustomJwt(jwt) => Some(&jwt.custom_user_id),
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
        Some(10.0),
        None
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

