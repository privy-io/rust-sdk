use std::time::SystemTime;

use anyhow::Result;
use privy_rs::{
    AuthorizationContext, JwtUser,
    generated::types::{
        CreateKeyQuorumBody, CreateKeyQuorumBodyDisplayName, UpdateKeyQuorumBody,
        UpdateKeyQuorumBodyDisplayName, UserLinkedAccountsItem,
    },
};

mod common;

#[tokio::test]
async fn test_key_quorums_create() -> Result<()> {
    let client = common::get_test_client()?;

    // Create a test user to associate with the key quorum
    let test_user = common::ensure_test_user(&client).await?;

    // Create a unique display name for the key quorum
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    let display_name =
        CreateKeyQuorumBodyDisplayName::try_from(format!("Test KQ {timestamp}").as_str())?;

    let create_body = CreateKeyQuorumBody {
        authorization_threshold: Some(1.0),
        display_name: Some(display_name),
        public_keys: vec![],
        user_ids: vec![test_user.id.clone()],
    };

    let result = client.key_quorums().create(&create_body).await?;

    println!("Created key quorum: {result:?}");

    let key_quorum = result.into_inner();
    assert_eq!(key_quorum.authorization_threshold, Some(1.0));
    assert!(key_quorum.user_ids.contains(&test_user.id));

    Ok(())
}

#[tokio::test]
async fn test_key_quorums_get() -> Result<()> {
    let client = common::get_test_client()?;

    // Create a test user for the key quorum
    let test_user = common::ensure_test_user(&client).await?;

    // First create a key quorum to retrieve
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    let display_name =
        CreateKeyQuorumBodyDisplayName::try_from(format!("Test Get KQ {timestamp}").as_str())?;

    let create_body = CreateKeyQuorumBody {
        authorization_threshold: Some(1.0),
        display_name: Some(display_name),
        public_keys: vec![],
        user_ids: vec![test_user.id.clone()],
    };

    let created = client.key_quorums().create(&create_body).await?;
    let key_quorum_id = created.into_inner().id;

    // Now test getting the key quorum
    let result = client.key_quorums().get(&key_quorum_id).await?;

    println!("Retrieved key quorum: {result:?}");

    let key_quorum = result.into_inner();
    assert_eq!(key_quorum.id, key_quorum_id);
    assert_eq!(key_quorum.authorization_threshold, Some(1.0));

    Ok(())
}

#[tokio::test]
async fn test_key_quorums_update_with_auth_context() -> Result<()> {
    let client = common::get_test_client()?;

    // Create a test user for the key quorum
    let test_user = common::ensure_test_user(&client).await?;

    let custom_sub = test_user
        .linked_accounts
        .iter()
        .find_map(|la| match la {
            UserLinkedAccountsItem::CustomJwt(cj) => Some(&cj.custom_user_id),
            _ => None,
        })
        .unwrap();

    let jwt = common::mint_staging_jwt(custom_sub)?;

    // First create a key quorum to update
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    let display_name =
        CreateKeyQuorumBodyDisplayName::try_from(format!("Test Update KQ {timestamp}").as_str())?;

    let create_body = CreateKeyQuorumBody {
        authorization_threshold: Some(1.0),
        display_name: Some(display_name),
        public_keys: vec![],
        user_ids: vec![test_user.id.clone()],
    };

    let created = debug_response!(client.key_quorums().create(&create_body)).await?;
    let key_quorum_id = created.into_inner().id;

    // Set up authorization context for the update
    let ctx = AuthorizationContext::new();
    ctx.push(JwtUser(client.clone(), jwt));

    // Update the key quorum
    let updated_display_name = UpdateKeyQuorumBodyDisplayName::try_from(
        format!("Updated Test KQ {timestamp}").as_str(),
    )?;

    let update_body = UpdateKeyQuorumBody {
        authorization_threshold: Some(1.0),
        display_name: Some(updated_display_name),
        public_keys: vec![
            "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEx4aoeD72yykviK+f/ckqE2CItVIG\n1rCnvC3/XZ1HgpOcMEMialRmTrqIK4oZlYd1RfxU3za/C9yjhboIuoPD3g==\n-----END PUBLIC KEY-----".to_string(),
        ],
        user_ids: vec![],
    };

    let result = debug_response!(
        client
            .key_quorums()
            .update(&key_quorum_id, &ctx, &update_body)
    )
    .await?;

    println!("Updated key quorum: {result:?}");

    let key_quorum = result.into_inner();

    assert_eq!(key_quorum.authorization_threshold, Some(1.0));

    // we expect the user id to be still there, and the public key to have been added
    assert_eq!(key_quorum.user_ids.len(), 1);
    assert_eq!(key_quorum.authorization_keys.len(), 1);

    Ok(())
}

#[tokio::test]
async fn test_key_quorums_delete() -> Result<()> {
    let client = common::get_test_client()?;

    // Create a test user for the key quorum
    let test_user = common::ensure_test_user(&client).await?;

    let custom_sub = test_user
        .linked_accounts
        .iter()
        .find_map(|la| match la {
            UserLinkedAccountsItem::CustomJwt(cj) => Some(&cj.custom_user_id),
            _ => None,
        })
        .unwrap();

    let jwt = common::mint_staging_jwt(custom_sub)?;

    // First create a key quorum to delete
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    let display_name =
        CreateKeyQuorumBodyDisplayName::try_from(format!("Test Delete KQ {timestamp}").as_str())?;

    let create_body = CreateKeyQuorumBody {
        authorization_threshold: Some(1.0),
        display_name: Some(display_name),
        public_keys: vec![],
        user_ids: vec![test_user.id.clone()],
    };

    let created = client.key_quorums().create(&create_body).await?;
    let key_quorum_id = created.into_inner().id;

    // Set up authorization context for the delete
    let ctx = AuthorizationContext::new();
    ctx.push(JwtUser(client.clone(), jwt));

    // Delete the key quorum
    let result = debug_response!(client.key_quorums().delete(&key_quorum_id, &ctx)).await?;

    println!("Deleted key quorum: {result:?}");

    // Verify deletion by trying to get the key quorum (should fail)
    let get_result = client.key_quorums().get(&key_quorum_id).await;
    assert!(get_result.is_err(), "Key quorum should be deleted");

    Ok(())
}
