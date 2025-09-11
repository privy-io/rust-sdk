use std::{collections::HashMap, time::SystemTime};

use anyhow::Result;
use privy_rust::generated::types::{
    CreateUserBody, CreateUserWalletBody, CreateUserWalletBodyWalletsItem,
    CreateUserWalletBodyWalletsItemChainType, CustomMetadataValue, LinkedAccountEmailInput,
    LinkedAccountEmailInputType, LinkedAccountInput, SearchUsersBody, UpdateUserCustomMetadataBody,
    UserLinkedAccountsItem,
};
use tracing_test::traced_test;

mod common;

#[tokio::test]
async fn test_users_list() -> Result<()> {
    let client = common::get_test_client()?;

    let result = client.users().list(None, Some(1.0)).await?;

    println!("Users list response: {:?}", result);

    Ok(())
}

#[tokio::test]
async fn test_users_create() -> Result<()> {
    let client = common::get_test_client()?;

    let unique_email = format!("test-{}@example.com", chrono::Utc::now().timestamp());
    let create_body = CreateUserBody {
        linked_accounts: vec![LinkedAccountInput::EmailInput(LinkedAccountEmailInput {
            address: unique_email.clone(),
            type_: LinkedAccountEmailInputType::Email,
        })],
        custom_metadata: None,
        wallets: vec![],
    };

    let result = debug_response!(client.users().create(&create_body)).await?;

    println!("Created user: {:?}", result);
    assert!(result.into_inner().linked_accounts.iter().any(|a| match a {
        UserLinkedAccountsItem::Email(e) => e.address == unique_email,
        _ => false,
    }));

    Ok(())
}

#[tokio::test]
async fn test_users_get() -> Result<()> {
    let client = common::get_test_client()?;

    let user = common::ensure_test_user(&client).await?;

    let result = client.users().get(&user.id).await?;
    println!("Retrieved user: {:?}", result);
    assert_eq!(result.into_inner().id, user.id);

    Ok(())
}

#[tokio::test]
async fn test_users_delete() -> Result<()> {
    let client = common::get_test_client()?;

    // First create a user to delete
    let unique_email = format!("delete-test-{}@example.com", chrono::Utc::now().timestamp());
    let create_body = CreateUserBody {
        linked_accounts: vec![LinkedAccountInput::EmailInput(LinkedAccountEmailInput {
            address: unique_email,
            type_: LinkedAccountEmailInputType::Email,
        })],
        custom_metadata: None,
        wallets: vec![],
    };

    let create_result = client.users().create(&create_body).await?;
    let user_id = create_result.into_inner().id;

    // Now delete the user
    let result = client.users().delete(&user_id).await?;

    println!("Deleted user response: {:?}", result);

    Ok(())
}

#[tokio::test]
async fn test_users_set_custom_metadata() -> Result<()> {
    let client = common::get_test_client()?;
    let user_id = common::ensure_test_user(&client).await?.id;

    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    let mut custom_metadata = HashMap::new();
    custom_metadata.insert(
        "test_key".to_string(),
        CustomMetadataValue::String("test_value".to_string()),
    );
    custom_metadata.insert("timestamp".to_string(), CustomMetadataValue::Number(time));

    let metadata_body = UpdateUserCustomMetadataBody {
        custom_metadata: custom_metadata.into(),
    };

    let result = client
        .users()
        .set_custom_metadata(&user_id, &metadata_body)
        .await;
    assert!(
        result.is_ok(),
        "Failed to set custom metadata: {:?}",
        result
    );

    let metadata_response = result.unwrap();
    println!("Set custom metadata response: {:?}", metadata_response);

    Ok(())
}

#[tokio::test]
async fn test_users_pregenerate_wallets() -> Result<()> {
    let client = common::get_test_client()?;
    let user_id = common::ensure_test_user(&client).await?.id;

    let pregenerate_body = CreateUserWalletBody {
        wallets: vec![CreateUserWalletBodyWalletsItem {
            chain_type: CreateUserWalletBodyWalletsItemChainType::Ethereum,
            additional_signers: vec![],
            create_smart_wallet: None,
            policy_ids: vec![],
        }],
    };

    let result = debug_response!(
        client
            .users()
            .pregenerate_wallets(&user_id, &pregenerate_body)
    )
    .await;
    assert!(
        result.is_ok(),
        "Failed to pregenerate wallets: {:?}",
        result
    );

    let pregenerate_response = result.unwrap();
    println!("Pregenerate wallets response: {:?}", pregenerate_response);

    Ok(())
}

#[tokio::test]
#[traced_test]
#[ignore = "throwing an error despite matching exactly the curl request"]
async fn test_users_search() -> Result<()> {
    let client = common::get_test_client()?;

    let search_body = SearchUsersBody::Variant0 {
        search_term: "rust-sdk".to_string(),
    };

    tracing::debug!(
        "Search users body: {:?}",
        serde_json::to_string(&search_body)
    );

    let result = debug_response!(client.users().search(&search_body)).await?;

    println!("Search users response: {:?}", result);

    Ok(())
}
