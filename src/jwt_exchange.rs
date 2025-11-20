use std::{
    num::NonZeroUsize,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use p256::{NistP256, elliptic_curve::SecretKey};

use crate::{JwtUser, KeyError, PrivyHpke, generated::types::AuthenticateBody};

const EXPIRY_BUFFER: Duration = Duration::from_secs(60);

type JwtCache = lru::LruCache<String, (SystemTime, SecretKey<NistP256>)>;

/// This needs interior mutability so that we don't have to lock the cache for the
/// entire duration of the network request. Otherwise, in a multi-threaded context,
/// you would only be able to sign a single signature at a time.
#[derive(Debug, Clone)]
pub struct JwtExchange {
    cache: Arc<Mutex<JwtCache>>,
}

impl JwtExchange {
    pub fn new(capacity: NonZeroUsize) -> Self {
        JwtExchange {
            cache: Arc::new(Mutex::new(lru::LruCache::new(capacity))),
        }
    }

    pub async fn exchange_jwt_for_authorization_key(
        &self,
        jwt_user: &JwtUser,
    ) -> Result<SecretKey<NistP256>, KeyError> {
        let client = &jwt_user.0;
        let jwt = &jwt_user.1;

        {
            let mut cache = self.cache.lock().expect("lock poisoned");
            let expired = if let Some((expiry, key)) = cache.get(jwt) {
                let buffer = *expiry - EXPIRY_BUFFER;
                if buffer > SystemTime::now() {
                    return Ok(key.clone());
                }
                true
            } else {
                false
            };

            if expired {
                // if it was expired, demote it so it is evicted ASAP
                // it costs the same to check if it exists so just mark it unconditionally
                cache.demote(jwt);
            }
        }

        #[cfg(all(feature = "unsafe_debug", debug_assertions))]
        {
            tracing::debug!("Starting HPKE JWT exchange for user JWT: {}", jwt);
        }

        // Get the HPKE manager and format the public key for the API request
        let hpke_manager = PrivyHpke::new();
        let public_key_b64 = hpke_manager.public_key()?;

        tracing::debug!(
            "Generated HPKE public key for authentication request {}",
            public_key_b64
        );

        // Build the authentication request with encryption parameters
        let body = AuthenticateBody {
            user_jwt: jwt.clone(),
            encryption_type: Some(crate::generated::types::AuthenticateBodyEncryptionType::Hpke),
            recipient_public_key: Some(public_key_b64),
        };

        // Send the authentication request
        let auth = match client.wallets().authenticate_with_jwt(&body).await {
            Ok(r) => r.into_inner(),
            Err(e) => {
                tracing::error!("failed to fetch authorization key: {:?}", e);
                return Err(KeyError::Other(Box::new(e)));
            }
        };

        // Process the response based on encryption type
        let (key, expiry) = match auth {
            crate::generated::types::AuthenticateResponse::WithEncryption {
                encrypted_authorization_key,
                expires_at,
                ..
            } => {
                tracing::debug!("Received encrypted authorization key, starting HPKE decryption");

                let key = hpke_manager.decrypt_p256(
                    &encrypted_authorization_key.encapsulated_key,
                    &encrypted_authorization_key.ciphertext,
                )?;

                let expiry = SystemTime::UNIX_EPOCH + Duration::from_secs_f64(expires_at);
                (key, expiry)
            }
            crate::generated::types::AuthenticateResponse::WithoutEncryption { .. } => {
                tracing::warn!("Received unencrypted authorization key (fallback mode)");
                unimplemented!()
            }
        };

        // NOTE: ugly hack ahead
        //
        // privy's caches are a little slow sometimes which means we need to insert an
        // artificial delay to increase the likelihood of the cache being populated.
        // good news is that retries will not need a new key so retries will be fast.
        tokio::time::sleep(Duration::from_millis(1000)).await;

        {
            let mut cache = self.cache.lock().expect("lock poisoned");
            cache.push(jwt.clone(), (expiry, key.clone()));
        }

        tracing::info!("Successfully obtained and parsed authorization key");
        Ok(key)
    }
}
