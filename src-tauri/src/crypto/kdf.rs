use argon2::{Algorithm, Argon2, Params, Version};
use rand::RngCore;
use zeroize::Zeroize;

pub const SALT_LEN: usize = 32;
pub const KEY_LEN: usize = 32;

#[derive(Debug)]
pub enum KdfError {
    DerivationFailed,
}

impl std::fmt::Display for KdfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KdfError::DerivationFailed => write!(f, "Key derivation failed"),
        }
    }
}

impl std::error::Error for KdfError {}

/// Securely containerized derived key that homes memory wiping on drop.
pub struct DerivedKey {
    bytes: [u8; KEY_LEN],
}

impl DerivedKey {
    pub fn as_bytes(&self) -> &[u8; KEY_LEN] {
        &self.bytes
    }
}

impl Drop for DerivedKey {
    fn drop(&mut self) {
        self.bytes.zeroize();
    }
}

/// Generates a cryptographically secure 32-byte random salt.
pub fn generate_salt() -> [u8; SALT_LEN] {
    let mut salt = [0u8; SALT_LEN];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

/// Derives a 256-bit key using Argon2id (t=3, m=65536 KB, p=4).
pub fn derive_key(password: &[u8], salt: &[u8; SALT_LEN]) -> Result<DerivedKey, KdfError> {
    let params = Params::new(65536, 3, 4, Some(KEY_LEN))
        .map_err(|_| KdfError::DerivationFailed)?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut key_bytes = [0u8; KEY_LEN];
    argon2
        .hash_password_into(password, salt, &mut key_bytes)
        .map_err(|_| KdfError::DerivationFailed)?;

    Ok(DerivedKey { bytes: key_bytes })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kdf_consistency() {
        let password = b"SecureMasterPassword123!";
        let salt = generate_salt();

        let key1 = derive_key(password, &salt).unwrap();
        let key2 = derive_key(password, &salt).unwrap();

        assert_eq!(key1.as_bytes(), key2.as_bytes());
    }

    #[test]
    fn test_kdf_different_passwords() {
        let salt = generate_salt();
        let key1 = derive_key(b"PasswordA", &salt).unwrap();
        let key2 = derive_key(b"PasswordB", &salt).unwrap();

        assert_ne!(key1.as_bytes(), key2.as_bytes());
    }
}