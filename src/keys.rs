use std::{
    future,
    path::{Path, PathBuf},
    pin::Pin,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use futures::{Stream, StreamExt};
use p256::{
    ecdsa::{Signature, SigningKey, signature::hazmat::PrehashSigner},
    elliptic_curve::SecretKey,
};
use privy_api::types::builder::AuthenticateBody;
use tokio::sync::RwLock;

use crate::{KeyError, SigningError, privy_hpke::PrivyHpke};

const EXPIRY_BUFFER: Duration = Duration::from_secs(60);
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
pub struct AuthorizationContext(
    // this is a mutex so that users can keep multiple copies and push new
    // keys at any time, potentially in different threads. we do not use
    // an async mutex, since we do not hold the lock across await boundaries
    //
    // we wrap the IntoSignatureBoxed in an Arc so that we can clone the
    // inner vector before signing so we don't need to hold the lock
    Arc<Mutex<Vec<Arc<dyn IntoSignatureBoxed + Send + Sync>>>>,
    usize,
);

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
        Self(Default::default(), SIGNATURE_RESOLUTION_CONCURRENCY)
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
    /// # use privy_rust::{AuthorizationContext, JwtUser, IntoSignature, PrivateKey, PrivyClient};
    /// # use p256::ecdsa::signature::SignerMut;
    /// # use p256::ecdsa::Signature;
    /// # use p256::elliptic_curve::SecretKey;
    /// # use std::time::Duration;
    /// # use std::sync::Arc;
    /// # async fn foo() {
    /// let privy = PrivyClient::new("app_id".to_string(), "app_secret".to_string()).unwrap();
    /// let jwt = JwtUser(Arc::new(privy), "test".to_string());
    /// let key = PrivateKey("test".to_string());
    /// let mut context = AuthorizationContext::new();
    /// context.push(jwt);
    /// context.push(key);
    /// # }
    /// ```
    pub fn push<T: IntoSignature + 'static + Send + Sync>(&self, key: T) {
        self.0.lock().expect("lock poisoned").push(Arc::new(key));
    }

    /// Sign a message with all the keys in the context.
    /// This produces a stream which yields values as they
    /// become available. You can collect it into a vec.
    /// This function will resolve all signatures concurrently,
    /// according to the policy set in `AuthorizationContext`.
    ///
    /// ```rust
    /// # use privy_rust::{AuthorizationContext, JwtUser, IntoSignature, PrivyClient};
    /// # use p256::ecdsa::signature::SignerMut;
    /// # use p256::ecdsa::Signature;
    /// # use p256::elliptic_curve::SecretKey;
    /// # use std::time::Duration;
    /// # use std::sync::Arc;
    /// # use futures::stream::StreamExt;
    /// # async fn foo() {
    /// let privy = PrivyClient::new("app_id".to_string(), "app_secret".to_string()).unwrap();
    /// let jwt = JwtUser(Arc::new(privy), "test".to_string());
    /// let mut context = AuthorizationContext::new();
    /// context.push(jwt);
    /// let key = context.sign(&[0, 1, 2, 3]).collect::<Vec<_>>().await;
    /// assert_eq!(key.len(), 1);
    /// # }
    /// ```
    ///
    /// You can also use `try_collect` to get a `Result<Vec<_>, Error>`,
    /// or any other combinators on the `StreamExt` and `TryStreamExt` traits.
    ///
    /// ```rust
    /// # use privy_rust::{AuthorizationContext, JwtUser, IntoSignature, PrivyClient};
    /// # use p256::ecdsa::signature::SignerMut;
    /// # use p256::ecdsa::Signature;
    /// # use p256::elliptic_curve::SecretKey;
    /// # use std::time::Duration;
    /// # use std::sync::Arc;
    /// # use futures::stream::TryStreamExt;
    /// # async fn foo() {
    /// let privy = PrivyClient::new("app_id".to_string(), "app_secret".to_string()).unwrap();
    /// let jwt = JwtUser(Arc::new(privy), "test".to_string());
    /// let mut context = AuthorizationContext::new();
    /// context.push(jwt);
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
        let keys = self.0.lock().expect("lock poisoned").clone();

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
            .buffer_unordered(self.1)
    }

    /// Exercise the signing mechanism to validate that all keys
    /// are valid and can produce signatures. Returns a vector
    /// of errors. An empty vector indicates that all keys are
    /// valid.
    ///
    /// ```
    /// # use privy_rust::{AuthorizationContext, JwtUser, IntoSignature, PrivyClient};
    /// # use p256::ecdsa::signature::SignerMut;
    /// # use p256::ecdsa::Signature;
    /// # use p256::elliptic_curve::SecretKey;
    /// # use std::time::Duration;
    /// # use std::sync::Arc;
    /// # async fn foo() {
    /// let privy = PrivyClient::new("app_id".to_string(), "app_secret".to_string()).unwrap();
    /// let jwt = JwtUser(Arc::new(privy), "test".to_string());
    /// let key = SecretKey::<p256::NistP256>::from_sec1_pem(&"test".to_string()).unwrap();
    /// let mut context = AuthorizationContext::new();
    /// context.push(jwt);
    /// context.push(key);
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

/// A newtype wrapper around a `p256::SecretKey`.
///
/// This wrapper facilitates the use of a generic `AsRef<Key>` pattern across the crate,
/// allowing different key representations (like `ExpirableKey`) to be used interchangeably
/// where a basic signing key is needed.
#[derive(Clone, Debug)]
pub struct Key(pub SecretKey<p256::NistP256>);

impl AsRef<Key> for Key {
    fn as_ref(&self) -> &Key {
        &self
    }
}

/// A trait for abstracting the retrieval of a signing key from various sources.
///
/// This trait is central to the key handling mechanism, providing a generic interface
/// for objects that can produce a signing key. It is designed to be flexible,
/// supporting different types of keys and retrieval strategies.
///
/// ## Associated Type `Output`
///
/// The `Output` associated type allows implementors to return different kinds of
/// key-containing structures. For example, one implementation might return a simple
/// `Key`, while another might return an `ExpirableKey` that includes expiry information.
///
/// The `AsRef<Key>` bound on `Output` is crucial. It ensures that no matter what
/// specific type is returned by `get_key`, it can always be treated as a reference
/// to a `Key` for signing operations. This allows for a consistent signing interface
/// while accommodating diverse key types and capabilities.
///
/// ## Example
///
/// ```rust,ignore
/// // A simple key provider
/// struct SimpleKeyProvider;
/// impl IntoKey for SimpleKeyProvider {
///     type Output = Key;
///     async fn get_key(&self) -> Result<Self::Output, KeyError> {
///         // ... logic to get a key
///     }
/// }
///
/// // A provider for keys that expire
/// struct ExpirableKeyProvider;
/// impl IntoKey for ExpirableKeyProvider {
///     type Output = ExpirableKey;
///     async fn get_key(&self) -> Result<Self::Output, KeyError> {
///         // ... logic to get a key with an expiry time
///     }
/// }
/// ```
pub trait IntoKey {
    /// The output type of the `get_key` method. Must be convertible to `&Key`.
    /// This allows for flexible return types (e.g. `Key`, `ExpirableKey`) while
    /// maintaining a consistent interface for consumers that need a signing key.
    type Output: AsRef<Key> + Sync + Send;

    /// Asynchronously retrieves a key from the source.
    ///
    /// This method is responsible for fetching, decoding, or otherwise preparing
    /// the signing key for use.
    fn get_key(&self) -> impl Future<Output = Result<Self::Output, KeyError>> + Send;
}

// the identity impl, all keys can of course be used as keys
impl IntoKey for SecretKey<p256::NistP256> {
    type Output = Key;

    async fn get_key(&self) -> Result<Self::Output, KeyError> {
        Ok(Key(self.clone()))
    }
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
    /// use privy_rust::{IntoSignature, PrivateKeyFromFile};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let key_source = PrivateKeyFromFile(PathBuf::from("private_key.pem"));
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
        key.as_ref().sign(message).await
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

/// A wrapper that adds caching to any `IntoKey` implementation.
///
/// This struct acts as a decorator, intercepting calls to `get_key` and returning a
/// cached key if one is available and not expired. This is particularly useful for
/// key sources that involve network requests or expensive computations, such as `JwtUser`.
///
/// ## Caching Logic
///
/// The caching mechanism relies on the `Output` of the wrapped `IntoKey` implementation.
/// If the `Output` type implements `AsRef<SystemTime>`, `TimeCachingKey` will use the
/// provided time as the key's expiration date.
///
/// When `get_key` is called:
/// 1. It checks for a cached key.
/// 2. If a key exists, it checks if its expiration time (obtained via `AsRef<SystemTime>`)
///    is in the future.
/// 3. If the key is valid, it's returned immediately.
/// 4. Otherwise, it calls the underlying `get_key` method, caches the new key with its
///    expiration time, and then returns it.
///
/// The `ExpirableKey` struct is a common `Output` type used with `TimeCachingKey`.
///
/// # Usage
/// ```
/// # use privy_rust::{IntoKey, JwtUser, TimeCachingKey, PrivyClient};
/// # use std::time::Duration;
/// # use std::sync::Arc;
/// # use p256::ecdsa::signature::SignerMut;
/// # use p256::ecdsa::Signature;
/// # use p256::elliptic_curve::SecretKey;
/// # async fn foo() {
/// let privy = PrivyClient::new("app_id".to_string(), "app_secret".to_string()).unwrap();
/// let jwt_key_source = JwtUser(Arc::new(privy), "test".to_string());
/// // Wrap the source with the caching decorator.
/// let cached_key_source = TimeCachingKey::new(jwt_key_source);
///
/// // The first call will fetch the key from the API.
/// let key1 = cached_key_source.get_key().await.unwrap();
/// // Subsequent calls (within the expiry window) will return the cached key.
/// let key2 = cached_key_source.get_key().await.unwrap();
/// # }
/// ```
pub struct TimeCachingKey<T: IntoKey>(T, Arc<RwLock<Option<(SystemTime, T::Output)>>>);

impl<T: IntoKey + Sync> TimeCachingKey<T> {
    /// Create a new `TimeCachingKey` from anything that implements `IntoKey`.
    pub fn new(key: T) -> Self {
        Self(key, Arc::new(RwLock::new(None)))
    }
}

impl<T: IntoKey + Sync> IntoKey for TimeCachingKey<T>
where
    T::Output: Clone + Send + Sync + AsRef<SystemTime>,
{
    type Output = T::Output;

    async fn get_key(&self) -> Result<Self::Output, KeyError> {
        {
            let valid = self.1.read().await;
            let now = SystemTime::now();
            if let Some((time, key)) = valid.as_ref().filter(|(time, _)| time > &now) {
                tracing::debug!("returning cached key expiring at {:?} vs {:?}", time, now);
                return Ok(key.clone());
            } else {
                tracing::debug!("key missing or expired at {:?}", now);
            }
        }

        let key = self.0.get_key().await?;
        let expiry: &SystemTime = key.as_ref();

        {
            let mut state = self.1.write().await;
            *state = Some((expiry.to_owned(), key.clone()));
        }

        Ok(key)
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
    type Output = Key;

    fn get_key(&self) -> impl Future<Output = Result<Key, KeyError>> + Send {
        (self.0)()
    }
}

/// A key that includes an expiration time.
///
/// This struct bundles a `Key` with its `SystemTime` of expiration. It is designed
/// to be used as the `Output` for `IntoKey` implementations where keys have a
/// limited lifetime (e.g., keys derived from JWTs).
///
/// It implements `AsRef<Key>` to allow it to be used for signing, and `AsRef<SystemTime>`
/// to expose its expiration time to caching mechanisms like `TimeCachingKey`.
#[derive(Clone, Debug)]
pub struct ExpirableKey(Key, SystemTime);

impl AsRef<Key> for ExpirableKey {
    fn as_ref(&self) -> &Key {
        &self.0
    }
}

impl AsRef<SystemTime> for ExpirableKey {
    fn as_ref(&self) -> &SystemTime {
        &self.1
    }
}

/// A key that is sourced from the user identified by the provided JWT.
/// If you wish to automatically cache the key for its expiry period,
/// you can use
///
/// This is used in JWT-based authentication. When attempting to sign,
/// the JWT is used to retrieve the user's key from the Privy API.
///
/// # Example
///
/// It is recommended to use this with `TimeCachingKey` to automatically
/// cache the key for its expiry period. If you would prefer to refresh
/// every time, then you can use `JwtUser` directly.
///
/// ```rust
/// # use privy_rust::{AuthorizationContext, JwtUser, IntoSignature, PrivyClient, TimeCachingKey};
/// # use p256::ecdsa::signature::SignerMut;
/// # use p256::ecdsa::Signature;
/// # use p256::elliptic_curve::SecretKey;
/// # use std::time::Duration;
/// # use std::sync::Arc;
/// # async fn foo() {
/// let privy = PrivyClient::new("app_id".to_string(), "app_secret".to_string()).unwrap();
/// let jwt = JwtUser(Arc::new(privy), "test".to_string());
/// let key = TimeCachingKey::new(jwt);
/// let mut context = AuthorizationContext::new();
/// context.push(key);
/// # }
/// ```
///
/// # Errors
/// This provider can fail if the JWT is invalid, does not match a user,
/// or if the API returns an error.
pub struct JwtUser(pub Arc<crate::PrivyClient>, pub String);

impl IntoKey for JwtUser {
    type Output = ExpirableKey;

    async fn get_key(&self) -> Result<Self::Output, KeyError> {
        tracing::debug!("Starting HPKE JWT exchange for user JWT: {}", self.1);

        // Get the HPKE manager and format the public key for the API request
        let hpke_manager = PrivyHpke::new();
        let public_key_b64 = hpke_manager.public_key().map_err(|e| {
            tracing::error!("Failed to format HPKE public key: {:?}", e);
            KeyError::Unknown
        })?;

        tracing::debug!(
            "Generated HPKE public key for authentication request {}",
            public_key_b64
        );

        // Build the authentication request with encryption parameters
        let body = AuthenticateBody::default()
            .user_jwt(self.1.clone())
            .encryption_type(privy_api::types::AuthenticateBodyEncryptionType::Hpke)
            .recipient_public_key(public_key_b64);

        // Send the authentication request
        let auth = match self.0.authenticate().body(body).send().await {
            Ok(r) => r.into_inner(),
            Err(privy_api::Error::UnexpectedResponse(response)) => {
                tracing::error!("Unexpected API response: {:?}", response.text().await);
                return Err(KeyError::Unknown);
            }
            Err(e) => {
                tracing::error!("API request failed: {:?}", e);
                return Err(KeyError::Unknown);
            }
        };

        // Process the response based on encryption type
        let (key, expiry) = match auth {
            privy_api::types::AuthenticateResponse::WithEncryption {
                encrypted_authorization_key,
                expires_at,
                ..
            } => {
                tracing::debug!("Received encrypted authorization key, starting HPKE decryption");

                // convert the f64 expiry to a SystemTime
                let expiry = SystemTime::UNIX_EPOCH + Duration::from_secs_f64(expires_at);

                let key = hpke_manager
                    .decrypt(
                        &encrypted_authorization_key.encapsulated_key,
                        &encrypted_authorization_key.ciphertext,
                    )
                    .map_err(|e| {
                        tracing::error!("HPKE decryption failed: {:?}", e);
                        KeyError::HpkeDecryption(format!("{e:?}"))
                    })
                    .map(Key)?;

                (key, expiry)
            }
            privy_api::types::AuthenticateResponse::WithoutEncryption { .. } => {
                tracing::warn!("Received unencrypted authorization key (fallback mode)");

                // Fallback to the old method for backwards compatibility
                todo!()
            }
        };

        tracing::debug!("successfully obtained and parsed authorization key");

        Ok(ExpirableKey(
            key,
            // subtract a reasonable buffer to ensure the key is valid by the time it is used
            // otherwise we may end up giving a soon-to-expire key to the signer which is ends up invalid
            expiry - EXPIRY_BUFFER,
        ))
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
        let signing_key = SigningKey::from(self.0.clone());

        // Use deterministic prehash signing to ensure consistent signatures
        let signature: Signature = signing_key
            .sign_prehash(&hashed)
            .map_err(|_| SigningError::Unknown)?;

        tracing::debug!("ECDSA signature generated using deterministic RFC 6979");

        Ok(signature)
    }
}

impl IntoKey for &Path {
    type Output = Key;

    async fn get_key(&self) -> Result<Key, KeyError> {
        let key = tokio::fs::read_to_string(self).await?;
        SecretKey::<p256::NistP256>::from_sec1_pem(&key)
            .map_err(|_| KeyError::InvalidFormat(key))
            .map(Key)
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

/// An `IntoKey` implementation that reads a private key from a file on disk.
/// This key is assumed to be in the SEC1 PEM format.
///
/// # Errors
/// This provider can fail if the file is not found or if the file
/// is not in the expected format.
pub struct PrivateKeyFromFile(pub PathBuf);

/// An example implementation of `IntoSignature` that calls out to
/// an arbitrary KMS service to retrieve a key.
pub struct KMSService;
impl IntoSignature for KMSService {
    async fn sign(&self, _message: &[u8]) -> Result<Signature, SigningError> {
        todo!("kms signature")
    }
}

impl IntoKey for PrivateKey {
    type Output = Key;

    async fn get_key(&self) -> Result<Key, KeyError> {
        SecretKey::<p256::NistP256>::from_sec1_pem(&self.0)
            .map_err(|e| {
                tracing::error!("Failed to parse SEC1 PEM: {:?}", e);
                KeyError::InvalidFormat(self.0.clone())
            })
            .map(Key)
    }
}

impl IntoKey for PrivateKeyFromFile {
    type Output = Key;

    async fn get_key(&self) -> Result<Key, KeyError> {
        let pem_content = std::fs::read_to_string(&self.0)?;
        PrivateKey(pem_content).get_key().await
    }
}

#[cfg(test)]
mod tests {
    use std::{
        path::Path,
        sync::{Arc, Mutex},
        time::{Duration, SystemTime},
    };

    use base64::{Engine, engine::general_purpose::STANDARD};
    use futures::TryStreamExt;
    use p256::{SecretKey, ecdsa::Signature};
    use tracing_test::traced_test;

    use super::{
        ExpirableKey, IntoKey, IntoSignature, JwtUser, KMSService, Key, PrivateKey, TimeCachingKey,
    };
    use crate::{AuthorizationContext, KeyError, PrivyClient};

    #[tokio::test]
    async fn jwt() {
        let jwt = JwtUser(todo!(), "test".to_string());
        let key = jwt.sign(&[0, 1, 2, 3]).await.unwrap();
        println!("{key:?}");
    }

    #[tokio::test]
    async fn cached_jwt() {
        let jwt = JwtUser(todo!(), "test".to_string());
        let cached_jwt = TimeCachingKey::new(jwt);
        let key = cached_jwt.get_key().await.unwrap();
        println!("{key:?}");
    }

    #[tokio::test]
    async fn cached_private_key() {
        let key = PrivateKey(include_str!("../private_key.pem").to_string());
        let key = key.get_key().await.unwrap();
        println!("{key:?}");
    }

    #[tokio::test]
    async fn custom_kms() {
        let kms = KMSService;
        let key = kms.sign(&[0, 1, 2, 3]).await.unwrap();
        println!("{key:?}");
    }

    #[tokio::test]
    #[traced_test]
    async fn authorization_context() {
        let client = Arc::new(
            PrivyClient::new(
                env!("PRIVY_APP_ID").to_string(),
                env!("PRIVY_APP_SECRET").to_string(),
            )
            .unwrap(),
        );

        let mut ctx = AuthorizationContext::new();

        ctx.push(Path::new("private_key.pem"));

        let sigs = ctx
            .sign(&[0, 1, 2, 3])
            .try_collect::<Vec<_>>()
            .await
            .expect("passes");

        assert!(!sigs.is_empty());
    }

    #[tokio::test]
    #[traced_test]
    async fn expiring_key() {
        struct FakeExpiring(Arc<Mutex<u32>>);
        impl IntoKey for FakeExpiring {
            type Output = ExpirableKey;

            async fn get_key(&self) -> Result<Self::Output, KeyError> {
                *self.0.lock().unwrap() += 1;

                let mut rng = rand::rng();

                Ok(ExpirableKey(
                    Key(SecretKey::random(&mut rng)),
                    SystemTime::now()
                        .checked_add(Duration::from_secs(1))
                        .unwrap(),
                ))
            }
        }

        let mut ctx = AuthorizationContext::new();
        let counter = Arc::new(Mutex::new(0));
        ctx.push(TimeCachingKey::new(FakeExpiring(counter.clone())));

        let sig_a = ctx
            .sign(&[0, 1, 2, 3])
            .try_collect::<Vec<_>>()
            .await
            .expect("passes");

        let sig_b = ctx
            .sign(&[0, 1, 2, 3])
            .try_collect::<Vec<_>>()
            .await
            .expect("passes");

        // we expect only one hit, since the key is cached
        assert_eq!(*counter.lock().unwrap(), 1);
        // they should be identical
        assert_eq!(sig_a, sig_b);

        // expires in 1s, lets sleep 2 to be safe
        tokio::time::sleep(Duration::from_secs(2)).await;

        let sig_c = ctx
            .sign(&[0, 1, 2, 3])
            .try_collect::<Vec<_>>()
            .await
            .expect("passes");

        // we expect a second hit, since we missed the window
        assert_eq!(*counter.lock().unwrap(), 2);
        // the impl above generates a new key, so a new signature
        assert_ne!(sig_a, sig_c);
    }
}
