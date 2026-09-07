use crate::crypto;
use crate::models::account::Account;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use zeroize::Zeroize;

#[derive(Serialize, Deserialize, Debug)]
pub struct VaultFile {
    pub salt: Vec<u8>,
    pub nonce: [u8; 12],
    pub encrypted_data: Vec<u8>,
    pub dpapi_protected: bool,
}

#[derive(Default)]
pub struct MasterKey {
    pub key: Option<[u8; 32]>,
}

impl Drop for MasterKey {
    fn drop(&mut self) {
        if let Some(ref mut k) = self.key {
            k.zeroize();
        }
    }
}

pub struct VaultState {
    pub master_key: Mutex<MasterKey>,
}

pub fn get_vault_path() -> Result<PathBuf, String> {
    let mut path = dirs::data_dir().ok_or("Failed to resolve app data dir")?;
    path.push("AegisVault");
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    path.push("vault.bin");
    Ok(path)
}

pub fn vault_exists() -> bool {
    get_vault_path().map(|p| p.exists()).unwrap_or(false)
}

pub fn save_vault(accounts: &[Account], master_key: &[u8; 32]) -> Result<(), String> {
    let path = get_vault_path()?;
    let serialized = serde_json::to_vec(accounts).map_err(|e| e.to_string())?;

    let mut salt = [0u8; 16];
    rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut salt);

    let (encrypted_data, nonce) = crypto::encrypt(&serialized, master_key)?;
    let dpapi_data = crypto::protect_key_dpapi(&encrypted_data)?;

    let vault_file = VaultFile {
        salt: salt.to_vec(),
        nonce,
        encrypted_data: dpapi_data,
        dpapi_protected: true,
    };

    let payload = serde_json::to_vec(&vault_file).map_err(|e| e.to_string())?;
    fs::write(path, payload).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_vault(master_key: &[u8; 32]) -> Result<Vec<Account>, String> {
    let path = get_vault_path()?;
    if !path.exists() {
        return Ok(Vec::new());
    }

    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let vault_file: VaultFile = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;

    let raw_ciphertext = if vault_file.dpapi_protected {
        crypto::unprotect_key_dpapi(&vault_file.encrypted_data)?
    } else {
        vault_file.encrypted_data
    };

    let decrypted = crypto::decrypt(&raw_ciphertext, &vault_file.nonce, master_key)?;
    let accounts: Vec<Account> = serde_json::from_slice(&decrypted).map_err(|e| e.to_string())?;
    Ok(accounts)
}