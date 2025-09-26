//! privy_hpke
//!
//! This module provides a helper that manages the ephemeral key decryption process for
//! Privy's secure information exchange protocol. For more information please see the
//! [`PrivyHpke`] struct documentation.

use base64::Engine;
use hpke::{
    Deserializable, Kem, OpModeR, Serializable, aead::ChaCha20Poly1305, kdf::HkdfSha256,
    kem::DhP256HkdfSha256,
};
use p256::{PublicKey, elliptic_curve::SecretKey, pkcs8::DecodePrivateKey};
use spki::EncodePublicKey;

use crate::KeyError;

/// An ephemeral HPKE (Hybrid Public Key Encryption) manager for secure key exchange with the Privy API.
///
/// # Overview
///
/// This struct implements a complete HPKE client for Privy's JWT authentication exchange system.
/// It generates an ephemeral P-256 keypair and handles the cryptographic handshake required to
/// securely obtain temporary authorization keys from Privy's backend.
///
/// The HPKE exchange follows this flow:
/// 1. **Client Setup**: Generate ephemeral P-256 keypair
/// 2. **Key Advertisement**: Send SPKI-formatted public key to Privy API
/// 3. **Server Response**: Receive encapsulated key + encrypted authorization key
/// 4. **Key Decryption**: Use HPKE to decrypt the authorization key
/// 5. **Key Parsing**: Parse the decrypted PKCS#8 DER key for signing operations
///
/// # Cryptographic Specifications
///
/// The implementation follows RFC 9180 HPKE specification with these exact algorithms:
/// - **KEM (Key Encapsulation Mechanism)**: DHKEM(P-256, HKDF-SHA256)
/// - **KDF (Key Derivation Function)**: HKDF-SHA256
/// - **AEAD (Authenticated Encryption)**: `ChaCha20Poly1305`
/// - **Curve**: NIST P-256 (secp256r1)
/// - **Key Format**: SPKI (Subject Public Key Info) for advertisement
/// - **Decrypted Key Format**: PKCS#8 DER encoded private keys
///
/// # Usage Pattern
///
/// ```rust,no_run
/// use privy_rs::{PrivyHpke, generated::types::WithEncryptionEncryptedAuthorizationKey};
///
/// # async fn get_encrypted_authorization_key() -> WithEncryptionEncryptedAuthorizationKey {
/// #     todo!()
/// # }
///
/// # async fn jwt_exchange_example() -> Result<(), Box<dyn std::error::Error>> {
/// // 1. Create ephemeral HPKE manager
/// let hpke = PrivyHpke::new();
///
/// // 2. Get public key for API request
/// let recipient_public_key = hpke.public_key()?;
///
/// // 3. Send to Privy API with JWT
/// let encrypted_authorization_key = get_encrypted_authorization_key().await;
///
/// // 4. Decrypt the authorization key
/// let auth_key = hpke.decrypt(
///     &encrypted_authorization_key.encapsulated_key,
///     &encrypted_authorization_key.ciphertext,
/// )?;
///
/// // 5. Use auth_key for wallet signing operations
/// # Ok(())
/// # }
/// ```
pub struct PrivyHpke {
    /// The ephemeral HPKE private key used for decryption operations.
    ///
    /// This key is generated fresh for each JWT exchange and is never persisted.
    /// It's used in conjunction with Privy's encapsulated key to derive the
    /// shared secret for decrypting authorization keys.
    private_key: <DhP256HkdfSha256 as Kem>::PrivateKey,

    /// The corresponding HPKE public key sent to Privy's API.
    ///
    /// This key is formatted as SPKI and base64-encoded before transmission.
    /// Privy uses this key to perform HPKE encryption of authorization keys,
    /// ensuring only this specific client can decrypt the response.
    public_key: <DhP256HkdfSha256 as Kem>::PublicKey,
}

impl PrivyHpke {
    /// Creates a new ephemeral HPKE manager with a cryptographically secure P-256 keypair.
    #[must_use]
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let (private_key, public_key) = DhP256HkdfSha256::gen_keypair(&mut rng);
        Self {
            private_key,
            public_key,
        }
    }

    /// Replace the cryptographic entropy source with a custom seed and a fast PRNG.
    ///
    /// # Security
    /// This should only be used for testing purposes.
    #[cfg(test)]
    pub(crate) fn new_with_seed(seed: u64) -> Self {
        use rand::SeedableRng;
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        let (private_key, public_key) = DhP256HkdfSha256::gen_keypair(&mut rng);
        Self {
            private_key,
            public_key,
        }
    }

    /// Formats the public key as a base64-encoded SPKI structure for Privy API requests.
    ///
    /// Uses the `RustCrypto` ecosystem (p256 + spki crates) to generate a standards-compliant
    /// DER-encoded Subject Public Key Info structure. This approach is:
    /// - **Correct by Construction**: Guaranteed ASN.1/DER compliance
    /// - **Secure**: Peer-reviewed cryptographic libraries
    /// - **Maintainable**: High-level, declarative API
    ///
    /// # SPKI Format Details
    ///
    /// The returned SPKI follows X.509 standards and is compatible with:
    /// - Privy's Java SDK using `BouncyCastle`'s `SubjectPublicKeyInfo.getInstance()`
    /// - OpenSSL's P-256 SPKI format
    /// - RFC 5480 Elliptic Curve Cryptography Subject Public Key Info
    ///
    /// # Return Format
    ///
    /// Returns a base64-encoded string (RFC 4648) of the DER-encoded SPKI structure.
    /// Example output: `MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE...`
    ///
    /// # Errors
    ///
    /// Returns a `KeyError::InvalidFormat` if:
    /// - The internal public key cannot be serialized to SEC1 format
    /// - SEC1 bytes cannot be parsed as a valid P-256 point
    /// - SPKI DER encoding fails
    /// - Base64 encoding fails
    pub fn public_key(&self) -> Result<String, KeyError> {
        // 1. Extract the raw SEC1-encoded bytes from the HPKE public key
        let public_key_bytes = self.public_key.to_bytes();

        // 2. Parse these bytes into a p256::PublicKey for validation and type safety
        let p256_pk = PublicKey::from_sec1_bytes(&public_key_bytes)
            .map_err(|_| KeyError::InvalidFormat("invalid SEC1 public key point".to_string()))?;

        // 3. Use the EncodePublicKey trait to generate the DER-encoded SPKI
        let spki_doc = p256_pk
            .to_public_key_der()
            .map_err(|_| KeyError::InvalidFormat("SPKI DER encoding failed".to_string()))?;

        // 4. Encode the DER bytes as base64
        Ok(base64::engine::general_purpose::STANDARD.encode(spki_doc.as_bytes()))
    }

    /// Decrypts an HPKE-encrypted authorization key from Privy's authentication response.
    ///
    /// # HPKE Decryption Process
    ///
    /// This method implements the complete HPKE receiver workflow according to RFC 9180:
    ///
    /// 1. **Input Validation**: Decode base64 encapsulated key and ciphertext
    /// 2. **Key Deserialization**: Parse the encapsulated key into P-256 point
    /// 3. **HPKE Setup**: Derive shared secret using DHKEM(P-256, HKDF-SHA256)
    /// 4. **Context Derivation**: Create AEAD context with HKDF-SHA256
    /// 5. **Authenticated Decryption**: Decrypt using `ChaCha20Poly1305`
    /// 6. **Key Parsing**: Parse decrypted PKCS#8 DER private key
    ///
    /// # Arguments
    ///
    /// * `encapsulated_key` - Base64-encoded HPKE encapsulated key (65 bytes decoded)
    ///   - This is Privy's ephemeral P-256 public key used for key agreement
    ///   - Format: Uncompressed point (0x04 + 32-byte X + 32-byte Y)
    ///
    /// * `ciphertext` - Base64-encoded HPKE ciphertext containing encrypted authorization key
    ///   - Contains the encrypted PKCS#8 DER private key
    ///   - Protected by `ChaCha20Poly1305` authenticated encryption
    ///
    /// # Return Value
    ///
    /// Returns a `SecretKey<p256::NistP256>` ready for ECDSA signing operations.
    /// The key can be used directly with `p256::ecdsa::SigningKey` for wallet signing.
    ///
    /// # Errors
    ///
    /// Returns `KeyError` variants for different failure modes:
    /// - `InvalidFormat`: Malformed base64, invalid key points, or bad DER
    /// - `HpkeDecryption`: HPKE setup failure, authentication failure, or decryption failure
    ///
    /// # Example Usage
    ///
    /// ```rust,no_run
    /// use privy_rs::{PrivyHpke, generated::types::WithEncryptionEncryptedAuthorizationKey};
    ///
    /// # async fn example(encrypted_authorization_key: WithEncryptionEncryptedAuthorizationKey) -> Result<(), Box<dyn std::error::Error>> {
    /// let hpke = PrivyHpke::new();
    ///
    /// let auth_key = hpke.decrypt(
    ///     &encrypted_authorization_key.encapsulated_key,
    ///     &encrypted_authorization_key.ciphertext
    /// )?;
    ///
    /// // auth_key can now be used for signing wallet transactions
    /// # Ok(())
    /// # }
    /// ```
    pub fn decrypt(
        self,
        encapsulated_key: &str,
        ciphertext: &str,
    ) -> Result<SecretKey<p256::NistP256>, KeyError> {
        let encapped_key_bytes = base64::engine::general_purpose::STANDARD
            .decode(encapsulated_key)
            .map_err(|_| KeyError::InvalidFormat("base64 encapsulated key".to_string()))?;

        let ciphertext_bytes = base64::engine::general_purpose::STANDARD
            .decode(ciphertext)
            .map_err(|_| KeyError::InvalidFormat("base64 ciphertext".to_string()))?;

        tracing::debug!(
            "Deserializing encapsulated key len {}: {:?}",
            encapped_key_bytes.len(),
            encapped_key_bytes
        );

        let encapped_key = <DhP256HkdfSha256 as Kem>::EncappedKey::from_bytes(&encapped_key_bytes)
            .map_err(|e| {
                tracing::error!("Failed to deserialize encapsulated key: {e:?}");
                KeyError::InvalidFormat("encapsulated key".to_string())
            })?;

        // Set up HPKE context for decryption
        let mut context = hpke::setup_receiver::<ChaCha20Poly1305, HkdfSha256, DhP256HkdfSha256>(
            &OpModeR::Base,
            &self.private_key,
            &encapped_key,
            &[],
        )
        .map_err(|e| {
            tracing::error!("HPKE setup failed: {:?}", e);
            KeyError::HpkeDecryption(format!("HPKE setup failed: {e:?}"))
        })?;

        // Decrypt the authorization key using the ciphertext
        let decrypted_key_bytes = context.open(&ciphertext_bytes, &[]).map_err(|e| {
            tracing::error!("HPKE decryption failed: {:?}", e);
            KeyError::HpkeDecryption(format!("HPKE decryption failed: {e:?}"))
        })?;

        // Parse the decrypted bytes as a UTF-8 base64 DER string, then parse as a private key
        let key_b64 = String::from_utf8(decrypted_key_bytes)
            .map_err(|_| KeyError::InvalidFormat("decrypted key is not valid UTF-8".to_string()))?;

        tracing::debug!("Decrypted authorization key (base64 DER): {}", key_b64);

        // Decode the base64 to get DER bytes
        let der_bytes = base64::engine::general_purpose::STANDARD
            .decode(&key_b64)
            .map_err(|_| {
                KeyError::InvalidFormat("decrypted key is not valid base64".to_string())
            })?;

        // Parse as PKCS#8 DER format (which is what the output format appears to be)
        SecretKey::<p256::NistP256>::from_pkcs8_der(&der_bytes).map_err(|e| {
            tracing::error!("Failed to parse decrypted PKCS#8 DER key: {:?}", e);
            KeyError::InvalidFormat("decrypted PKCS#8 DER key".to_string())
        })
    }
}

impl Default for PrivyHpke {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use spki::DecodePublicKey;
    use test_case::test_case;

    use super::*;

    #[test_case(0, "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAECT+o7IjvJ+4MjHTU51k5HLoXT9WKzjJKbqkGA3bcvx+ESEbM/wtxRDsptOMcsP+Vn60KdYOjIyLAU/P96CB2lA==" ; "zero")]
    #[test_case(1, "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE5C1LvDxhkHINqB7lRM47O+sUIKTs/2YiPoNOQaRH2tnkhUjRC1x+g9yo0UZr/HzdJKNMAkSXRovCzovSr0jL3A==" ; "one")]
    #[test_case(10, "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAECh6n0GOhDIloBuKZWx2/tPG3rX6oNuQdzH666gAYINFrZcC+GB/zICKGq+f7iXeobumsQiz38X8KKmOQoYkryA==" ; "ten")]
    fn test_generate_key(seed: u64, expected: &str) {
        let hpke = super::PrivyHpke::new_with_seed(seed);
        let public_key = hpke.public_key().unwrap();
        assert_eq!(public_key, expected);
    }

    #[test]
    fn test_spki_round_trip() {
        // Generate a fresh key pair for the test using a deterministic seed
        let hpke = PrivyHpke::new_with_seed(42);

        // Generate SPKI using our new library-based method
        let spki_b64 = hpke.public_key().unwrap();
        let spki_der = base64::engine::general_purpose::STANDARD
            .decode(&spki_b64)
            .unwrap();

        // Use the DecodePublicKey trait to parse the DER back into a key
        let decoded_pk = PublicKey::from_public_key_der(&spki_der).unwrap();

        // Extract the original key bytes for comparison
        let original_pk_bytes = hpke.public_key.to_bytes();
        let original_pk = PublicKey::from_sec1_bytes(&original_pk_bytes).unwrap();

        // Assert that the original key and the decoded key are identical
        assert_eq!(original_pk, decoded_pk);
    }

    #[test]
    fn test_spki_structure_correctness() {
        let hpke = PrivyHpke::new_with_seed(1337);
        let spki_b64 = hpke.public_key().unwrap();
        let spki_der = base64::engine::general_purpose::STANDARD
            .decode(&spki_b64)
            .unwrap();

        // Verify SPKI structure basics
        assert_eq!(spki_der.len(), 91, "SPKI should be exactly 91 bytes");

        // Verify it starts with SEQUENCE tag and correct length
        assert_eq!(spki_der[0], 0x30, "Should start with SEQUENCE tag");
        assert_eq!(
            spki_der[1], 0x59,
            "SEQUENCE should have 89 bytes of content"
        );

        // Verify we can successfully parse it back
        let parsed_key = PublicKey::from_public_key_der(&spki_der).unwrap();

        // Verify the parsed key matches the original
        let original_pk_bytes = hpke.public_key.to_bytes();
        let original_pk = PublicKey::from_sec1_bytes(&original_pk_bytes).unwrap();
        assert_eq!(original_pk, parsed_key);
    }

    #[test]
    fn test_spki_base64_format() {
        let hpke = PrivyHpke::new_with_seed(999);
        let spki_b64 = hpke.public_key().unwrap();

        // Verify it's valid base64
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&spki_b64)
            .expect("Should be valid base64");

        // Verify the decoded length is correct for P-256 SPKI
        assert_eq!(decoded.len(), 91);

        // Verify round-trip through base64
        let re_encoded = base64::engine::general_purpose::STANDARD.encode(&decoded);
        assert_eq!(spki_b64, re_encoded);
    }

    #[test]
    fn test_error_handling_invalid_key() {
        // This test ensures our error handling works correctly
        // We can't easily create an invalid HPKE key, but we can test the p256 parsing path
        use p256::PublicKey;

        // Test with invalid SEC1 bytes (wrong length)
        let invalid_bytes = vec![0x04; 32]; // Too short
        let result = PublicKey::from_sec1_bytes(&invalid_bytes);
        assert!(result.is_err(), "Should reject invalid SEC1 bytes");

        // Test with invalid SEC1 bytes (wrong format)
        let invalid_bytes = vec![0x02; 65]; // Wrong format indicator
        let result = PublicKey::from_sec1_bytes(&invalid_bytes);
        assert!(result.is_err(), "Should reject invalid format indicator");
    }

    #[test]
    #[ignore]
    fn test_hpke_decrypt_success() {}

    #[test]
    #[ignore]
    fn test_hpke_decrypt_invalid_ciphertext() {}

    #[test]
    #[ignore]
    fn test_hpke_decrypt_invalid_encapsulated_key() {}
}
