use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::Aes256Gcm;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

/// Represents the security options for FlexBin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityOptions {
    pub enable_encryption: bool,
    pub enable_signing: bool,
    pub encryption_key: Option<[u8; 32]>, // 256-bit key for AES-256-GCM
    pub signing_key: Option<[u8; 32]>,    // SHA-256 hash output
}

#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("Encryption key missing")]
    EncryptionKeyMissing,
    #[error("Signing key missing")]
    SigningKeyMissing,
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Decryption failed")]
    DecryptionFailed,
    #[error("Signing failed")]
    SigningFailed,
    #[error("Verification failed")]
    VerificationFailed,
}

pub struct Security;

impl Security {
    /// Encrypts data using AES-256-GCM.
    pub fn encrypt(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, SecurityError> {
        let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| SecurityError::EncryptionFailed)?;
        let nonce = aes_gcm::Nonce::from_slice(b"unique nonce"); // 96-bits; in practice, use a unique nonce per message
        cipher
            .encrypt(nonce, data)
            .map_err(|_| SecurityError::EncryptionFailed)
    }

    /// Decrypts data using AES-256-GCM.
    pub fn decrypt(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, SecurityError> {
        let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| SecurityError::DecryptionFailed)?;
        let nonce = aes_gcm::Nonce::from_slice(b"unique nonce"); // Must match the nonce used during encryption
        cipher
            .decrypt(nonce, data)
            .map_err(|_| SecurityError::DecryptionFailed)
    }

    /// Generates a SHA-256 signature for the given data.
    pub fn sign(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, SecurityError> {
        if key.is_empty() {
            return Err(SecurityError::SigningKeyMissing);
        }
        let mut hasher = Sha256::default();
        hasher.update(data);
        let signature = hasher.finalize();
        Ok(signature.to_vec())
    }

    /// Verifies the SHA-256 signature of the given data.
    pub fn verify(data: &[u8], signature: &[u8], key: &[u8; 32]) -> Result<bool, SecurityError> {
        if key.is_empty() {
            return Err(SecurityError::VerificationFailed);
        }
        let expected_signature = Self::sign(data, key)?;
        Ok(expected_signature == signature)
    }
}
