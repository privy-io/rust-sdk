#[tokio::test]
async fn test_requests_include_privy_client_header() {
    use httpmock::prelude::*;
    use privy_rs::{PrivyClient, client::PrivyClientOptions};

    // Start a mock server
    let server = MockServer::start();

    // Create a mock endpoint that requires the privy-client header
    // The header should have format "rust:{version}" where version is like "0.1.0-alpha.3"
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/v1/users")
            // Verify the privy-client header exists
            .header_exists("privy-client")
            // Use a custom function to validate the header value format
            .is_true(|req: &HttpMockRequest| {
                // Get the headers and check for privy-client
                let headers = req.headers();
                if let Some(header_value) = headers.get("privy-client") {
                    // Convert the HeaderValue to a string and check the format
                    if let Ok(value_str) = header_value.to_str() {
                        // The header should start with "rust:" and contain version info
                        return value_str.starts_with("rust:") && value_str.contains('.');
                    }
                }
                false
            });
        then.status(200)
            .header("content-type", "application/json")
            .json_body(serde_json::json!({
                "data": []
            }));
    });

    // Create a PrivyClient pointing to the mock server
    let client = PrivyClient::new_with_options(
        "test_app_id".to_string(),
        "test_app_secret".to_string(),
        PrivyClientOptions {
            base_url: server.base_url(),
            ..Default::default()
        },
    )
    .expect("Failed to create client");

    // Make a request to trigger the mock using the public users() API
    // The list method takes (order_by: Option<&str>, order_direction: Option<&str>)
    let _ = client.users().list(None, None).await;

    // If we get here without the mock failing, it means the header was present
    // and had the correct format. Now verify the mock was actually called.
    mock.assert();
}

/// Drives `wallets().rpc` against a mock that answers with `status` and `body`, and
/// returns the `PrivyApiError` the SDK hands back.
async fn rpc_api_error(status: u16, body: &str) -> privy_rs::PrivyApiError {
    use httpmock::prelude::*;
    use privy_rs::{
        AuthorizationContext, PrivateKey, PrivyClient, PrivySignedApiError,
        client::PrivyClientOptions, generated::types::*,
    };

    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST).path("/v1/wallets/w1/rpc");
        then.status(status)
            .header("content-type", "application/json")
            .body(body);
    });

    let client = PrivyClient::new_with_options(
        "app_id".into(),
        "secret".into(),
        PrivyClientOptions {
            base_url: server.base_url(),
            ..Default::default()
        },
    )
    .expect("Failed to create client");
    let ctx = AuthorizationContext::new().push(PrivateKey::new(
        include_str!("test_private_key.pem").to_string(),
    ));
    let rpc_body =
        WalletRpcRequestBody::SolanaSignTransactionRpcInput(SolanaSignTransactionRpcInput {
            address: None,
            chain_type: None,
            method: SolanaSignTransactionRpcInputMethod::SignTransaction,
            params: SolanaSignTransactionRpcInputParams {
                encoding: SolanaSignTransactionRpcInputParamsEncoding::Base64,
                transaction: "AQ==".parse().unwrap(),
            },
            wallet_id: None,
        });

    let result = client.wallets().rpc("w1", &ctx, None, &rpc_body).await;
    mock.assert();
    match result {
        Err(PrivySignedApiError::Api(e)) => e,
        other => panic!("expected PrivySignedApiError::Api, got {other:?}"),
    }
}

#[tokio::test]
async fn test_api_error_body_parsed_with_code_and_data() {
    use privy_rs::{PrivyApiErrorCode, PrivyApiErrorResponse};
    use std::error::Error;

    #[derive(serde::Deserialize)]
    struct Data {
        example: String,
    }

    // synthetic: the reference does not document a data shape
    let e = rpc_api_error(
        403,
        r#"{"error":"The app secret does not have permission to make this request","code":"insufficient_app_secret_permissions","data":{"example":"value"}}"#,
    )
    .await;
    let r = PrivyApiErrorResponse::from_api_error(e).await.unwrap();

    assert_eq!(r.status, 403);
    let body = r.body.as_ref().unwrap();
    assert_eq!(
        body.code(),
        Some(PrivyApiErrorCode::InsufficientAppSecretPermissions)
    );
    assert_eq!(
        body.error.as_deref(),
        Some("The app secret does not have permission to make this request")
    );
    assert_eq!(body.data_as::<Data>().unwrap().unwrap().example, "value");
    assert!(r.source().is_none());
}

#[tokio::test]
async fn test_api_error_unknown_code_and_code_round_trip() {
    use privy_rs::{PrivyApiErrorCode, PrivyApiErrorResponse};

    let e = rpc_api_error(
        400,
        r#"{"error":"Wallet has insufficient funds for this transaction","code":"brand_new"}"#,
    )
    .await;
    let r = PrivyApiErrorResponse::from_api_error(e).await.unwrap();
    let body = r.body.as_ref().unwrap();
    assert_eq!(
        body.code(),
        Some(PrivyApiErrorCode::Unknown("brand_new".into()))
    );
    assert!(body.data_as::<String>().is_none());

    assert_eq!(
        PrivyApiErrorCode::from("policy_violation"),
        PrivyApiErrorCode::PolicyViolation
    );
    for code in [
        "policy_violation",
        "insufficient_funds",
        "transaction_broadcast_failure",
        "missing_or_empty_authorization_header",
        "zero_correct_authorization_signatures",
        "insufficient_correct_authorization_signatures",
        "incorrect_quantity_of_authorization_signatures",
        "request_expired",
        "no_valid_user_session_keys",
        "user_session_keys_expired",
        "insufficient_app_secret_permissions",
    ] {
        let parsed = PrivyApiErrorCode::from(code);
        assert!(
            !matches!(parsed, PrivyApiErrorCode::Unknown(_)),
            "{code} parsed as Unknown"
        );
        assert_eq!(parsed.as_str(), code);
    }
}

#[tokio::test]
async fn test_api_error_non_json_body_keeps_raw_and_cause() {
    use privy_rs::{PrivyApiErrorBodyError, PrivyApiErrorResponse};
    use std::error::Error;

    let e = rpc_api_error(503, "not json").await;
    let r = PrivyApiErrorResponse::from_api_error(e).await.unwrap();

    assert_eq!(r.status, 503);
    assert!(
        matches!(&r.body, Err(PrivyApiErrorBodyError::Parse { raw, .. }) if raw == "not json"),
        "unexpected body: {:?}",
        r.body
    );
    assert!(r.source().is_some());
    assert!(r.to_string().starts_with("Privy API error 503"));
}

#[tokio::test]
async fn test_api_error_non_response_variant_passes_through() {
    use privy_rs::{PrivyApiError, PrivyApiErrorResponse};

    let result =
        PrivyApiErrorResponse::from_api_error(PrivyApiError::InvalidRequest("x".into())).await;
    assert!(matches!(result, Err(PrivyApiError::InvalidRequest(_))));
}
