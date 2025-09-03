use p256::{
    ecdsa::{Signature, SigningKey, signature::SignerMut},
    elliptic_curve::SecretKey,
};
use std::{
    path::PathBuf,
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::RwLock;

const EXPIRY_BUFFER: Duration = Duration::from_secs(60);

pub type Error = ();

type Key = SecretKey<p256::NistP256>;

pub trait IntoKey {
    fn get_key(&self) -> impl Future<Output = Result<Key, Error>> + Send;
}

pub trait IntoSignature {
    fn sign(&self, message: &[u8]) -> impl Future<Output = Result<Signature, Error>> + Send;
}

pub struct TimeCachingKey<T: IntoKey>(T, Arc<RwLock<Option<(SystemTime, Key)>>>);

impl<T: IntoKey + Sync> TimeCachingKey<T> {
    pub fn new(key: T) -> Self {
        Self(key, Arc::new(RwLock::new(None)))
    }
}

impl<T: IntoKey + Sync> IntoKey for TimeCachingKey<T> {
    async fn get_key(&self) -> Result<Key, Error> {
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

pub struct JwtUser(String);

impl IntoKey for JwtUser {
    async fn get_key(&self) -> Result<Key, Error> {
        tracing::debug!("getting key from jwt {}", self.0);
        todo!("get the key from the authorization endpoint in privy")
    }
}

impl IntoSignature for Key {
    async fn sign(&self, message: &[u8]) -> Result<Signature, Error> {
        tracing::debug!("signing with key {:?}", self);
        let mut sk = SigningKey::from(self.clone());
        Ok(sk.sign(message))
    }
}

impl<T> IntoSignature for T
where
    T: IntoKey + Sync,
{
    async fn sign(&self, message: &[u8]) -> Result<Signature, Error> {
        let key = self.get_key().await?;
        key.sign(message).await
    }
}

pub struct PrivateKey(String);

pub struct PrivateKeyFromFile(pub PathBuf);

pub struct KMSService;
impl IntoSignature for KMSService {
    async fn sign(&self, _message: &[u8]) -> Result<Signature, Error> {
        todo!("kms signature")
    }
}

impl IntoKey for PrivateKey {
    async fn get_key(&self) -> Result<Key, Error> {
        Ok(SecretKey::<p256::NistP256>::from_sec1_pem(&self.0).unwrap())
    }
}

impl IntoKey for PrivateKeyFromFile {
    async fn get_key(&self) -> Result<Key, Error> {
        std::fs::read_to_string(&self.0)
            .map_err(|_| ())
            .and_then(|s| SecretKey::<p256::NistP256>::from_sec1_pem(&s).map_err(|_| ()))
    }
}

#[cfg(test)]
mod tests {
    use crate::keys::{IntoKey, IntoSignature, JwtUser, KMSService, PrivateKey, TimeCachingKey};

    #[tokio::test]
    async fn jwt() {
        let jwt = JwtUser("test".to_string());
        let key = jwt.sign(&[0, 1, 2, 3]);
    }

    #[tokio::test]
    async fn cached_jwt() {
        let jwt = JwtUser("test".to_string());
        let cached_jwt = TimeCachingKey::new(jwt);
        let key = cached_jwt.get_key().await.unwrap();
        println!("{:?}", key);
    }

    #[tokio::test]
    async fn cached_private_key() {
        let key = PrivateKey(include_str!("../private_key.pem").to_string());
        let key = key.get_key().await.unwrap();
        println!("{:?}", key);
    }

    #[tokio::test]
    async fn custom_kms() {
        let kms = KMSService;
        let key = kms.sign(&[0, 1, 2, 3]).await.unwrap();
        println!("{:?}", key);
    }
}
