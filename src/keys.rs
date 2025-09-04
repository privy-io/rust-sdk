use std::{
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
    time::{Duration, SystemTime},
};

use futures::{Stream, StreamExt};
use p256::{
    ecdsa::{Signature, SigningKey, signature::hazmat::PrehashSigner},
    elliptic_curve::{SecretKey, generic_array::GenericArray},
};
use privy_api::types::builder::AuthenticateBody;
use tokio::sync::RwLock;

use crate::{KeyError, SigningError};

const EXPIRY_BUFFER: Duration = Duration::from_secs(60);
const SIGNATURE_RESOLUTION_CONCURRENCY: usize = 10;

pub struct AuthorizationContext(Vec<Box<dyn IntoSignatureBoxed>>, usize);

impl Default for AuthorizationContext {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthorizationContext {
    #[must_use]
    pub fn new() -> Self {
        Self(Vec::default(), SIGNATURE_RESOLUTION_CONCURRENCY)
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
    /// # use privy_rust::{AuthorizationContext, JwtUser, IntoSignature, PrivateKey, PrivySigner};
    /// # use p256::ecdsa::signature::SignerMut;
    /// # use p256::ecdsa::Signature;
    /// # use p256::elliptic_curve::SecretKey;
    /// # use std::time::Duration;
    /// # use std::sync::Arc;
    /// # async fn foo() {
    /// let privy = PrivySigner::new("app_id".to_string(), "app_secret".to_string(), "wallet_id".to_string(), "public_key".to_string()).unwrap();
    /// let jwt = JwtUser(Arc::new(privy), "test".to_string());
    /// let key = PrivateKey("test".to_string());
    /// let mut context = AuthorizationContext::new();
    /// context.push(jwt);
    /// context.push(key);
    /// # }
    /// ```
    pub fn push<T: IntoSignature + 'static>(&mut self, key: T) {
        self.0.push(Box::new(key));
    }

    /// Sign a message with all the keys in the context.
    /// This produces a stream which yields values as they
    /// become available. You can collect it into a vec.
    /// This function will resolve all signatures concurrently,
    /// according to the policy set in `AuthorizationContext`.
    ///
    /// ```rust
    /// # use privy_rust::{AuthorizationContext, JwtUser, IntoSignature, PrivySigner};
    /// # use p256::ecdsa::signature::SignerMut;
    /// # use p256::ecdsa::Signature;
    /// # use p256::elliptic_curve::SecretKey;
    /// # use std::time::Duration;
    /// # use std::sync::Arc;
    /// # use futures::stream::StreamExt;
    /// # async fn foo() {
    /// let privy = PrivySigner::new(
    ///     "app_id".to_string(),
    ///     "app_secret".to_string(),
    ///     "wallet_id".to_string(),
    ///     "public_key".to_string(),
    /// )
    /// .unwrap();
    /// let jwt = JwtUser(Arc::new(privy), "test".to_string());
    /// let mut context = AuthorizationContext::new();
    /// context.push(jwt);
    /// let key = context.sign(&[0, 1, 2, 3]).collect::<Vec<_>>().await;
    /// assert_eq!(key.len(), 1);
    /// # }
    /// ```
    ///
    /// You can also use `try_collect` to get a `Result<Vec<_>, Error>`,
    /// failing on the first error.
    ///
    /// ```rust
    /// # use privy_rust::{AuthorizationContext, JwtUser, IntoSignature, PrivySigner};
    /// # use p256::ecdsa::signature::SignerMut;
    /// # use p256::ecdsa::Signature;
    /// # use p256::elliptic_curve::SecretKey;
    /// # use std::time::Duration;
    /// # use std::sync::Arc;
    /// # use futures::stream::TryStreamExt;
    /// # async fn foo() {
    /// let privy = PrivySigner::new(
    ///     "app_id".to_string(),
    ///     "app_secret".to_string(),
    ///     "wallet_id".to_string(),
    ///     "public_key".to_string(),
    /// )
    /// .unwrap();
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
        futures::stream::iter(self.0.iter())
            .map(|key| key.sign_boxed(message))
            .buffer_unordered(self.1)
    }

    /// Exercise the signing mechanism to validate that all keys
    /// are valid and can produce signatures. Returns a vector
    /// of errors. An empty vector indicates that all keys are
    /// valid.
    ///
    /// ```
    /// # use privy_rust::{AuthorizationContext, JwtUser, IntoSignature, PrivySigner};
    /// # use p256::ecdsa::signature::SignerMut;
    /// # use p256::ecdsa::Signature;
    /// # use p256::elliptic_curve::SecretKey;
    /// # use std::time::Duration;
    /// # use std::sync::Arc;
    /// # async fn foo() {
    /// let privy = PrivySigner::new(
    ///     "app_id".to_string(),
    ///     "app_secret".to_string(),
    ///     "wallet_id".to_string(),
    ///     "public_key".to_string(),
    /// )
    /// .unwrap();
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
            .filter_map(|r| async move { r.err() })
            .collect::<Vec<_>>()
            .await
    }
}

type Key = SecretKey<p256::NistP256>;

pub trait IntoKey {
    /// Get a key from the `IntoKey` source.
    fn get_key(&self) -> impl Future<Output = Result<Key, KeyError>> + Send;
}

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

impl<T> IntoSignature for T
where
    T: IntoKey + Sync,
{
    async fn sign(&self, message: &[u8]) -> Result<Signature, SigningError> {
        let key = self.get_key().await?;
        key.sign(message).await
    }
}

pub trait ExpiringKey: IntoKey {
    fn expires_at(&self) -> SystemTime;
}

/// Rust has a concept of 'object safety' and `IntoSignature` is not object safe.
/// What we can do, however, is to blanket impl `IntoSignatureBoxed` for all types
/// that implement `IntoSignature`.
trait IntoSignatureBoxed {
    fn sign_boxed<'a>(
        &'a self,
        message: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = Result<Signature, SigningError>> + Send + 'a>>;
}

impl<T: IntoSignature + 'static> IntoSignatureBoxed for T {
    fn sign_boxed<'a>(
        &'a self,
        message: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = Result<Signature, SigningError>> + Send + 'a>> {
        Box::pin(self.sign(message))
    }
}

pub struct TimeCachingKey<T: IntoKey>(T, Arc<RwLock<Option<(SystemTime, Key)>>>);

impl<T: IntoKey + Sync> TimeCachingKey<T> {
    pub fn new(key: T) -> Self {
        Self(key, Arc::new(RwLock::new(None)))
    }
}

impl<T: IntoKey + Sync> IntoKey for TimeCachingKey<T> {
    async fn get_key(&self) -> Result<Key, KeyError> {
        {
            let valid = self.1.read().await;
            if let Some((_, key)) = valid.as_ref().filter(|(time, _)| time > &SystemTime::now()) {
                return Ok(key.clone());
            }
        }

        let key = self.0.get_key().await?;

        {
            let mut state = self.1.write().await;
            // TODO: set this time from the key
            let now = SystemTime::now();
            *state = Some((now + EXPIRY_BUFFER, key.clone()));
        }

        Ok(key)
    }
}

pub struct JwtUser(pub Arc<crate::PrivySigner>, pub String);

impl IntoKey for JwtUser {
    async fn get_key(&self) -> Result<Key, KeyError> {
        tracing::debug!("getting key from jwt {}", self.1);

        let auth = match self
            .0
            .authenticate()
            .body(AuthenticateBody::default().user_jwt(self.1.clone()))
            .send()
            .await
        {
            Ok(r) => r.into_inner(),
            Err(privy_api::Error::UnexpectedResponse(response)) => {
                tracing::error!("unexpected response {:?}", response.text().await);
                return Err(KeyError::Unknown);
            }
            Err(e) => {
                tracing::error!("error {:?}", e);
                return Err(KeyError::Unknown);
            }
        };

        let key = match auth {
            privy_api::types::AuthenticateResponse::WithoutEncryption {
                authorization_key, ..
            } => authorization_key,
            privy_api::types::AuthenticateResponse::WithEncryption { .. } => {
                todo!()
            }
        };

        tracing::info!("got key {:?}", key);

        Key::from_bytes(GenericArray::from_slice(&key.into_bytes())).map_err(|_| {
            tracing::error!("invalid key");
            KeyError::Unknown
        })
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
        let signature: Signature = signing_key
            .sign_prehash(&hashed)
            .map_err(|_| SigningError::Unknown)?;

        tracing::debug!("ECDSA signature generated using deterministic RFC 6979");

        Ok(signature)
    }
}

impl IntoKey for &Path {
    async fn get_key(&self) -> Result<Key, KeyError> {
        let key = tokio::fs::read_to_string(self).await?;
        SecretKey::<p256::NistP256>::from_sec1_pem(&key).map_err(|_| KeyError::InvalidFormat(key))
    }
}

impl IntoSignature for Signature {
    async fn sign(&self, _message: &[u8]) -> Result<Signature, SigningError> {
        Ok(*self)
    }
}

pub struct PrivateKey(pub String);

pub struct PrivateKeyFromFile(pub PathBuf);

pub struct KMSService;
impl IntoSignature for KMSService {
    async fn sign(&self, _message: &[u8]) -> Result<Signature, SigningError> {
        todo!("kms signature")
    }
}

impl IntoKey for PrivateKey {
    async fn get_key(&self) -> Result<Key, KeyError> {
        SecretKey::<p256::NistP256>::from_sec1_pem(&self.0)
            .map_err(|_| KeyError::InvalidFormat(self.0.clone()))
    }
}

impl IntoKey for PrivateKeyFromFile {
    async fn get_key(&self) -> Result<Key, KeyError> {
        let pem_content = std::fs::read_to_string(&self.0)?;

        let key = SecretKey::<p256::NistP256>::from_sec1_pem(&pem_content).map_err(|e| {
            tracing::error!("Failed to parse SEC1 PEM: {:?}", e);
            KeyError::InvalidFormat(pem_content)
        })?;

        tracing::debug!("Successfully parsed private key from file: {:?}", self.0);

        Ok(key)
    }
}

#[cfg(test)]
mod tests {
    use std::{path::Path, sync::Arc};

    use base64::{Engine, engine::general_purpose::STANDARD};
    use futures::TryStreamExt;
    use p256::{ecdsa::Signature, elliptic_curve::generic_array::GenericArray};
    use tracing_test::traced_test;

    use crate::{
        AuthorizationContext, PrivySigner,
        keys::{IntoKey, IntoSignature, JwtUser, KMSService, PrivateKey, TimeCachingKey},
    };

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
            PrivySigner::new(
                env!("PRIVY_APP_ID").to_string(),
                env!("PRIVY_APP_SECRET").to_string(),
                env!("PRIVY_WALLET_ID").to_string(),
                env!("PRIVY_PUBLIC_KEY").to_string(),
            )
            .unwrap(),
        );

        let mut ctx = AuthorizationContext::new();

        ctx.push(Path::new("private_key.pem"));
        ctx.push(JwtUser(client, "my_jwt".to_string()));
        // ctx.push(PrivateKey("my_key".to_string()));
        ctx.push(Signature::from_bytes(GenericArray::from_slice(&STANDARD.decode("J7GLk/CIqvCNCOSJ8sUZb0rCsqWF9l1H1VgYfsAd1ew2uBJHE5hoY+kV7CSzdKkgOhtdvzj22gXA7gcn5gSqvQ==").unwrap())).expect("right size"));

        let sigs = ctx
            .sign(&[0, 1, 2, 3])
            .try_collect::<Vec<_>>()
            .await
            .expect("passes");

        assert!(!sigs.is_empty());
    }
}
