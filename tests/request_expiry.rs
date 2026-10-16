//! Offline tests for the `privy-request-expiry` header on signed requests.
//!
//! Each test points a client at an `httpmock` server, captures the signed
//! request, and checks the header value and that the authorization signature
//! covers it.

use std::{
    future::Future,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use base64::{Engine, engine::general_purpose::STANDARD};
use httpmock::{Method as HttpMethod, Mock, MockServer};
use p256::{
    ecdsa::{Signature, VerifyingKey, signature::Verifier},
    pkcs8::DecodePublicKey,
};
use privy_rs::{
    AuthorizationContext, AuthorizationSignatureHeaders, Method, PrivateKey, PrivyClient,
    RequestOptions,
    client::PrivyClientOptions,
    ethereum::SendTransactionOptions,
    format_request_for_authorization_signature_with_headers,
    generated::types::{
        CreateRulePolicyId, DeletePolicyPolicyId, DeleteRulePolicyId, DeleteRuleRuleId,
        KeyQuorumId, KeyQuorumUpdateRequestBody, PolicyAction, PolicyMethod, PolicyRuleRequestBody,
        RawSignHashParams, RawSignInput, RawSignInputParams, UnsignedStandardEthereumTransaction,
        UpdatePolicyBody, UpdatePolicyPolicyId, UpdateRulePolicyId, UpdateRuleRuleId,
        WalletUpdateRequestBody,
    },
    request_expiry::{DEFAULT_REQUEST_EXPIRY, RequestExpiryOptions},
    solana::SignAndSendTransactionOptions,
};

const APP_ID: &str = "test_app_id";
const WALLET_ID: &str = "wallet_123";
// Policy and rule ids must be 24 characters.
const POLICY_ID: &str = "policy000000000000000001";
const RULE_ID: &str = "rule00000000000000000001";
const KEY_QUORUM_ID: &str = "key_quorum_123";
const PRIVATE_KEY_PEM: &str = include_str!("test_private_key.pem");
const PUBLIC_KEY_PEM: &str = include_str!("test_public_key.pem");

/// The parts of a request the tests check.
#[derive(Clone, Debug)]
struct CapturedRequest {
    url: String,
    body: serde_json::Value,
    idempotency_key: Option<String>,
    request_expiry: Option<String>,
    signature: Option<String>,
}

impl CapturedRequest {
    fn request_expiry_ms(&self) -> u64 {
        self.request_expiry
            .as_deref()
            .expect("privy-request-expiry header should be present")
            .parse()
            .expect("privy-request-expiry should be a decimal integer")
    }
}

/// Holds the most recent request a mock matched.
#[derive(Clone, Default)]
struct Capture(Arc<Mutex<Option<CapturedRequest>>>);

impl Capture {
    fn get(&self) -> CapturedRequest {
        self.0
            .lock()
            .unwrap()
            .clone()
            .expect("the mock server should have received a request")
    }
}

/// Mocks `method path`, records the request, and responds with `{}`.
fn capture<'a>(server: &'a MockServer, method: HttpMethod, path: &str) -> (Mock<'a>, Capture) {
    let captured = Capture::default();
    let sink = captured.clone();
    let base_url = server.base_url();
    let mock = server.mock(|when, then| {
        when.method(method).path(path).is_true(move |req| {
            let header = |name: &str| {
                req.headers()
                    .get(name)
                    .and_then(|v| v.to_str().ok())
                    .map(str::to_owned)
            };
            let body = if req.body_ref().is_empty() {
                serde_json::json!({})
            } else {
                serde_json::from_slice(req.body_ref()).unwrap_or(serde_json::Value::Null)
            };
            *sink.0.lock().unwrap() = Some(CapturedRequest {
                url: format!("{base_url}{}", req.uri().path()),
                body,
                idempotency_key: header("privy-idempotency-key"),
                request_expiry: header("privy-request-expiry"),
                signature: header("privy-authorization-signature"),
            });
            true
        });
        then.status(200).json_body(serde_json::json!({}));
    });
    (mock, captured)
}

fn client(server: &MockServer, request_expiry: RequestExpiryOptions) -> PrivyClient {
    PrivyClient::new_with_options(
        APP_ID.to_string(),
        "test_app_secret".to_string(),
        PrivyClientOptions {
            base_url: server.base_url(),
            request_expiry,
            ..Default::default()
        },
    )
    .expect("client should build")
}

fn ctx() -> AuthorizationContext {
    AuthorizationContext::new().push(PrivateKey::new(PRIVATE_KEY_PEM.to_string()))
}

fn now_ms() -> u64 {
    u64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    )
    .unwrap()
}

/// Asserts `expiry` is `delta` after a time between `before` and `after`.
fn assert_expiry_near(expiry: u64, before: u64, after: u64, delta: Duration) {
    let delta = u64::try_from(delta.as_millis()).unwrap();
    assert!(
        (before + delta..=after + delta).contains(&expiry),
        "expiry {expiry} should be within [{}, {}]",
        before + delta,
        after + delta
    );
}

fn canonical_request(
    method: Method,
    request: &CapturedRequest,
    request_expiry: Option<u64>,
) -> String {
    let mut headers = AuthorizationSignatureHeaders::new();
    if let Some(key) = &request.idempotency_key {
        headers = headers.with_idempotency_key(key);
    }
    if let Some(expiry) = request_expiry {
        headers = headers.with_request_expiry(expiry);
    }
    format_request_for_authorization_signature_with_headers(
        APP_ID,
        method,
        request.url.clone(),
        &request.body,
        &headers,
    )
    .unwrap()
}

/// Verifies the captured signature against the test public key, using the
/// captured headers, and checks that changing the expiry breaks it.
///
/// `method` is a constructor because `Method` is not `Clone`.
fn assert_signature_covers_headers(method: fn() -> Method, request: &CapturedRequest) {
    let der = STANDARD
        .decode(request.signature.as_deref().expect("signature header"))
        .unwrap();
    let signature = Signature::from_der(&der).unwrap();
    let key = VerifyingKey::from_public_key_pem(PUBLIC_KEY_PEM).unwrap();

    let expiry = request
        .request_expiry
        .as_deref()
        .map(|v| v.parse().unwrap());
    let signed = canonical_request(method(), request, expiry);
    key.verify(signed.as_bytes(), &signature)
        .expect("signature should cover the headers that were sent");

    let tampered = canonical_request(method(), request, Some(expiry.map_or(1, |e: u64| e + 1)));
    assert!(
        key.verify(tampered.as_bytes(), &signature).is_err(),
        "signature should not verify with a different request expiry"
    );
}

/// Sends one request through `call` with the default settings, then checks the
/// expiry header and that the signature covers it.
async fn assert_sends_default_expiry<Fut: Future>(
    http_method: HttpMethod,
    path: &str,
    method: fn() -> Method,
    call: impl FnOnce(PrivyClient) -> Fut,
) {
    let server = MockServer::start();
    let (mock, captured) = capture(&server, http_method, path);
    let client = client(&server, RequestExpiryOptions::default());

    let before = now_ms();
    call(client).await;
    let after = now_ms();

    mock.assert();
    let request = captured.get();
    assert_expiry_near(
        request.request_expiry_ms(),
        before,
        after,
        DEFAULT_REQUEST_EXPIRY,
    );
    assert_signature_covers_headers(method, &request);
}

fn rule_body() -> PolicyRuleRequestBody {
    PolicyRuleRequestBody {
        action: PolicyAction::Allow,
        conditions: vec![],
        method: PolicyMethod::SignTransaction,
        name: "rule".try_into().unwrap(),
    }
}

#[tokio::test]
async fn wallet_rpc_sends_default_expiry_covered_by_signature() {
    let server = MockServer::start();
    let (mock, captured) = capture(
        &server,
        HttpMethod::POST,
        &format!("/v1/wallets/{WALLET_ID}/rpc"),
    );
    let client = client(&server, RequestExpiryOptions::default());

    let before = now_ms();
    let _ = client
        .wallets()
        .ethereum()
        .sign_message(WALLET_ID, "hello", &ctx(), Some("idem-1"))
        .await;
    let after = now_ms();

    mock.assert();
    let request = captured.get();
    assert_eq!(request.idempotency_key.as_deref(), Some("idem-1"));
    assert_expiry_near(
        request.request_expiry_ms(),
        before,
        after,
        DEFAULT_REQUEST_EXPIRY,
    );
    assert_signature_covers_headers(|| Method::POST, &request);
}

#[tokio::test]
async fn wallet_raw_sign_sends_default_expiry_covered_by_signature() {
    let server = MockServer::start();
    let (mock, captured) = capture(
        &server,
        HttpMethod::POST,
        &format!("/v1/wallets/{WALLET_ID}/raw_sign"),
    );
    let client = client(&server, RequestExpiryOptions::default());
    let body = RawSignInput {
        params: RawSignInputParams::HashParams(RawSignHashParams {
            hash: "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"
                .parse()
                .unwrap(),
        }),
    };

    let before = now_ms();
    let _ = client
        .wallets()
        .raw_sign(WALLET_ID, &ctx(), None, &body)
        .await;
    let after = now_ms();

    mock.assert();
    let request = captured.get();
    assert_expiry_near(
        request.request_expiry_ms(),
        before,
        after,
        DEFAULT_REQUEST_EXPIRY,
    );
    assert_signature_covers_headers(|| Method::POST, &request);
}

#[tokio::test]
async fn wallet_update_sends_default_expiry_covered_by_signature() {
    let server = MockServer::start();
    let (mock, captured) = capture(
        &server,
        HttpMethod::PATCH,
        &format!("/v1/wallets/{WALLET_ID}"),
    );
    let client = client(&server, RequestExpiryOptions::default());

    let before = now_ms();
    let _ = client
        .wallets()
        .update(WALLET_ID, &ctx(), &WalletUpdateRequestBody::default())
        .await;
    let after = now_ms();

    mock.assert();
    let request = captured.get();
    assert_expiry_near(
        request.request_expiry_ms(),
        before,
        after,
        DEFAULT_REQUEST_EXPIRY,
    );
    assert_signature_covers_headers(|| Method::PATCH, &request);
}

#[tokio::test]
async fn wallet_export_sends_default_expiry_covered_by_signature() {
    let server = MockServer::start();
    let (mock, captured) = capture(
        &server,
        HttpMethod::POST,
        &format!("/v1/wallets/{WALLET_ID}/export"),
    );
    let client = client(&server, RequestExpiryOptions::default());

    let before = now_ms();
    // The mock response can't be decrypted, so the call fails after the request is sent.
    let _ = client.wallets().export(WALLET_ID, &ctx()).await;
    let after = now_ms();

    mock.assert();
    let request = captured.get();
    assert_expiry_near(
        request.request_expiry_ms(),
        before,
        after,
        DEFAULT_REQUEST_EXPIRY,
    );
    assert_signature_covers_headers(|| Method::POST, &request);
}

#[tokio::test]
async fn solana_helper_sends_default_expiry() {
    let server = MockServer::start();
    let (mock, captured) = capture(
        &server,
        HttpMethod::POST,
        &format!("/v1/wallets/{WALLET_ID}/rpc"),
    );
    let client = client(&server, RequestExpiryOptions::default());

    let before = now_ms();
    let _ = client
        .wallets()
        .solana()
        .sign_message(WALLET_ID, "aGVsbG8=", &ctx(), None)
        .await;
    let after = now_ms();

    mock.assert();
    let request = captured.get();
    assert_expiry_near(
        request.request_expiry_ms(),
        before,
        after,
        DEFAULT_REQUEST_EXPIRY,
    );
    assert_signature_covers_headers(|| Method::POST, &request);
}

#[tokio::test]
async fn custom_default_expiry_is_used() {
    let server = MockServer::start();
    let (mock, captured) = capture(
        &server,
        HttpMethod::POST,
        &format!("/v1/wallets/{WALLET_ID}/rpc"),
    );
    let custom = Duration::from_secs(60);
    let client = client(
        &server,
        RequestExpiryOptions::new().with_default_expiry(custom),
    );

    let before = now_ms();
    let _ = client
        .wallets()
        .ethereum()
        .sign_message(WALLET_ID, "hello", &ctx(), None)
        .await;
    let after = now_ms();

    mock.assert();
    let request = captured.get();
    assert_expiry_near(request.request_expiry_ms(), before, after, custom);
    assert_signature_covers_headers(|| Method::POST, &request);
}

#[tokio::test]
async fn expiry_is_computed_per_request() {
    let server = MockServer::start();
    let (mock, captured) = capture(
        &server,
        HttpMethod::PATCH,
        &format!("/v1/wallets/{WALLET_ID}"),
    );
    let client = client(&server, RequestExpiryOptions::default());
    let wallets = client.wallets();
    let body = WalletUpdateRequestBody::default();

    let _ = wallets.update(WALLET_ID, &ctx(), &body).await;
    let first = captured.get().request_expiry_ms();
    tokio::time::sleep(Duration::from_millis(10)).await;
    let _ = wallets.update(WALLET_ID, &ctx(), &body).await;
    let second = captured.get().request_expiry_ms();

    mock.assert_calls(2);
    assert!(
        second > first,
        "second expiry {second} should be later than {first}"
    );
}

#[tokio::test]
async fn disabled_expiry_sends_no_header() {
    let server = MockServer::start();
    let (mock, captured) = capture(
        &server,
        HttpMethod::POST,
        &format!("/v1/wallets/{WALLET_ID}/rpc"),
    );
    let client = client(&server, RequestExpiryOptions::new().with_disabled(true));

    let _ = client
        .wallets()
        .ethereum()
        .sign_message(WALLET_ID, "hello", &ctx(), Some("idem-1"))
        .await;

    mock.assert();
    let request = captured.get();
    assert_eq!(request.request_expiry, None);
    assert_signature_covers_headers(|| Method::POST, &request);
}

#[tokio::test]
async fn policy_update_sends_default_expiry_covered_by_signature() {
    assert_sends_default_expiry(
        HttpMethod::PATCH,
        &format!("/v1/policies/{POLICY_ID}"),
        || Method::PATCH,
        |client| async move {
            let id = UpdatePolicyPolicyId::try_from(POLICY_ID).unwrap();
            let _ = client
                .policies()
                .update(&id, &ctx(), &UpdatePolicyBody::default())
                .await;
        },
    )
    .await;
}

#[tokio::test]
async fn policy_delete_sends_default_expiry_covered_by_signature() {
    assert_sends_default_expiry(
        HttpMethod::DELETE,
        &format!("/v1/policies/{POLICY_ID}"),
        || Method::DELETE,
        |client| async move {
            let id = DeletePolicyPolicyId::try_from(POLICY_ID).unwrap();
            let _ = client.policies().delete(&id, &ctx()).await;
        },
    )
    .await;
}

#[tokio::test]
async fn policy_create_rule_sends_default_expiry_covered_by_signature() {
    assert_sends_default_expiry(
        HttpMethod::POST,
        &format!("/v1/policies/{POLICY_ID}/rules"),
        || Method::POST,
        |client| async move {
            let id = CreateRulePolicyId::try_from(POLICY_ID).unwrap();
            let _ = client
                .policies()
                .create_rule(&id, &ctx(), &rule_body())
                .await;
        },
    )
    .await;
}

#[tokio::test]
async fn policy_update_rule_sends_default_expiry_covered_by_signature() {
    assert_sends_default_expiry(
        HttpMethod::PATCH,
        &format!("/v1/policies/{POLICY_ID}/rules/{RULE_ID}"),
        || Method::PATCH,
        |client| async move {
            let policy_id = UpdateRulePolicyId::try_from(POLICY_ID).unwrap();
            let rule_id = UpdateRuleRuleId::try_from(RULE_ID).unwrap();
            let _ = client
                .policies()
                .update_rule(&policy_id, &rule_id, &ctx(), &rule_body())
                .await;
        },
    )
    .await;
}

#[tokio::test]
async fn policy_delete_rule_sends_default_expiry_covered_by_signature() {
    assert_sends_default_expiry(
        HttpMethod::DELETE,
        &format!("/v1/policies/{POLICY_ID}/rules/{RULE_ID}"),
        || Method::DELETE,
        |client| async move {
            let policy_id = DeleteRulePolicyId::try_from(POLICY_ID).unwrap();
            let rule_id = DeleteRuleRuleId::try_from(RULE_ID).unwrap();
            let _ = client
                .policies()
                .delete_rule(&policy_id, &rule_id, &ctx())
                .await;
        },
    )
    .await;
}

#[tokio::test]
async fn key_quorum_update_sends_default_expiry_covered_by_signature() {
    assert_sends_default_expiry(
        HttpMethod::PATCH,
        &format!("/v1/key_quorums/{KEY_QUORUM_ID}"),
        || Method::PATCH,
        |client| async move {
            let id = KeyQuorumId(KEY_QUORUM_ID.to_string());
            let _ = client
                .key_quorums()
                .update(&id, &ctx(), &KeyQuorumUpdateRequestBody::default())
                .await;
        },
    )
    .await;
}

#[tokio::test]
async fn key_quorum_delete_sends_default_expiry_covered_by_signature() {
    assert_sends_default_expiry(
        HttpMethod::DELETE,
        &format!("/v1/key_quorums/{KEY_QUORUM_ID}"),
        || Method::DELETE,
        |client| async move {
            let id = KeyQuorumId(KEY_QUORUM_ID.to_string());
            let _ = client.key_quorums().delete(&id, &ctx()).await;
        },
    )
    .await;
}

#[tokio::test]
async fn disabled_expiry_applies_to_policies_and_key_quorums() {
    let server = MockServer::start();
    let (policy_mock, policy_request) = capture(
        &server,
        HttpMethod::DELETE,
        &format!("/v1/policies/{POLICY_ID}"),
    );
    let (quorum_mock, quorum_request) = capture(
        &server,
        HttpMethod::DELETE,
        &format!("/v1/key_quorums/{KEY_QUORUM_ID}"),
    );
    let client = client(&server, RequestExpiryOptions::new().with_disabled(true));

    let policy_id = DeletePolicyPolicyId::try_from(POLICY_ID).unwrap();
    let _ = client.policies().delete(&policy_id, &ctx()).await;
    let quorum_id = KeyQuorumId(KEY_QUORUM_ID.to_string());
    let _ = client.key_quorums().delete(&quorum_id, &ctx()).await;

    policy_mock.assert();
    quorum_mock.assert();
    for request in [policy_request.get(), quorum_request.get()] {
        assert_eq!(request.request_expiry, None);
        assert_signature_covers_headers(|| Method::DELETE, &request);
    }
}

/// An absolute expiry far from any default, so a match means it was sent as-is.
const PER_CALL_EXPIRY: u64 = 1_900_000_000_123;

/// Sends one request through `call` with `config`, then checks that the header
/// is exactly `expected` and that the signature covers it.
async fn assert_sends_expiry<Fut: Future>(
    config: RequestExpiryOptions,
    http_method: HttpMethod,
    path: &str,
    method: fn() -> Method,
    expected: &str,
    call: impl FnOnce(PrivyClient) -> Fut,
) {
    let server = MockServer::start();
    let (mock, captured) = capture(&server, http_method, path);

    call(client(&server, config)).await;

    mock.assert();
    let request = captured.get();
    assert_eq!(request.request_expiry.as_deref(), Some(expected));
    assert_signature_covers_headers(method, &request);
}

fn per_call() -> RequestOptions {
    RequestOptions::new().with_request_expiry(PER_CALL_EXPIRY)
}

#[tokio::test]
async fn per_call_expiry_is_sent_verbatim_on_every_signed_wrapper() {
    let expected = PER_CALL_EXPIRY.to_string();
    let config = RequestExpiryOptions::default();
    let rpc_body = serde_json::from_value(serde_json::json!({
        "method": "personal_sign",
        "params": {"message": "hello", "encoding": "utf-8"}
    }))
    .unwrap();
    let raw_sign_body = RawSignInput {
        params: RawSignInputParams::HashParams(RawSignHashParams {
            hash: "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"
                .parse()
                .unwrap(),
        }),
    };

    assert_sends_expiry(
        config,
        HttpMethod::POST,
        &format!("/v1/wallets/{WALLET_ID}/rpc"),
        || Method::POST,
        &expected,
        |client| async move {
            let _ = client
                .wallets()
                .rpc_with_options(WALLET_ID, &ctx(), Some("idem-1"), &rpc_body, &per_call())
                .await;
        },
    )
    .await;
    assert_sends_expiry(
        config,
        HttpMethod::POST,
        &format!("/v1/wallets/{WALLET_ID}/raw_sign"),
        || Method::POST,
        &expected,
        |client| async move {
            let _ = client
                .wallets()
                .raw_sign_with_options(WALLET_ID, &ctx(), None, &raw_sign_body, &per_call())
                .await;
        },
    )
    .await;
    assert_sends_expiry(
        config,
        HttpMethod::PATCH,
        &format!("/v1/wallets/{WALLET_ID}"),
        || Method::PATCH,
        &expected,
        |client| async move {
            let _ = client
                .wallets()
                .update_with_options(
                    WALLET_ID,
                    &ctx(),
                    &WalletUpdateRequestBody::default(),
                    &per_call(),
                )
                .await;
        },
    )
    .await;
    assert_sends_expiry(
        config,
        HttpMethod::POST,
        &format!("/v1/wallets/{WALLET_ID}/export"),
        || Method::POST,
        &expected,
        |client| async move {
            let _ = client
                .wallets()
                .export_with_options(WALLET_ID, &ctx(), &per_call())
                .await;
        },
    )
    .await;
    assert_sends_expiry(
        config,
        HttpMethod::PATCH,
        &format!("/v1/policies/{POLICY_ID}"),
        || Method::PATCH,
        &expected,
        |client| async move {
            let id = UpdatePolicyPolicyId::try_from(POLICY_ID).unwrap();
            let _ = client
                .policies()
                .update_with_options(&id, &ctx(), &UpdatePolicyBody::default(), &per_call())
                .await;
        },
    )
    .await;
    assert_sends_expiry(
        config,
        HttpMethod::DELETE,
        &format!("/v1/policies/{POLICY_ID}"),
        || Method::DELETE,
        &expected,
        |client| async move {
            let id = DeletePolicyPolicyId::try_from(POLICY_ID).unwrap();
            let _ = client
                .policies()
                .delete_with_options(&id, &ctx(), &per_call())
                .await;
        },
    )
    .await;
    assert_sends_expiry(
        config,
        HttpMethod::POST,
        &format!("/v1/policies/{POLICY_ID}/rules"),
        || Method::POST,
        &expected,
        |client| async move {
            let id = CreateRulePolicyId::try_from(POLICY_ID).unwrap();
            let _ = client
                .policies()
                .create_rule_with_options(&id, &ctx(), &rule_body(), &per_call())
                .await;
        },
    )
    .await;
    assert_sends_expiry(
        config,
        HttpMethod::PATCH,
        &format!("/v1/policies/{POLICY_ID}/rules/{RULE_ID}"),
        || Method::PATCH,
        &expected,
        |client| async move {
            let policy_id = UpdateRulePolicyId::try_from(POLICY_ID).unwrap();
            let rule_id = UpdateRuleRuleId::try_from(RULE_ID).unwrap();
            let _ = client
                .policies()
                .update_rule_with_options(&policy_id, &rule_id, &ctx(), &rule_body(), &per_call())
                .await;
        },
    )
    .await;
    assert_sends_expiry(
        config,
        HttpMethod::DELETE,
        &format!("/v1/policies/{POLICY_ID}/rules/{RULE_ID}"),
        || Method::DELETE,
        &expected,
        |client| async move {
            let policy_id = DeleteRulePolicyId::try_from(POLICY_ID).unwrap();
            let rule_id = DeleteRuleRuleId::try_from(RULE_ID).unwrap();
            let _ = client
                .policies()
                .delete_rule_with_options(&policy_id, &rule_id, &ctx(), &per_call())
                .await;
        },
    )
    .await;
    assert_sends_expiry(
        config,
        HttpMethod::PATCH,
        &format!("/v1/key_quorums/{KEY_QUORUM_ID}"),
        || Method::PATCH,
        &expected,
        |client| async move {
            let id = KeyQuorumId(KEY_QUORUM_ID.to_string());
            let _ = client
                .key_quorums()
                .update_with_options(
                    &id,
                    &ctx(),
                    &KeyQuorumUpdateRequestBody::default(),
                    &per_call(),
                )
                .await;
        },
    )
    .await;
    assert_sends_expiry(
        config,
        HttpMethod::DELETE,
        &format!("/v1/key_quorums/{KEY_QUORUM_ID}"),
        || Method::DELETE,
        &expected,
        |client| async move {
            let id = KeyQuorumId(KEY_QUORUM_ID.to_string());
            let _ = client
                .key_quorums()
                .delete_with_options(&id, &ctx(), &per_call())
                .await;
        },
    )
    .await;
}

#[tokio::test]
async fn per_call_expiry_is_sent_when_disabled() {
    assert_sends_expiry(
        RequestExpiryOptions::new().with_disabled(true),
        HttpMethod::PATCH,
        &format!("/v1/wallets/{WALLET_ID}"),
        || Method::PATCH,
        &PER_CALL_EXPIRY.to_string(),
        |client| async move {
            let _ = client
                .wallets()
                .update_with_options(
                    WALLET_ID,
                    &ctx(),
                    &WalletUpdateRequestBody::default(),
                    &per_call(),
                )
                .await;
        },
    )
    .await;
}

#[tokio::test]
async fn per_call_zero_expiry_is_sent() {
    assert_sends_expiry(
        RequestExpiryOptions::default(),
        HttpMethod::PATCH,
        &format!("/v1/wallets/{WALLET_ID}"),
        || Method::PATCH,
        "0",
        |client| async move {
            let _ = client
                .wallets()
                .update_with_options(
                    WALLET_ID,
                    &ctx(),
                    &WalletUpdateRequestBody::default(),
                    &RequestOptions::new().with_request_expiry(0),
                )
                .await;
        },
    )
    .await;
}

#[tokio::test]
async fn ethereum_send_transaction_options_expiry_is_sent() {
    assert_sends_expiry(
        RequestExpiryOptions::new().with_disabled(true),
        HttpMethod::POST,
        &format!("/v1/wallets/{WALLET_ID}/rpc"),
        || Method::POST,
        &PER_CALL_EXPIRY.to_string(),
        |client| async move {
            let _ = client
                .wallets()
                .ethereum()
                .send_transaction_with_options(
                    WALLET_ID,
                    "eip155:1",
                    UnsignedStandardEthereumTransaction::default().into(),
                    &ctx(),
                    None,
                    &SendTransactionOptions::new().with_request_expiry(PER_CALL_EXPIRY),
                )
                .await;
        },
    )
    .await;
}

#[tokio::test]
async fn solana_sign_and_send_options_expiry_is_sent() {
    assert_sends_expiry(
        RequestExpiryOptions::default(),
        HttpMethod::POST,
        &format!("/v1/wallets/{WALLET_ID}/rpc"),
        || Method::POST,
        &PER_CALL_EXPIRY.to_string(),
        |client| async move {
            let _ = client
                .wallets()
                .solana()
                .sign_and_send_transaction_with_options(
                    WALLET_ID,
                    "solana:EtWTRABZaYq6iMfeYKouRu166VU2xqa1",
                    "AQAB",
                    &ctx(),
                    None,
                    &SignAndSendTransactionOptions::new().with_request_expiry(PER_CALL_EXPIRY),
                )
                .await;
        },
    )
    .await;
}
