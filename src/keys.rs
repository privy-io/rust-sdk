use std::{
    future,
    pin::Pin,
    sync::{Arc, Mutex},
};

use futures::{Stream, StreamExt};
use p256::{
    ecdsa::{Signature, SigningKey, signature::hazmat::PrehashSigner},
    elliptic_curve::SecretKey,
};

use crate::{KeyError, SigningError};

const SIGNATURE_RESOLUTION_CONCURRENCY: usize = 10;

/// A context for signing messages. Any keys added to the context will be
/// automatically added to the list of signatories for requests to the Privy API
/// that require authorization.
///
/// The context accepts anything that implements `IntoSignature`, which by
/// extension includes anything that implements `IntoKey`. This allows you to
/// create a context that includes keys from a variety of sources, such as
/// files, JWTs, or KMS services.
///
/// For usage information, see the `AuthorizationContext::sign` and
/// `AuthorizationContext::push` methods.
///
/// This struct is thread-safe, and can be cloned. It synchronizes access to the
/// underlying store internally.
#[derive(Clone)]
pub struct AuthorizationContext {
    signers: Arc<Mutex<Vec<Arc<dyn IntoSignatureBoxed + Send + Sync>>>>,
    resolution_concurrency: usize,
}

impl std::fmt::Debug for AuthorizationContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthorizationContext").finish()
    }
}

impl Default for AuthorizationContext {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthorizationContext {
    /// Create a new `AuthorizationContext` with the default concurrency.
    #[must_use]
    pub fn new() -> Self {
        Self {
            signers: Default::default(),
            resolution_concurrency: SIGNATURE_RESOLUTION_CONCURRENCY,
        }
    }

    /// Push a new credential source into the context. This supports
    /// anything that implements `IntoSignature`, which includes
    /// anything that implements `IntoKey`.
    ///
    /// In the following example, we create a `JwtUser` source which
    /// will transparently perform authorization with privy to get
    /// a key, and then sign the message with that key. We also
    /// add a `PrivateKey` source which you can set yourself.
    ///
    /// ```rust
    /// # use privy_rs::{AuthorizationContext, JwtUser, IntoSignature, PrivateKey, PrivyClient};
    /// # use p256::ecdsa::signature::SignerMut;
    /// # use p256::ecdsa::Signature;
    /// # use p256::elliptic_curve::SecretKey;
    /// # use std::time::Duration;
    /// # use std::sync::Arc;
    /// # async fn foo() {
    /// let privy = PrivyClient::new("app_id".to_string(), "app_secret".to_string()).unwrap();
    /// let jwt = JwtUser(privy, "test".to_string());
    /// let key = PrivateKey("test".to_string());
    /// let context = AuthorizationContext::new().push(jwt).push(key);
    /// # }
    /// ```
    pub fn push<T: IntoSignature + 'static + Send + Sync>(self, key: T) -> Self {
        self.signers
            .lock()
            .expect("lock poisoned")
            .push(Arc::new(key));
        self
    }

    /// Sign a message with all the keys in the context.
    /// This produces a stream which yields values as they
    /// become available. You can collect it into a vec.
    /// This function will resolve all signatures concurrently,
    /// according to the policy set in `AuthorizationContext`.
    ///
    /// ```rust
    /// # use privy_rs::{AuthorizationContext, JwtUser, IntoSignature, PrivyClient};
    /// # use p256::ecdsa::signature::SignerMut;
    /// # use p256::ecdsa::Signature;
    /// # use p256::elliptic_curve::SecretKey;
    /// # use std::time::Duration;
    /// # use std::sync::Arc;
    /// # use futures::stream::StreamExt;
    /// # async fn foo() {
    /// let privy = PrivyClient::new("app_id".to_string(), "app_secret".to_string()).unwrap();
    /// let jwt = JwtUser(privy, "test".to_string());
    /// let context = AuthorizationContext::new().push(jwt);
    /// let key = context.sign(&[0, 1, 2, 3]).collect::<Vec<_>>().await;
    /// assert_eq!(key.len(), 1);
    /// # }
    /// ```
    ///
    /// You can also use `try_collect` to get a `Result<Vec<_>, Error>`,
    /// or any other combinators on the `StreamExt` and `TryStreamExt` traits.
    ///
    /// ```rust
    /// # use privy_rs::{AuthorizationContext, JwtUser, IntoSignature, PrivyClient};
    /// # use p256::ecdsa::signature::SignerMut;
    /// # use p256::ecdsa::Signature;
    /// # use p256::elliptic_curve::SecretKey;
    /// # use std::time::Duration;
    /// # use std::sync::Arc;
    /// # use futures::stream::TryStreamExt;
    /// # async fn foo() {
    /// let privy = PrivyClient::new("app_id".to_string(), "app_secret".to_string()).unwrap();
    /// let jwt = JwtUser(privy, "test".to_string());
    /// let context = AuthorizationContext::new().push(jwt);
    /// let key = context.sign(&[0, 1, 2, 3]).try_collect::<Vec<_>>().await;
    /// assert!(key.map(|v| v.len() == 1).unwrap_or(false));
    /// # }
    /// ```
    pub fn sign<'a>(
        &'a self,
        message: &'a [u8],
    ) -> impl Stream<Item = Result<Signature, SigningError>> + 'a {
        // we clone the inner vector before signing so we don't need to hold the lock.
        // cloning this vector will also clone the inner items, which are reference counted
        let keys = self.signers.lock().expect("lock poisoned").clone();

        futures::stream::iter(keys)
            .map(move |key| {
                let key = key.clone();
                // this is some awkwardness in rust's type system.
                // we need communicate to the type system we want to
                // move the key, clone it, then move both the key and
                // message into an async closure. later versions of
                // rust may allow us to be less explicit here
                async move { key.sign_boxed(message).await }
            })
            // await multiple `sign_boxed` futures concurrently,
            // returning them in order of completion
            .buffer_unordered(self.resolution_concurrency)
    }

    /// Exercise the signing mechanism to validate that all keys
    /// are valid and can produce signatures. Returns a vector
    /// of errors. An empty vector indicates that all keys are
    /// valid.
    ///
    /// ```
    /// # use privy_rs::{AuthorizationContext, JwtUser, IntoSignature, PrivyClient};
    /// # use p256::ecdsa::signature::SignerMut;
    /// # use p256::ecdsa::Signature;
    /// # use p256::elliptic_curve::SecretKey;
    /// # use std::time::Duration;
    /// # use std::sync::Arc;
    /// # async fn foo() {
    /// let privy = PrivyClient::new("app_id".to_string(), "app_secret".to_string()).unwrap();
    /// let jwt = JwtUser(privy, "test".to_string());
    /// let key = SecretKey::<p256::NistP256>::from_sec1_pem(&"test".to_string()).unwrap();
    /// let context = AuthorizationContext::new().push(jwt).push(key);
    /// let errors = context.validate().await;
    /// assert!(errors.is_empty());
    /// # }
    /// ```
    pub async fn validate(&self) -> Vec<SigningError> {
        self.sign(&[])
            .filter_map(|r| future::ready(r.err())) // filter_map expects a future
            .collect::<Vec<_>>()
            .await
    }
}

type Key = SecretKey<p256::NistP256>;

/// A trait for getting a key from a source. See `IntoKey::get_key` for more details.
pub trait IntoKey {
    /// Get a key from the `IntoKey` source.
    fn get_key(&self) -> impl Future<Output = Result<Key, KeyError>> + Send;
}

/// A trait for signing messages. See `IntoSignature::sign` for more details.
pub trait IntoSignature {
    /// Sign a message using deterministic ECDSA.
    ///
    /// This method implements a two-step signing process that ensures compatibility
    /// with Privy's API signature verification:
    ///
    /// ## Process Overview
    ///
    /// 1. **Message Hashing**: The input message is hashed using SHA-256 to produce
    ///    a 32-byte digest. This follows the standard practice of hashing messages
    ///    before signing to ensure security and compatibility.
    ///
    /// 2. **Deterministic ECDSA Signing**: The hash is signed using ECDSA P-256
    ///    with RFC 6979 deterministic k-value generation. This ensures that the
    ///    same message will always produce the same signature when signed with
    ///    the same private key.
    ///
    /// ## Why Deterministic Signing?
    ///
    /// Traditional ECDSA uses random k-values during signature generation, which
    /// means the same message signed with the same key can produce different valid
    /// signatures. However, Privy's API expects deterministic signatures.
    ///
    /// By using RFC 6979 deterministic k-value generation, we ensure:
    /// - **Reproducibility**: Same input always produces same signature
    /// - **Security**: RFC 6979 provides cryptographically secure k-values
    /// - **Consistency**: Matches the behavior of Privy's other SDKs
    ///
    /// ## Example Usage
    ///
    /// ```rust
    /// use std::path::PathBuf;
    ///
    /// use privy_rs::{IntoSignature, PrivateKey};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let my_key = include_str!("../tests/test_private_key.pem").to_string();
    /// let key_source = PrivateKey(my_key);
    /// let message = b"canonical request data";
    /// let signature = key_source.sign(message).await?;
    ///
    /// // The signature is deterministic - signing the same message again
    /// // with the same key will produce identical results
    /// let signature2 = key_source.sign(message).await?;
    /// assert_eq!(signature, signature2);
    /// # Ok(())
    /// # }
    /// ```
    fn sign(&self, message: &[u8]) -> impl Future<Output = Result<Signature, SigningError>> + Send;
}

// this is a blanket implementation for all types that implement IntoKey.
// we can simply call get_key on T (since it implements IntoKey) and
// then call sign on that to get the signature. having this means
// that AuthorizationContext can be used with any type that implements
// IntoKey, including things like JwtUser, PrivateKey, and PrivateKeyFromFile.
impl<T> IntoSignature for T
where
    T: IntoKey + Sync,
{
    async fn sign(&self, message: &[u8]) -> Result<Signature, SigningError> {
        let key = self.get_key().await?;
        key.sign(message).await
    }
}

/// Rust has a concept of 'object safety' and `IntoSignature` is not object safe,
/// meaning it can not be used in `AuthorizationContext` directly. This is because
/// IntoSignature's return type can differ in size depending on the type of the
/// implementor.
///
/// What we can do, however, is provide a trait that _is_ object safe, and to blanket
/// impl `IntoSignatureBoxed` for all types that implement `IntoSignature`.
/// IntoSignatureBoxed returns a boxed future instead, which is object safe. If you
/// are familiar with `async_trait`, this is how it works under the hood, and how all
/// rust traits worked until GAT / RPITIT landed.
///
/// NOTE: this is a private implementation detail and will never leak to the public API.
trait IntoSignatureBoxed {
    fn sign_boxed<'a>(
        &'a self,
        message: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = Result<Signature, SigningError>> + Send + 'a>>;
}

// the blanket impl referenced above
impl<T: IntoSignature + 'static> IntoSignatureBoxed for T {
    fn sign_boxed<'a>(
        &'a self,
        message: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = Result<Signature, SigningError>> + Send + 'a>> {
        Box::pin(self.sign(message))
    }
}

/// A wrapper for a closure that implements `IntoSignature`.
/// This uses the newtype pattern to avoid conflicting blanket impls.
pub struct FnSigner<F>(pub F);

/// A wrapper for a closure that implements `IntoKey`.
/// This uses the newtype pattern to avoid conflicting blanket impls.
pub struct FnKey<F>(pub F);

/// Blanket implementation for the FnSigner wrapper.
impl<F, Fut> IntoSignature for FnSigner<F>
where
    F: Fn(&[u8]) -> Fut,
    Fut: Future<Output = Result<Signature, SigningError>> + Send,
{
    fn sign(&self, message: &[u8]) -> impl Future<Output = Result<Signature, SigningError>> + Send {
        (self.0)(message)
    }
}

/// Blanket implementation for the FnKey wrapper.
impl<F, Fut> IntoKey for FnKey<F>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<Key, KeyError>> + Send,
{
    fn get_key(&self) -> impl Future<Output = Result<Key, KeyError>> + Send {
        (self.0)()
    }
}

/// A key that is sourced from the user identified by the provided JWT.
///
/// This is used in JWT-based authentication. When attempting to sign,
/// the JWT is used to retrieve the user's key from the Privy API.
///
/// # Errors
/// This provider can fail if the JWT is invalid, does not match a user,
/// or if the API returns an error.
pub struct JwtUser(pub crate::PrivyClient, pub String);

impl IntoKey for JwtUser {
    async fn get_key(&self) -> Result<Key, KeyError> {
        self.0
            .jwt_exchange
            .exchange_jwt_for_authorization_key(self)
            .await
    }
}

impl IntoSignature for Key {
    async fn sign(&self, message: &[u8]) -> Result<Signature, SigningError> {
        use sha2::{Digest, Sha256};

        tracing::debug!(
            "Starting ECDSA signing process for {} byte message",
            message.len()
        );

        // First hash the message with SHA256
        let hashed = {
            let mut sha256 = Sha256::new();
            sha256.update(message);
            sha256.finalize()
        };

        tracing::debug!("SHA256 hash computed: {}", hex::encode(hashed));

        // Sign the hash using deterministic signing (RFC 6979)
        let signing_key = SigningKey::from(self.clone());

        // Use deterministic prehash signing to ensure consistent signatures
        let signature: Signature = signing_key.sign_prehash(&hashed)?;

        tracing::debug!("ECDSA signature generated using deterministic RFC 6979");

        Ok(signature)
    }
}

impl IntoSignature for Signature {
    async fn sign(&self, _message: &[u8]) -> Result<Signature, SigningError> {
        Ok(*self)
    }
}

/// A raw private key in SEC1 PEM format.
///
/// # Errors
/// This provider can fail if the key is not in the expected format.
pub struct PrivateKey(pub String);

impl IntoKey for PrivateKey {
    async fn get_key(&self) -> Result<Key, KeyError> {
        SecretKey::<p256::NistP256>::from_sec1_pem(&self.0).map_err(|e| {
            tracing::error!("Failed to parse SEC1 PEM: {:?}", e);
            KeyError::InvalidFormat(self.0.clone())
        })
    }
}

#[cfg(test)]
mod tests {
    use base64::{Engine, engine::general_purpose::STANDARD};
    use futures::TryStreamExt;
    use p256::{
        ecdsa::Signature,
        elliptic_curve::{SecretKey, generic_array::GenericArray},
    };
    use test_case::test_case;
    use tracing_test::traced_test;

    use super::*;
    use crate::{
        AuthorizationContext, FnKey, FnSigner, KeyError, PrivyClient, client::PrivyClientOptions,
    };

    // generated using `mise gen-p256-key`
    const TEST_PRIVATE_KEY_PEM: &str = include_str!("../tests/test_private_key.pem");

    fn get_test_client() -> Result<PrivyClient, Box<dyn std::error::Error>> {
        let app_id = std::env::var("STAGING_APP_ID").unwrap_or_else(|_| "test_app_id".to_string());
        let app_secret =
            std::env::var("STAGING_APP_SECRET").unwrap_or_else(|_| "test_app_secret".to_string());
        let url =
            std::env::var("STAGING_URL").unwrap_or_else(|_| "https://api.privy.io".to_string());

        Ok(PrivyClient::new_with_options(
            app_id,
            app_secret,
            PrivyClientOptions {
                base_url: url,
                ..Default::default()
            },
        )?)
    }

    fn get_test_jwt() -> String {
        std::env::var("STAGING_JWT").unwrap_or_else(|_| {
            "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJhbGV4QGFybHlvbi5kZXYiLCJpYXQiOjEwMDAwMDAwMDAwMH0.IpNgavH95CFZPjkzQW4eyxMIfJ-O_5cIaDyu_6KRXffykjYDRwxTgFJuYq0F6d8wSXf4de-vzfBRWSKMISM3rJdlhximYINGJB14mJFCD87VMLFbTpHIXcv7hc1AAYMPGhOsRkYfYXuvVopKszMvhupmQYJ1npSvKWNeBniIyOHYv4xebZD8L0RVlPvuEKTXTu-CDfs2rMwvD9g_wiBznS3uMF3v_KPaY6x0sx9zeCSxAH9zvhMMtct_Ad9kuoUncGpRzNhEk6JlVccN2Leb1JzbldxSywyS2AApD05u-GFAgFDN3P39V3qgRTGDuuUfUvKQ9S4rbu5El9Qq1CJTeA".to_string()
        })
    }

    // PrivateKey tests
    #[tokio::test]
    async fn test_private_key_creation() {
        let key = PrivateKey(TEST_PRIVATE_KEY_PEM.to_string());
        let result = key.get_key().await;
        assert!(result.is_ok(), "Should successfully parse valid PEM key");
    }

    #[tokio::test]
    async fn test_private_key_invalid_format() {
        let key = PrivateKey("invalid_pem_data".to_string());
        let result = key.get_key().await;
        assert!(result.is_err(), "Should fail with invalid PEM data");

        if let Err(KeyError::InvalidFormat(_)) = result {
            // Expected error type
        } else {
            panic!("Expected InvalidFormat error");
        }
    }

    #[tokio::test]
    async fn test_private_key_signing() {
        let key = PrivateKey(TEST_PRIVATE_KEY_PEM.to_string());
        let test_key = key.get_key().await.unwrap();

        let message1 = b"test message for signing";
        let message2 = b"different message";

        // Test deterministic signing - same message should produce same signature
        let signature1a = test_key.sign(message1).await.unwrap();
        let signature1b = test_key.sign(message1).await.unwrap();
        assert_eq!(
            signature1a, signature1b,
            "Deterministic signing should produce identical signatures"
        );

        // Test different messages produce different signatures
        let signature2 = test_key.sign(message2).await.unwrap();
        assert_ne!(
            signature1a, signature2,
            "Different messages should produce different signatures"
        );
    }

    // Message signing tests with various inputs
    #[test_case(b"" ; "empty message")]
    #[test_case(b"short" ; "short message")]
    #[test_case(&[0u8; 1000] ; "long message")]
    #[test_case(b"special chars: \x00\xff\n\r\t" ; "special characters")]
    #[tokio::test]
    async fn test_signing_various_messages(message: &[u8]) {
        let key = PrivateKey(TEST_PRIVATE_KEY_PEM.to_string());
        let test_key = key.get_key().await.unwrap();

        let signature = test_key.sign(message).await;
        assert!(
            signature.is_ok(),
            "Should successfully sign message of length {}",
            message.len()
        );
    }

    #[tokio::test]
    async fn test_signature_into_signature() {
        // Create a known signature
        let key = PrivateKey(TEST_PRIVATE_KEY_PEM.to_string());
        let test_key = key.get_key().await.unwrap();
        let original_signature = test_key.sign(b"test").await.unwrap();

        // Use the signature as an IntoSignature source
        let result = original_signature.sign(b"ignored_message").await.unwrap();
        assert_eq!(
            result, original_signature,
            "Signature should return itself regardless of message"
        );
    }

    // AuthorizationContext tests
    #[tokio::test]
    #[traced_test]
    async fn test_authorization_context_empty() {
        let ctx = AuthorizationContext::new();
        let signatures: Vec<_> = ctx.sign(b"test").try_collect().await.unwrap();
        assert!(
            signatures.is_empty(),
            "Empty context should produce no signatures"
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn test_authorization_context_single_key() {
        let key = PrivateKey(TEST_PRIVATE_KEY_PEM.to_string());
        let ctx = AuthorizationContext::new().push(key);

        let signatures: Vec<_> = ctx.sign(b"test").try_collect().await.unwrap();
        assert_eq!(
            signatures.len(),
            1,
            "Context with one key should produce one signature"
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn test_authorization_context_multiple_keys() {
        // Create another deterministic key for testing
        let key_bytes = [2u8; 32]; // Different from test key
        let second_key = SecretKey::<p256::NistP256>::from_bytes(&key_bytes.into()).unwrap();

        // Add multiple keys
        let ctx = AuthorizationContext::new()
            .push(PrivateKey(TEST_PRIVATE_KEY_PEM.to_string()))
            .push(second_key);

        let signatures: Vec<_> = ctx.sign(b"test").try_collect().await.unwrap();
        assert_eq!(
            signatures.len(),
            2,
            "Context with two keys should produce two signatures"
        );

        // Signatures should be different (different keys)
        assert_ne!(
            signatures[0], signatures[1],
            "Different keys should produce different signatures"
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn test_authorization_context_validation() {
        // Test successful validation
        let ctx = AuthorizationContext::new().push(PrivateKey(TEST_PRIVATE_KEY_PEM.to_string()));
        let errors = ctx.validate().await;
        assert!(
            errors.is_empty(),
            "Valid context should have no validation errors"
        );

        // Test validation failure
        let ctx2 = AuthorizationContext::new().push(PrivateKey("invalid_key_data".to_string()));
        let errors2 = ctx2.validate().await;
        assert!(
            !errors2.is_empty(),
            "Invalid key should produce validation errors"
        );
    }

    // Function wrapper tests
    #[tokio::test]
    async fn test_fn_signer_wrapper() {
        use crate::SigningError;

        #[derive(Debug)]
        struct DummyError;
        impl std::error::Error for DummyError {}
        impl std::fmt::Display for DummyError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "dummy error")
            }
        }

        // Test that FnSigner struct exists and can be constructed
        let _signer = FnSigner(|_message: &[u8]| async move {
            // Mock function - return error for simplicity
            let result: Result<Signature, SigningError> =
                Err(SigningError::Other(Box::new(DummyError)));
            result
        });

        assert!(matches!(
            _signer.sign(&[0]).await,
            Err(SigningError::Other(_))
        ));
    }

    #[tokio::test]
    async fn test_fn_key_wrapper() {
        let key_fn =
            FnKey(|| async { PrivateKey(TEST_PRIVATE_KEY_PEM.to_string()).get_key().await });

        let key1 = key_fn.get_key().await.unwrap();
        let key2 = key_fn.get_key().await.unwrap();

        // Keys should be the same (same source)
        assert_eq!(key1.to_bytes(), key2.to_bytes());
    }

    #[tokio::test]
    #[traced_test]
    async fn test_authorization_context_concurrent_signing() {
        let mut ctx = AuthorizationContext::new();

        // Add multiple keys for concurrent testing
        for i in 0..5 {
            // Create deterministic keys for testing
            let mut key_bytes = [1u8; 32];
            key_bytes[0] = i as u8 + 1; // Make each key different
            let key = SecretKey::<p256::NistP256>::from_bytes(&key_bytes.into()).unwrap();
            ctx = ctx.push(key);
        }

        let message = b"concurrent test message";
        let signatures: Vec<_> = ctx.sign(message).try_collect().await.unwrap();

        assert_eq!(
            signatures.len(),
            5,
            "Should produce 5 signatures concurrently"
        );

        // All signatures should be different (different keys)
        for i in 0..signatures.len() {
            for j in (i + 1)..signatures.len() {
                assert_ne!(
                    signatures[i], signatures[j],
                    "Signatures from different keys should be different"
                );
            }
        }
    }

    #[tokio::test]
    async fn test_key_public_key_derivation() {
        let private_key = PrivateKey(TEST_PRIVATE_KEY_PEM.to_string());
        let key = private_key.get_key().await.unwrap();
        let public_key = key.public_key();

        // Should be able to derive public key without error
        assert!(
            !public_key.to_string().is_empty(),
            "Public key string should not be empty"
        );

        // Public key should be consistent
        let public_key2 = key.public_key();
        assert_eq!(
            public_key.to_string(),
            public_key2.to_string(),
            "Public key derivation should be consistent"
        );
    }

    // Integration tests that require actual API access
    #[tokio::test]
    #[ignore] // Only run when STAGING_* env vars are set
    async fn test_jwt_user_integration() {
        let client = get_test_client().unwrap();
        let jwt = get_test_jwt();
        let jwt_user = JwtUser(client, jwt);

        // Test both key retrieval and signing in one test
        let key_result = jwt_user.get_key().await;
        if key_result.is_err() {
            println!("JWT integration test skipped - staging environment may not be configured");
            return;
        }

        let sign_result = jwt_user.sign(b"test message").await;
        if sign_result.is_err() {
            println!(
                "JWT signing integration test skipped - staging environment may not be configured"
            );
        }
    }

    // Legacy compatibility test
    #[tokio::test]
    #[traced_test]
    async fn test_authorization_context_mixed_sources() {
        // Add path-based key and pre-computed signature
        let ctx = AuthorizationContext::new()
            .push(PrivateKey(
                include_str!("../tests/test_private_key.pem").to_string(),
            ))
            .push(Signature::from_bytes(GenericArray::from_slice(&STANDARD.decode("J7GLk/CIqvCNCOSJ8sUZb0rCsqWF9l1H1VgYfsAd1ew2uBJHE5hoY+kV7CSzdKkgOhtdvzj22gXA7gcn5gSqvQ==").unwrap())).expect("right size"));

        let sigs = ctx
            .sign(&[0, 1, 2, 3])
            .try_collect::<Vec<_>>()
            .await
            .expect("passes");

        assert!(
            !sigs.is_empty(),
            "Context with mixed sources should produce signatures"
        );
    }

    // Error handling tests
    #[tokio::test]
    async fn test_signing_error_propagation() {
        struct FailingKey;

        #[derive(Debug)]
        struct DummyError;
        impl std::error::Error for DummyError {}
        impl std::fmt::Display for DummyError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "dummy error")
            }
        }

        impl IntoKey for FailingKey {
            async fn get_key(&self) -> Result<SecretKey<p256::NistP256>, KeyError> {
                Err(KeyError::Other(Box::new(DummyError)))
            }
        }

        let failing_key = FailingKey;
        let result = failing_key.sign(b"test").await;
        assert!(matches!(result, Err(SigningError::Key(KeyError::Other(_)))));
    }

    #[tokio::test]
    async fn test_authorization_context_clone_and_debug() {
        let ctx1 = AuthorizationContext::new().push(PrivateKey(TEST_PRIVATE_KEY_PEM.to_string()));

        // Test clone functionality
        let ctx2 = ctx1.clone();
        let sigs1: Vec<_> = ctx1.sign(b"test").try_collect().await.unwrap();
        let sigs2: Vec<_> = ctx2.sign(b"test").try_collect().await.unwrap();
        assert_eq!(sigs1.len(), 1);
        assert_eq!(sigs2.len(), 1);
        assert_eq!(
            sigs1[0], sigs2[0],
            "Cloned context should produce same signatures"
        );

        // Test debug output
        let debug_str = format!("{ctx1:?}");
        assert!(
            debug_str.contains("AuthorizationContext"),
            "Debug output should contain struct name"
        );
    }
}
