use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::RwLock;

const EXPIRY_BUFFER: Duration = Duration::from_secs(60);

pub type Error = ();

#[derive(Clone)]
pub struct Key(String);

pub trait IntoKey {
    fn get_key(&self) -> impl Future<Output = Result<Key, Error>> + Send;
}

pub trait IntoSignature {
    fn sign(&self, message: &[u8]) -> impl Future<Output = Result<Vec<u8>, Error>> + Send;
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
    async fn sign(&self, _message: &[u8]) -> Result<Vec<u8>, Error> {
        tracing::debug!("signing with key {}", self.0);
        todo!("impl signature for a type of key")
    }
}

impl<T> IntoSignature for T
where
    T: IntoKey + Sync,
{
    async fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Error> {
        let key = self.get_key().await?;
        key.sign(message).await
    }
}

pub struct PrivateKey(String);

pub struct KMSService;
impl IntoSignature for KMSService {
    async fn sign(&self, _message: &[u8]) -> Result<Vec<u8>, Error> {
        todo!("kms signature")
    }
}

impl IntoKey for PrivateKey {
    async fn get_key(&self) -> Result<Key, Error> {
        Ok(Key(self.0.clone()))
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
    }

    #[tokio::test]
    async fn cached_private_key() {
        let key = PrivateKey("test".to_string());
        let key = key.get_key().await.unwrap();
    }

    #[tokio::test]
    async fn custom_kms() {
        let kms = KMSService;
        let key = kms.sign(&[0, 1, 2, 3]).await.unwrap();
    }
}
