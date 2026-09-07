use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::RngCore;
use zeroize::Zeroize;

pub const NONCE_LEN: usize = 12;

#[derive(Debug)]
pub enum CipherError {
    EncryptionFailed,
    DecryptionFailed,
}

impl std::fmt::Display for CipherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CipherError::EncryptionFailed => write!(f, "Encryption operation failed"),
            CipherError::DecryptionFailed => write!(f, "Decryption operation failed or invalid tag"),
        }
    }
}

impl std::error::Error for CipherError {}

/// Encrypts plaintext bytes using AES-256-GCM.
/// Returns a tuple containing (ciphertext, nonce).
pub fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<(Vec<u8>, [u8; NONCE_LEN]), CipherError> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CipherError::EncryptionFailed)?;

    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| CipherError::EncryptionFailed)?;

    Ok((ciphertext, nonce_bytes))
}

/// Decrypts ciphertext bytes using AES-256-GCM and verifies authenticity tag.
pub fn decrypt(
    key: &[u8; 32],
    ciphertext: &[u8],
    nonce_bytes: &[u8; NONCE_LEN],
) -> Result<Vec<u8>, CipherError> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CipherError::DecryptionFailed)?;
    let nonce = Nonce::from_slice(nonce_bytes);

    let mut plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| CipherError::DecryptionFailed)?;

    let result = plaintext.clone();
    plaintext.zeroize();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = [0x42u8; 32];
        let secret_data = b"my_super_secret_totp_key";

        let (ciphertext, nonce) = encrypt(&key, secret_data).unwrap();
        assert_ne!(ciphertext, secret_data);

        let decrypted = decrypt(&key, &ciphertext, &nonce).unwrap();
        assert_eq!(decrypted, secret_data);
    }

    #[test]
    fn test_decrypt_tampered_ciphertext() {
        let key = [0x42u8; 32];
        let secret_data = b"my_super_secret_totp_key";

        let (mut ciphertext, nonce) = encrypt(&key, secret_data).unwrap();
        ciphertext[0] ^= 0xFF; // Corrupt ciphertext byte

        let result = decrypt(&key, &ciphertext, &nonce);
        assert!(result.is_err());
    }
}