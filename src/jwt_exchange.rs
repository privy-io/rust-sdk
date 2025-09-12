use std::{
    num::NonZeroUsize,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use p256::{NistP256, elliptic_curve::SecretKey};

use crate::{JwtUser, KeyError, PrivyHpke, generated::types::AuthenticateBody};

const EXPIRY_BUFFER: Duration = Duration::from_secs(60);

/// This needs interior mutability so that we don't have to lock the cache for the
/// entire duration of the network request. Otherwise, in a multi-threaded context,
/// you would only be able to sign a single signature at a time.
#[derive(Debug, Clone)]
pub struct JwtExchange(Arc<Mutex<lru::LruCache<String, (SystemTime, SecretKey<NistP256>)>>>);

impl JwtExchange {
    pub fn new(capacity: NonZeroUsize) -> Self {
        JwtExchange(Arc::new(Mutex::new(lru::LruCache::new(capacity))))
    }

    pub async fn exchange_jwt_for_authorization_key(
        &self,
        jwt_user: &JwtUser,
    ) -> Result<SecretKey<NistP256>, KeyError> {
        let client = &jwt_user.0;
        let jwt = &jwt_user.1;

        {
            let mut cache = self.0.lock().expect("lock poisoned");
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

        tracing::debug!("Starting HPKE JWT exchange for user JWT: {}", jwt);

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
        let body = AuthenticateBody {
            user_jwt: jwt.clone(),
            encryption_type: Some(crate::generated::types::AuthenticateBodyEncryptionType::Hpke),
            recipient_public_key: Some(public_key_b64),
        };

        // Send the authentication request
        let auth = match client.wallets().authenticate_with_jwt(&body).await {
            Ok(r) => r.into_inner(),
            Err(crate::generated::Error::UnexpectedResponse(response)) => {
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
            crate::generated::types::AuthenticateResponse::WithEncryption {
                encrypted_authorization_key,
                expires_at,
                ..
            } => {
                tracing::debug!("Received encrypted authorization key, starting HPKE decryption");

                let key = hpke_manager
                    .decrypt(
                        &encrypted_authorization_key.encapsulated_key,
                        &encrypted_authorization_key.ciphertext,
                    )
                    .map_err(|e| {
                        tracing::error!("HPKE decryption failed: {:?}", e);
                        KeyError::HpkeDecryption(format!("{e:?}"))
                    })?;
                let expiry = SystemTime::UNIX_EPOCH + Duration::from_secs_f64(expires_at);
                (key, expiry)
            }
            crate::generated::types::AuthenticateResponse::WithoutEncryption { .. } => {
                tracing::warn!("Received unencrypted authorization key (fallback mode)");
                unimplemented!()
            }
        };

        {
            let mut cache = self.0.lock().expect("lock poisoned");
            cache.push(jwt.clone(), (expiry, key.clone()));
        }

        tracing::info!("Successfully obtained and parsed authorization key");
        Ok(key)
    }
}
