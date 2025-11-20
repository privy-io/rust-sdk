use std::str::FromStr;

use anyhow::Result;
use privy_rs::{AuthorizationContext, IntoKey, PrivateKey, generated::types::*};

mod common;

#[tokio::test]
async fn test_policies_create() -> Result<()> {
    let client = common::get_test_client()?;

    let unique_name = format!("test-policy-{}", chrono::Utc::now().timestamp());
    let policy_body = CreatePolicyBody {
        chain_type: PolicyChainType::Solana,
        name: CreatePolicyBodyName::try_from(unique_name).unwrap(),
        owner: None,
        owner_id: None,
        rules: vec![PolicyRuleRequestBody {
            action: PolicyAction::Allow,
            conditions: vec![PolicyCondition::SolanaSystemProgramInstructionCondition(
                SolanaSystemProgramInstructionCondition {
                    field: SolanaSystemProgramInstructionConditionField::TransferLamports,
                    field_source: SolanaSystemProgramInstructionConditionFieldSource::SolanaSystemProgramInstruction,
                    operator: ConditionOperator::Lt,
                    value: ConditionValue::String("1000000".to_string()),
                }
            )],
            method: PolicyMethod::SignTransaction,
            name: PolicyRuleRequestBodyName::try_from("test-rule").unwrap(),
        }],
        version: CreatePolicyBodyVersion::try_from("1.0").unwrap(),
    };

    let result = debug_response!(client.policies().create(None, &policy_body)).await?;

    println!("Created policy: {result:?}");
    assert!(result.into_inner().id.len() == 24);

    Ok(())
}

#[tokio::test]
async fn test_policies_get() -> Result<()> {
    let client = common::get_test_client()?;
    let policy = common::ensure_test_policy(&client, vec![]).await.unwrap();

    let result = client
        .policies()
        .get(&GetPolicyPolicyId::try_from(&*policy.id).unwrap())
        .await;
    assert!(result.is_ok(), "Failed to get policy: {result:?}");

    let policy_response = result.unwrap();
    println!("Retrieved policy: {policy_response:?}");
    assert_eq!(policy_response.into_inner().id, policy.id);

    Ok(())
}

#[tokio::test]
async fn test_policies_get_rule() {
    let client = common::get_test_client().unwrap();
    let policy = common::ensure_test_policy(&client, vec![
        PolicyRuleRequestBody {
            action: PolicyAction::Allow,
            conditions: vec![PolicyCondition::SolanaSystemProgramInstructionCondition(
                SolanaSystemProgramInstructionCondition {
                    field: SolanaSystemProgramInstructionConditionField::TransferLamports,
                    field_source: SolanaSystemProgramInstructionConditionFieldSource::SolanaSystemProgramInstruction,
                    operator: ConditionOperator::Lt,
                    value: ConditionValue::String("2000000".to_string()),
                }
            )],

            method: PolicyMethod::SignTransaction,
            name: PolicyRuleRequestBodyName::try_from("updated-rule").unwrap(),
        }
    ]).await.unwrap();

    let policy_id = GetRulePolicyId::try_from(&*policy.id).unwrap();
    let rule_id = GetRuleRuleId::try_from(&*policy.rules.first().unwrap().id).unwrap();

    let result = client.policies().get_rule(&policy_id, &rule_id).await;
    assert!(result.is_ok(), "Failed to get policy rule: {result:?}");

    let rule_response = result.unwrap();
    println!("Retrieved policy rule: {rule_response:?}");
}

#[tokio::test]
async fn test_policies_update_with_auth_context() {
    let client = common::get_test_client().unwrap();

    let private_key = include_str!("./test_private_key.pem");
    let key = PrivateKey::new(private_key.into());
    let pubkey = key.get_key().await.unwrap().public_key();

    let policy = common::ensure_test_policy_with_user(
        &client,
        vec![],
        Some(OwnerInput::PublicKey(pubkey.to_string())),
    )
    .await
    .unwrap();

    let ctx = AuthorizationContext::new().push(key);

    let update_body = UpdatePolicyBody {
        owner: Some(OwnerInput::PublicKey(pubkey.to_string())),
        owner_id: None,
        name: Some(UpdatePolicyBodyName::try_from("my-owned-policy").unwrap()),
        rules: vec![PolicyRuleRequestBody {
            action: PolicyAction::Allow,
            conditions: vec![PolicyCondition::SolanaSystemProgramInstructionCondition(
                SolanaSystemProgramInstructionCondition {
                    field: SolanaSystemProgramInstructionConditionField::TransferLamports,
                    field_source: SolanaSystemProgramInstructionConditionFieldSource::SolanaSystemProgramInstruction,
                    operator: ConditionOperator::Lt,
                    value: ConditionValue::String("2000000".to_string()),
                }
            )],

            method: PolicyMethod::SignTransaction,
            name: PolicyRuleRequestBodyName::try_from("updated-rule").unwrap(),
        }],
    };

    let policy_id = UpdatePolicyPolicyId::try_from(&*policy.id).unwrap();

    let result = client
        .policies()
        .update(&policy_id, &ctx, &update_body)
        .await;

    assert!(
        result.is_ok(),
        "Failed to update policy with auth context: {result:?}"
    );

    let policy_response = result.unwrap();
    println!("Updated policy: {policy_response:?}");
}

#[tokio::test]
async fn test_policies_delete() {
    let client = common::get_test_client().unwrap();

    let private_key = include_str!("./test_private_key.pem");
    let ctx = AuthorizationContext::new().push(PrivateKey::new(private_key.into()));

    // First create a policy to delete
    let unique_name = format!("delete-policy-{}", chrono::Utc::now().timestamp());
    let policy_body = CreatePolicyBody {
        chain_type: PolicyChainType::Solana,
        name: CreatePolicyBodyName::try_from(unique_name).unwrap(),
        // TODO: set the owner here once we have a JWT
        owner: None,
        owner_id: None,
        rules: vec![PolicyRuleRequestBody {
            action: PolicyAction::Allow,
            conditions: vec![PolicyCondition::SolanaSystemProgramInstructionCondition(
                SolanaSystemProgramInstructionCondition {
                    field: SolanaSystemProgramInstructionConditionField::TransferLamports,
                    field_source: SolanaSystemProgramInstructionConditionFieldSource::SolanaSystemProgramInstruction,
                    operator: ConditionOperator::Lt,
                    value: ConditionValue::String("1000000".to_string()),
                }
            )],
            method: PolicyMethod::SignTransaction,
            name: PolicyRuleRequestBodyName::try_from("test-rule").unwrap(),
        }],
        version: CreatePolicyBodyVersion::try_from("1.0").unwrap(),
    };

    let create_result = client.policies().create(None, &policy_body).await.unwrap();
    let policy_id = create_result.into_inner().id;

    // Now delete the policy
    let result = client
        .policies()
        .delete(
            &DeletePolicyPolicyId::try_from(policy_id.as_str()).unwrap(),
            &ctx,
        )
        .await;
    assert!(result.is_ok(), "Failed to delete policy: {result:?}");

    let delete_response = result.unwrap();
    println!("Deleted policy response: {delete_response:?}");
}

#[tokio::test]
async fn test_policies_create_rule() {
    let client = common::get_test_client().unwrap();
    let policy = common::ensure_test_policy(&client, vec![]).await.unwrap();

    let private_key = include_str!("./test_private_key.pem");
    let ctx = AuthorizationContext::new().push(PrivateKey::new(private_key.into()));

    let unique_rule_name = format!("test-rule-{}", chrono::Utc::now().timestamp());
    let rule = PolicyRuleRequestBody {
        action: PolicyAction::Allow,
        conditions: vec![PolicyCondition::SolanaSystemProgramInstructionCondition(
            SolanaSystemProgramInstructionCondition {
                field: SolanaSystemProgramInstructionConditionField::TransferLamports,
                field_source: SolanaSystemProgramInstructionConditionFieldSource::SolanaSystemProgramInstruction,
                operator: ConditionOperator::Lt,
                value: ConditionValue::String("5000000".to_string()),
            }
        )],
        method: PolicyMethod::SignTransaction,
        name: PolicyRuleRequestBodyName::try_from(unique_rule_name).unwrap(),
    };

    let result = client
        .policies()
        .create_rule(
            &CreateRulePolicyId::from_str(&policy.id).unwrap(),
            &ctx,
            &rule,
        )
        .await;
    assert!(result.is_ok(), "Failed to create policy rule: {result:?}");

    let rule_response = result.unwrap();
    println!("Created policy rule: {rule_response:?}");
}

#[tokio::test]
async fn test_policies_update_rule() {
    let client = common::get_test_client().unwrap();

    let mut rule = PolicyRuleRequestBody {
        action: PolicyAction::Deny,
        conditions: vec![PolicyCondition::SolanaSystemProgramInstructionCondition(
            SolanaSystemProgramInstructionCondition {
                field: SolanaSystemProgramInstructionConditionField::TransferLamports,
                field_source: SolanaSystemProgramInstructionConditionFieldSource::SolanaSystemProgramInstruction,
                operator: ConditionOperator::Gt,
                value: ConditionValue::String("10000000".to_string()),
            }
        )],
        method: PolicyMethod::SignTransaction,
        name: PolicyRuleRequestBodyName::try_from("my-great-rule").unwrap(),
    };

    let policy = common::ensure_test_policy(&client, vec![rule.clone()])
        .await
        .unwrap();

    rule.action = PolicyAction::Allow;

    let private_key = include_str!("./test_private_key.pem");
    let ctx = AuthorizationContext::new().push(PrivateKey::new(private_key.into()));

    let result = client
        .policies()
        .update_rule(
            &UpdateRulePolicyId::try_from(&*policy.id).unwrap(),
            &UpdateRuleRuleId::try_from(&*policy.rules.first().unwrap().id).unwrap(),
            &ctx,
            &rule,
        )
        .await;
    assert!(result.is_ok(), "Failed to update policy rule: {result:?}");

    let rule_response = result.unwrap();
    println!("Updated policy rule: {rule_response:?}");
}

#[tokio::test]
async fn test_policies_delete_rule() {
    let client = common::get_test_client().unwrap();
    let policy = common::ensure_test_policy(&client, vec![PolicyRuleRequestBody {
        action: PolicyAction::Allow,
        conditions: vec![PolicyCondition::SolanaSystemProgramInstructionCondition(
            SolanaSystemProgramInstructionCondition {
                field: SolanaSystemProgramInstructionConditionField::TransferLamports,
                field_source: SolanaSystemProgramInstructionConditionFieldSource::SolanaSystemProgramInstruction,
                operator: ConditionOperator::Lt,
                value: ConditionValue::String("1000000".to_string()),
            }
        )],
        method: PolicyMethod::SignTransaction,
        name: PolicyRuleRequestBodyName::try_from("my-unique-rule").unwrap(),
    }]).await.unwrap();

    let private_key = include_str!("./test_private_key.pem");
    let ctx = AuthorizationContext::new().push(PrivateKey::new(private_key.into()));

    let policy_id = DeleteRulePolicyId::try_from(&*policy.id).unwrap();
    let rule_id = DeleteRuleRuleId::try_from(&*policy.rules.first().unwrap().id).unwrap();

    let result = client
        .policies()
        .delete_rule(&policy_id, &rule_id, &ctx)
        .await;

    assert!(result.is_ok(), "Failed to delete policy rule: {result:?}");

    let delete_response = result.unwrap();
    println!("Deleted policy rule response: {delete_response:?}");
}
