use base64::{Engine, engine::general_purpose::STANDARD};
use futures::TryStreamExt;
use serde::Serialize;

use crate::AuthorizationContext;

pub const PRIVY_AUTHORIZATION_HEADER: &str = "privy-authorization-signature";

/// Create canonical request data for signing
///
/// # Errors
/// This can fail if JSON serialization fails
pub fn build_canonical_request<S: Serialize>(
    app_id: &str,
    method: Method,
    url: String,
    body: S,
    idempotency_key: Option<String>,
) -> Result<String, serde_json::Error> {
    let mut headers = serde_json::Map::new();
    headers.insert(
        "privy-app-id".into(),
        serde_json::Value::String(app_id.to_owned()),
    );
    if let Some(key) = idempotency_key {
        headers.insert(
            "privy-idempotency-key".to_string(),
            serde_json::Value::String(key),
        );
    }

    WalletApiRequestSignatureInput::new(method, url)
        .headers(serde_json::Value::Object(headers))
        .body(body)
        .canonicalize()
}

pub async fn sign_canonical_request<S: Serialize>(
    ctx: &AuthorizationContext,
    app_id: &str,
    method: Method,
    url: String,
    body: S,
    idempotency_key: Option<String>,
) -> Result<String, serde_json::Error> {
    let canonical = build_canonical_request(app_id, method, url, body, idempotency_key)?;

    tracing::info!("canonical request data: {}", canonical);

    Ok(ctx
        .sign(canonical.as_bytes())
        .map_ok(|s| {
            let der_bytes = s.to_der();
            STANDARD.encode(&der_bytes)
        })
        .try_collect::<Vec<_>>()
        .await
        .map_err(|e| {
            tracing::error!("failed to sign request: {}", e);
            todo!()
        })?
        .join(","))
}

/// The HTTP method used in the request.
///
/// Note that `GET` requests do not need
/// signatures by definition.
#[derive(serde::Serialize, Debug)]
pub enum Method {
    /// `PATCH` requests are used to update an existing resource.
    PATCH,
    /// `POST` requests are used to create a new resource.
    POST,
    /// `PUT` requests are used to update an existing resource.
    PUT,
    /// `GET` requests are used to retrieve an existing resource.
    DELETE,
}

impl TryFrom<&reqwest::Method> for Method {
    type Error = ();

    fn try_from(value: &reqwest::Method) -> Result<Self, Self::Error> {
        match *value {
            reqwest::Method::PATCH => Ok(Method::PATCH),
            reqwest::Method::POST => Ok(Method::POST),
            reqwest::Method::PUT => Ok(Method::PUT),
            reqwest::Method::DELETE => Ok(Method::DELETE),
            _ => Err(()),
        }
    }
}

/// The wallet API request signature input is used
/// during the signing process as a canonical representation
/// of the request. Ensure that you serialize this struct
/// with the `serde_json_canonicalizer` to get the appropriate
/// RFC-8785 canonicalized string. For more information, see
/// <https://datatracker.ietf.org/doc/html/rfc8785>
///
/// Note: Version is currently hardcoded to 1.
#[derive(serde::Serialize)]
pub struct WalletApiRequestSignatureInput<S: Serialize> {
    version: u32,
    method: Method,
    url: String,
    body: Option<S>,
    headers: Option<serde_json::Value>,
}

impl<S: Serialize> WalletApiRequestSignatureInput<S> {
    /// Create a new request builder.
    #[must_use]
    pub fn new(method: Method, url: String) -> Self {
        Self {
            version: 1,
            method,
            url,
            body: None,
            headers: None,
        }
    }

    /// Set the request body.
    #[must_use]
    pub fn body(mut self, body: S) -> Self {
        self.body = Some(body);
        self
    }

    /// Set the request headers.
    #[must_use]
    pub fn headers(mut self, headers: serde_json::Value) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Canonicalize the request body.
    ///
    /// # Errors
    /// Returns an error if the serialization fails.
    pub fn canonicalize(self) -> Result<String, serde_json::Error> {
        serde_json_canonicalizer::to_string(&self)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use test_case::test_case;
    use tracing_test::traced_test;

    use super::*;
    use crate::{
        AuthorizationContext, IntoKey, PrivateKey, PrivateKeyFromFile, PrivyClient,
        generated::types::{OwnerInput, PublicKeyOwner, UpdateWalletBody},
        get_auth_header,
    };

    #[tokio::test]
    async fn test_build_canonical_request() {
        let key = PrivateKeyFromFile("private_key.pem".into());
        let public_key = key.get_key().await.unwrap().public_key();

        // Create the request body that will be sent using the generated privy-api type
        let update_wallet_body = UpdateWalletBody {
            owner: Some(OwnerInput {
                subtype_0: Some(PublicKeyOwner {
                    public_key: public_key.to_string(),
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        // Build the canonical request data for signing using the serialized body
        let canonical_data = build_canonical_request(
            "cmf418pa801bxl40b5rcgjvd9".into(),
            Method::PATCH,
            "https://api.privy.io/v1/wallets/o5zuf7fbygwze9l9gaxyc0bm".into(),
            update_wallet_body.clone(),
            None,
        )
        .unwrap();

        assert_eq!(
            canonical_data,
            "{\"body\":{\"owner\":{\"public_key\":\"-----BEGIN PUBLIC KEY-----\\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAESYrvEwooR33jt/8Up0lWdDNAcxmg\\nNZrCX23OThCPA+WxDx+dHYrjRlfPmHX0/aMTopp1PdKAtlQjRJDHSNd8XA==\\n-----END PUBLIC KEY-----\\n\"}},\"headers\":{\"privy-app-id\":\"cmf418pa801bxl40b5rcgjvd9\"},\"method\":\"PATCH\",\"url\":\"https://api.privy.io/v1/wallets/o5zuf7fbygwze9l9gaxyc0bm\",\"version\":1}"
        );
    }

    // Method enum tests
    #[test]
    fn test_method_serialization() {
        assert_eq!(serde_json::to_string(&Method::PATCH).unwrap(), "\"PATCH\"");
        assert_eq!(serde_json::to_string(&Method::POST).unwrap(), "\"POST\"");
        assert_eq!(serde_json::to_string(&Method::PUT).unwrap(), "\"PUT\"");
        assert_eq!(
            serde_json::to_string(&Method::DELETE).unwrap(),
            "\"DELETE\""
        );
    }

    #[test]
    fn test_method_debug() {
        assert_eq!(format!("{:?}", Method::PATCH), "PATCH");
        assert_eq!(format!("{:?}", Method::POST), "POST");
        assert_eq!(format!("{:?}", Method::PUT), "PUT");
        assert_eq!(format!("{:?}", Method::DELETE), "DELETE");
    }

    #[test]
    fn test_method_try_from_reqwest_method() {
        assert!(matches!(
            Method::try_from(&reqwest::Method::PATCH),
            Ok(Method::PATCH)
        ));
        assert!(matches!(
            Method::try_from(&reqwest::Method::POST),
            Ok(Method::POST)
        ));
        assert!(matches!(
            Method::try_from(&reqwest::Method::PUT),
            Ok(Method::PUT)
        ));
        assert!(matches!(
            Method::try_from(&reqwest::Method::DELETE),
            Ok(Method::DELETE)
        ));

        // GET should fail
        assert!(Method::try_from(&reqwest::Method::GET).is_err());
        assert!(Method::try_from(&reqwest::Method::HEAD).is_err());
        assert!(Method::try_from(&reqwest::Method::OPTIONS).is_err());
    }

    // WalletApiRequestSignatureInput tests
    #[test]
    fn test_wallet_api_request_signature_input_new() {
        let input = WalletApiRequestSignatureInput::new(
            Method::POST,
            "https://api.privy.io/v1/test".to_string(),
        )
        .body(json!({}));

        // Can't directly test private fields, but we can test the behavior
        let canonical = input.canonicalize().unwrap();
        assert!(canonical.contains("\"version\":1"));
        assert!(canonical.contains("\"method\":\"POST\""));
        assert!(canonical.contains("https://api.privy.io/v1/test"));
    }

    #[test]
    fn test_wallet_api_request_signature_input_with_body() {
        let body = json!({"test": "value"});
        let input = WalletApiRequestSignatureInput::new(
            Method::POST,
            "https://api.privy.io/v1/test".to_string(),
        )
        .body(body);

        let canonical = input.canonicalize().unwrap();
        assert!(canonical.contains("\"body\":{\"test\":\"value\"}"));
    }

    #[test]
    fn test_wallet_api_request_signature_input_with_headers() {
        let headers = json!({"header1": "value1", "header2": "value2"});
        let input = WalletApiRequestSignatureInput::new(
            Method::POST,
            "https://api.privy.io/v1/test".to_string(),
        )
        .body(json!({}))
        .headers(headers);

        let canonical = input.canonicalize().unwrap();
        assert!(canonical.contains("\"headers\":{\"header1\":\"value1\",\"header2\":\"value2\"}"));
    }

    #[test]
    fn test_wallet_api_request_signature_input_complete() {
        let body = json!({"data": "test"});
        let headers = json!({"auth": "token"});
        let input = WalletApiRequestSignatureInput::new(
            Method::PATCH,
            "https://api.privy.io/v1/wallets/123".to_string(),
        )
        .body(body)
        .headers(headers);

        let canonical = input.canonicalize().unwrap();
        assert!(canonical.contains("\"body\":{\"data\":\"test\"}"));
        assert!(canonical.contains("\"headers\":{\"auth\":\"token\"}"));
        assert!(canonical.contains("\"method\":\"PATCH\""));
        assert!(canonical.contains("\"version\":1"));
    }

    #[test]
    fn test_wallet_api_request_signature_input_no_body() {
        let input = WalletApiRequestSignatureInput::new(
            Method::DELETE,
            "https://api.privy.io/v1/test".to_string(),
        )
        .body(json!(null));

        let canonical = input.canonicalize().unwrap();
        assert!(canonical.contains("\"body\":null"));
    }

    #[test]
    fn test_wallet_api_request_signature_input_no_headers() {
        let input = WalletApiRequestSignatureInput::new(
            Method::POST,
            "https://api.privy.io/v1/test".to_string(),
        )
        .body(json!({}));

        let canonical = input.canonicalize().unwrap();
        assert!(canonical.contains("\"headers\":null"));
    }

    #[test]
    fn test_build_canonical_request_different_methods() {
        for method in [Method::POST, Method::PUT, Method::PATCH, Method::DELETE] {
            let result = build_canonical_request(
                "test_app_id",
                method,
                "https://api.privy.io/v1/test".to_string(),
                json!({}),
                None,
            );

            assert!(result.is_ok());
            let canonical = result.unwrap();
            assert!(canonical.contains("\"version\":1"));
        }
    }

    // Constants tests
    #[test]
    fn test_privy_authorization_header_constant() {
        assert_eq!(PRIVY_AUTHORIZATION_HEADER, "privy-authorization-signature");
    }

    // Canonicalization tests (expanded from existing)
    #[test]
    fn test_canonicalization_matches_docs_example() {
        let builder = WalletApiRequestSignatureInput::new(
            Method::PATCH,
            "https://api.privy.io/v1/wallets/clw4cc3a700b811p865d21b7b".to_string(),
        )
        .body(json!({
            "policy_ids": ["pol_123abc"]
        }))
        .headers(json!({
            "privy-app-id": "your-privy-app-id",
            "privy-idempotency-key": "a-unique-uuid-for-the-request"
        }));

        let canonical = builder
            .canonicalize()
            .expect("canonicalization should succeed");
        let expected = r#"{"body":{"policy_ids":["pol_123abc"]},"headers":{"privy-app-id":"your-privy-app-id","privy-idempotency-key":"a-unique-uuid-for-the-request"},"method":"PATCH","url":"https://api.privy.io/v1/wallets/clw4cc3a700b811p865d21b7b","version":1}"#;

        assert_eq!(canonical, expected);
    }

    #[test]
    fn test_key_ordering() {
        let builder =
            WalletApiRequestSignatureInput::new(Method::POST, "https://example.com".to_string())
                .body(json!({
                    "z_last": "last",
                    "a_first": "first",
                    "m_middle": "middle"
                }))
                .headers(json!({
                    "z-header": "last",
                    "a-header": "first"
                }));

        let canonical = builder
            .canonicalize()
            .expect("canonicalization should succeed");

        // Keys should be sorted alphabetically at all levels
        assert!(canonical.contains(r#"{"a_first":"first","m_middle":"middle","z_last":"last"}"#));
        assert!(canonical.contains(r#"{"a-header":"first","z-header":"last"}"#));
    }

    #[test]
    fn test_nested_object_sorting() {
        let builder =
            WalletApiRequestSignatureInput::new(Method::POST, "https://example.com".to_string())
                .body(json!({
                    "outer": {
                        "z_inner": "last",
                        "a_inner": "first"
                    }
                }));

        let canonical = builder
            .canonicalize()
            .expect("canonicalization should succeed");

        // Nested object keys should also be sorted
        assert!(canonical.contains(r#"{"a_inner":"first","z_inner":"last"}"#));
    }

    #[test]
    fn test_array_preservation() {
        let builder =
            WalletApiRequestSignatureInput::new(Method::POST, "https://example.com".to_string())
                .body(json!({
                    "items": ["third", "first", "second"]
                }));

        let canonical = builder
            .canonicalize()
            .expect("canonicalization should succeed");

        // Array order should be preserved (not sorted)
        assert!(canonical.contains(r#"["third","first","second"]"#));
    }

    #[test]
    fn test_canonicalization_deterministic() {
        let body = json!({
            "b": "second",
            "a": "first",
            "c": {
                "z": "last",
                "y": "middle"
            }
        });

        let input1 =
            WalletApiRequestSignatureInput::new(Method::POST, "https://example.com".to_string())
                .body(body.clone());

        let input2 =
            WalletApiRequestSignatureInput::new(Method::POST, "https://example.com".to_string())
                .body(body);

        let canonical1 = input1.canonicalize().unwrap();
        let canonical2 = input2.canonicalize().unwrap();

        assert_eq!(
            canonical1, canonical2,
            "Canonicalization should be deterministic"
        );
    }

    #[test]
    fn test_canonicalization_empty_objects() {
        let builder =
            WalletApiRequestSignatureInput::new(Method::POST, "https://example.com".to_string())
                .body(json!({}))
                .headers(json!({}));

        let canonical = builder.canonicalize().unwrap();
        assert!(canonical.contains("\"body\":{}"));
        assert!(canonical.contains("\"headers\":{}"));
    }

    #[test]
    fn test_canonicalization_special_values() {
        let builder =
            WalletApiRequestSignatureInput::new(Method::POST, "https://example.com".to_string())
                .body(json!({
                    "null_value": null,
                    "boolean_true": true,
                    "boolean_false": false,
                    "number_int": 42,
                    "number_float": 3.14159,
                    "string_empty": "",
                    "string_with_quotes": "He said \"Hello\"",
                    "string_with_newlines": "line1\nline2\r\nline3",
                    "array_mixed": [null, true, 1, "string"]
                }));

        let canonical = builder.canonicalize().unwrap();

        // Verify special values are handled correctly
        assert!(canonical.contains("\"null_value\":null"));
        assert!(canonical.contains("\"boolean_true\":true"));
        assert!(canonical.contains("\"boolean_false\":false"));
        assert!(canonical.contains("\"number_int\":42"));
        assert!(canonical.contains("\"string_empty\":\"\""));
        assert!(canonical.contains("\\\"Hello\\\""));
        assert!(canonical.contains("\"array_mixed\":[null,true,1,\"string\"]"));
    }

    #[test]
    fn test_canonicalization_unicode() {
        let builder =
            WalletApiRequestSignatureInput::new(Method::POST, "https://example.com".to_string())
                .body(json!({
                    "unicode": "Hello 世界 🌍",
                    "emoji": "🔐🚀💎",
                    "accents": "café naïve résumé"
                }));

        let canonical = builder.canonicalize().unwrap();

        // Unicode should be preserved
        assert!(canonical.contains("Hello 世界 🌍"));
        assert!(canonical.contains("🔐🚀💎"));
        assert!(canonical.contains("café naïve résumé"));
    }

    #[test_case(
        &json!({"name": "John", "age": 30}),
        r#"{"age":30,"name":"John"}"#;
        "simple object"
    )]
    #[test_case(
        &json!({"name": "John", "address": {"street": "123 Main St", "city": "Boston"}}),
        r#"{"address":{"city":"Boston","street":"123 Main St"},"name":"John"}"#;
        "nested object"
    )]
    #[test_case(
        &json!({"name": "John", "numbers": [1, 2, 3]}),
        r#"{"name":"John","numbers":[1,2,3]}"#;
        "array"
    )]
    #[test_case(
        &json!({"name": "John", "age": null}),
        r#"{"age":null,"name":"John"}"#;
        "null value"
    )]
    #[test_case(
        &json!({"name": "John", "age": 30, "address": {"street": "123 Main St", "city": "Boston"}, "hobbies": ["reading", "gaming"], "middleName": null}),
        r#"{"address":{"city":"Boston","street":"123 Main St"},"age":30,"hobbies":["reading","gaming"],"middleName":null,"name":"John"}"#;
        "complex object"
    )]
    fn test_json_canonicalization(json: &serde_json::Value, expected: &str) {
        let result =
            serde_json_canonicalizer::to_string(json).expect("canonicalization should succeed");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_url_with_query_parameters() {
        let builder = WalletApiRequestSignatureInput::new(
            Method::POST,
            "https://api.privy.io/v1/wallets?limit=10&offset=0".to_string(),
        )
        .body(json!(null));

        let canonical = builder.canonicalize().unwrap();
        assert!(canonical.contains("https://api.privy.io/v1/wallets?limit=10&offset=0"));
    }

    #[test]
    fn test_url_with_fragment() {
        let builder = WalletApiRequestSignatureInput::new(
            Method::POST,
            "https://api.privy.io/v1/wallets#section".to_string(),
        )
        .body(json!(null));

        let canonical = builder.canonicalize().unwrap();
        assert!(canonical.contains("https://api.privy.io/v1/wallets#section"));
    }

    #[test]
    fn test_deeply_nested_structure() {
        let builder =
            WalletApiRequestSignatureInput::new(Method::POST, "https://example.com".to_string())
                .body(json!({
                    "level1": {
                        "level2": {
                            "level3": {
                                "level4": {
                                    "data": "deep"
                                }
                            }
                        }
                    }
                }));

        let canonical = builder.canonicalize().unwrap();
        assert!(canonical.contains("\"data\":\"deep\""));
    }

    const TEST_PRIVATE_KEY_PEM: &str = include_str!("../tests/test_private_key.pem");

    fn get_staging_env() -> (String, String, String) {
        let app_id = std::env::var("STAGING_APP_ID").unwrap_or_else(|_| "test_app_id".to_string());
        let app_secret =
            std::env::var("STAGING_APP_SECRET").unwrap_or_else(|_| "test_app_secret".to_string());
        let url =
            std::env::var("STAGING_URL").unwrap_or_else(|_| "https://api.privy.io".to_string());
        (app_id, app_secret, url)
    }

    #[test]
    fn test_privy_client_new() {
        let ctx = AuthorizationContext::new();
        let result = PrivyClient::new("test_app".to_string(), "test_secret".to_string(), ctx);
        assert!(result.is_ok(), "Should successfully create client");

        let client = result.unwrap();
        assert_eq!(client.app_id, "test_app");
    }

    #[test]
    fn test_privy_client_new_with_url() {
        let ctx = AuthorizationContext::new();
        let result = PrivyClient::new_with_url(
            "test_app".to_string(),
            "test_secret".to_string(),
            ctx,
            "https://staging.privy.io",
        );
        assert!(
            result.is_ok(),
            "Should successfully create client with custom URL"
        );

        let client = result.unwrap();
        assert_eq!(client.app_id, "test_app");
    }

    #[test]
    fn test_privy_client_invalid_credentials() {
        let ctx = AuthorizationContext::new();
        // Test with invalid characters that would fail header validation
        let result = PrivyClient::new("test\napp".to_string(), "test_secret".to_string(), ctx);
        assert!(
            result.is_err(),
            "Should fail with invalid app_id characters"
        );
    }

    #[test]
    fn test_privy_client_clone() {
        let ctx = AuthorizationContext::new();
        let client =
            PrivyClient::new("test_app".to_string(), "test_secret".to_string(), ctx).unwrap();
        let client2 = client.clone();

        assert_eq!(client.app_id, client2.app_id);
        assert_eq!(client.app_secret, client2.app_secret);
    }

    #[test]
    fn test_privy_client_debug() {
        let ctx = AuthorizationContext::new();
        let client =
            PrivyClient::new("test_app".to_string(), "test_secret".to_string(), ctx).unwrap();
        let debug_str = format!("{:?}", client);
        assert!(
            debug_str.contains("PrivyClient"),
            "Debug output should contain struct name"
        );
        assert!(
            debug_str.contains("test_app"),
            "Debug output should contain app_id"
        );
    }

    #[test]
    fn test_build_canonical_request_with_idempotency_key() {
        let body = serde_json::json!({"test": "data"});
        let idempotency_key = "unique-key-123".to_string();

        let canonical_data = build_canonical_request(
            "test_app_id",
            Method::POST,
            "https://api.privy.io/v1/test".to_string(),
            body,
            Some(idempotency_key.clone()),
        )
        .unwrap();

        assert!(
            canonical_data.contains(&idempotency_key),
            "Should include idempotency key"
        );
        assert!(
            canonical_data.contains("privy-idempotency-key"),
            "Should include idempotency key header"
        );
    }

    #[test_case(Method::POST ; "POST method")]
    #[test_case(Method::PUT ; "PUT method")]
    #[test_case(Method::PATCH ; "PATCH method")]
    #[test_case(Method::DELETE ; "DELETE method")]
    fn test_build_canonical_request_methods(method: Method) {
        let body = serde_json::json!({"test": "data"});

        let canonical_data = build_canonical_request(
            "test_app_id",
            method,
            "https://api.privy.io/v1/test".to_string(),
            body,
            None,
        )
        .unwrap();

        assert!(
            canonical_data.contains("\"method\":"),
            "Should include method"
        );
        assert!(
            canonical_data.contains("test_app_id"),
            "Should include app_id"
        );
        assert!(
            canonical_data.contains("\"version\":1"),
            "Should include version"
        );
    }

    #[test]
    fn test_build_canonical_request_empty_body() {
        let canonical_data = build_canonical_request(
            "test_app_id",
            Method::POST,
            "https://api.privy.io/v1/test".to_string(),
            serde_json::Value::Null,
            None,
        )
        .unwrap();

        assert!(
            canonical_data.contains("\"body\":null"),
            "Should handle null body"
        );
    }

    #[test]
    fn test_build_canonical_request_complex_body() {
        let body = serde_json::json!({
            "nested": {
                "field": "value",
                "array": [1, 2, 3],
                "boolean": true
            },
            "string": "test"
        });

        let canonical_data = build_canonical_request(
            "test_app_id",
            Method::POST,
            "https://api.privy.io/v1/test".to_string(),
            body,
            None,
        )
        .unwrap();

        // Should be properly canonicalized (keys sorted)
        assert!(
            canonical_data
                .contains("\"nested\":{\"array\":[1,2,3],\"boolean\":true,\"field\":\"value\"}")
        );
        assert!(canonical_data.contains("\"string\":\"test\""));
    }

    #[tokio::test]
    #[traced_test]
    async fn test_sign_canonical_request() {
        let ctx = AuthorizationContext::new();
        ctx.push(PrivateKey(TEST_PRIVATE_KEY_PEM.to_string()));

        let body = serde_json::json!({"test": "data"});

        let result = sign_canonical_request(
            &ctx,
            "test_app_id",
            Method::POST,
            "https://api.privy.io/v1/test".to_string(),
            body,
            None,
        )
        .await;

        assert!(result.is_ok(), "Should successfully sign canonical request");

        let signature = result.unwrap();
        assert!(!signature.is_empty(), "Signature should not be empty");
        assert!(
            signature.contains(',') == false || signature.split(',').count() == 1,
            "Should have one signature for one key"
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn test_sign_canonical_request_multiple_keys() {
        let ctx = AuthorizationContext::new();
        ctx.push(PrivateKey(TEST_PRIVATE_KEY_PEM.to_string()));

        // Add another key
        use p256::elliptic_curve::SecretKey;
        let key_bytes = [2u8; 32];
        let second_key = SecretKey::<p256::NistP256>::from_bytes(&key_bytes.into()).unwrap();
        ctx.push(second_key);

        let body = serde_json::json!({"test": "data"});

        let result = sign_canonical_request(
            &ctx,
            "test_app_id",
            Method::POST,
            "https://api.privy.io/v1/test".to_string(),
            body,
            None,
        )
        .await;

        assert!(
            result.is_ok(),
            "Should successfully sign with multiple keys"
        );

        let signature = result.unwrap();
        assert!(
            signature.contains(','),
            "Should have comma-separated signatures for multiple keys"
        );
        assert_eq!(
            signature.split(',').count(),
            2,
            "Should have exactly two signatures"
        );
    }

    #[tokio::test]
    async fn test_sign_canonical_request_deterministic() {
        let ctx = AuthorizationContext::new();
        ctx.push(PrivateKey(TEST_PRIVATE_KEY_PEM.to_string()));

        let body = serde_json::json!({"test": "data"});

        let signature1 = sign_canonical_request(
            &ctx,
            "test_app_id",
            Method::POST,
            "https://api.privy.io/v1/test".to_string(),
            body.clone(),
            None,
        )
        .await
        .unwrap();

        let signature2 = sign_canonical_request(
            &ctx,
            "test_app_id",
            Method::POST,
            "https://api.privy.io/v1/test".to_string(),
            body,
            None,
        )
        .await
        .unwrap();

        assert_eq!(signature1, signature2, "Signatures should be deterministic");
    }

    #[test]
    fn test_build_canonical_request_json_serialization_error() {
        // This should not fail in practice with serde_json, but test the error path
        use std::f64;
        let body = serde_json::json!({"invalid": f64::NAN});

        let result = build_canonical_request(
            "test_app_id",
            Method::POST,
            "https://api.privy.io/v1/test".to_string(),
            body,
            None,
        );

        // NaN should serialize to null in serde_json, so this should actually succeed
        assert!(result.is_ok(), "serde_json handles NaN gracefully");
    }

    // Integration tests with staging environment
    #[tokio::test]
    #[ignore] // Only run when STAGING_* env vars are set
    async fn test_client_creation_staging() {
        let (app_id, app_secret, url) = get_staging_env();
        let ctx = AuthorizationContext::new();

        let result = PrivyClient::new_with_url(app_id, app_secret, ctx, &url);
        assert!(
            result.is_ok(),
            "Should successfully create client with staging credentials"
        );
    }

    // Test auth header generation
    #[test]
    fn test_auth_header_generation() {
        let app_id = "test_app_id";
        let app_secret = "test_app_secret";

        let auth_header = get_auth_header(app_id, app_secret);

        assert!(
            auth_header.starts_with("Basic "),
            "Should start with Basic "
        );

        // Decode and verify
        let encoded = auth_header.strip_prefix("Basic ").unwrap();
        let decoded = STANDARD.decode(encoded).unwrap();
        let credentials = String::from_utf8(decoded).unwrap();

        assert_eq!(credentials, "test_app_id:test_app_secret");
    }

    #[test]
    fn test_canonical_request_url_encoding() {
        let body = serde_json::json!({"test": "data"});
        let url_with_query = "https://api.privy.io/v1/test?param=value&other=123";

        let canonical_data = build_canonical_request(
            "test_app_id",
            Method::POST,
            url_with_query.to_string(),
            body,
            None,
        )
        .unwrap();

        assert!(
            canonical_data.contains(url_with_query),
            "Should preserve URL as-is including query parameters"
        );
    }

    #[test]
    fn test_canonical_request_special_characters() {
        let body = serde_json::json!({
            "special": "test with spaces and símböls",
            "unicode": "🔐🌟",
            "escaped": "quotes \"inside\" string"
        });

        let canonical_data = build_canonical_request(
            "test_app_id",
            Method::POST,
            "https://api.privy.io/v1/test".to_string(),
            body,
            None,
        )
        .unwrap();

        // Should properly escape JSON
        assert!(
            canonical_data.contains("\\\"inside\\\""),
            "Should escape internal quotes"
        );
        assert!(
            canonical_data.contains("🔐🌟"),
            "Should preserve Unicode characters"
        );
    }
}
