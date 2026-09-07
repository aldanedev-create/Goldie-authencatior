use crate::crypto::{cipher, dpapi, kdf};
use crate::models::account::{Account, AccountDisplay, EncryptedVault, HashAlgorithm, OtpType};
use data_encoding::BASE32_NOPAD;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager, State};

pub struct VaultState {
    pub key: Mutex<Option<[u8; 32]>>,
    pub salt: Mutex<Option<[u8; kdf::SALT_LEN]>>,
    pub accounts: Mutex<Vec<Account>>,
}

impl Default for VaultState {
    fn default() -> Self {
        Self {
            key: Mutex::new(None),
            salt: Mutex::new(None),
            accounts: Mutex::new(Vec::new()),
        }
    }
}

// Helper: Normalize and decode Base32 input (handles missing padding, spaces, lowercase)
fn decode_base32(secret: &str) -> Result<Vec<u8>, String> {
    let clean_secret = secret
        .trim()
        .replace(' ', "")
        .to_uppercase();

    // Standard RFC 4648 Base32 alphabet without padding
    let nopad_clean = clean_secret.trim_end_matches('=');
    
    BASE32_NOPAD
        .decode(nopad_clean.as_bytes())
        .map_err(|_| "Invalid Base32 secret string".to_string())
}

// Dynamic Truncation according to RFC 4226 / RFC 6238
fn generate_otp_code(
    secret_bytes: &[u8],
    counter: u64,
    digits: u32,
    algo: &HashAlgorithm,
) -> Result<String, String> {
    let counter_bytes = counter.to_be_bytes();

    let hash_bytes = match algo {
        HashAlgorithm::SHA1 => {
            let mut mac = Hmac::<Sha1>::new_from_slice(secret_bytes)
                .map_err(|_| "HMAC initialization failed")?;
            mac.update(&counter_bytes);
            mac.finalize().into_bytes().to_vec()
        }
        HashAlgorithm::SHA256 => {
            let mut mac = Hmac::<Sha256>::new_from_slice(secret_bytes)
                .map_err(|_| "HMAC initialization failed")?;
            mac.update(&counter_bytes);
            mac.finalize().into_bytes().to_vec()
        }
        HashAlgorithm::SHA512 => {
            let mut mac = Hmac::<Sha512>::new_from_slice(secret_bytes)
                .map_err(|_| "HMAC initialization failed")?;
            mac.update(&counter_bytes);
            mac.finalize().into_bytes().to_vec()
        }
    };

    let offset = (hash_bytes.last().unwrap() & 0x0F) as usize;
    let binary = ((hash_bytes[offset] & 0x7F) as u32) << 24
        | (hash_bytes[offset + 1] as u32) << 16
        | (hash_bytes[offset + 2] as u32) << 8
        | (hash_bytes[offset + 3] as u32);

    let modulus = 10u32.pow(digits);
    let code = binary % modulus;

    Ok(format!("{:0width$}", code, width = digits as usize))
}

fn get_vault_path(app: &AppHandle) -> Result<PathBuf, String> {
    let mut path = app
        .path()
        .app_data_dir()
        .map_err(|_| "Failed to resolve app data directory")?;
    fs::create_dir_all(&path).map_err(|_| "Failed to create app directory")?;
    path.push("vault.enc");
    Ok(path)
}

#[tauri::command]
pub async fn check_vault_exists(app: AppHandle) -> Result<bool, String> {
    let path = get_vault_path(&app)?;
    Ok(path.exists())
}

#[tauri::command]
pub async fn initialize_vault(
    password: String,
    state: State<'_, VaultState>,
    app: AppHandle,
) -> Result<(), String> {
    let path = get_vault_path(&app)?;
    if path.exists() {
        return Err("Vault already exists".to_string());
    }

    let salt = kdf::generate_salt();
    let derived = kdf::derive_key(password.as_bytes(), &salt).map_err(|e| e.to_string())?;

    let empty_accounts: Vec<Account> = Vec::new();
    let plaintext = serde_json::to_vec(&empty_accounts).map_err(|_| "Serialization error")?;

    let (ciphertext, nonce) =
        cipher::encrypt(derived.as_bytes(), &plaintext).map_err(|e| e.to_string())?;

    let dpapi_protected_salt = dpapi::protect_bytes(&salt).unwrap_or_else(|_| salt.to_vec());

    let encrypted_vault = EncryptedVault {
        version: 1,
        salt: data_encoding::BASE64.encode(&dpapi_protected_salt),
        nonce: data_encoding::BASE64.encode(&nonce),
        ciphertext: data_encoding::BASE64.encode(&ciphertext),
    };

    let json_data = serde_json::to_string_pretty(&encrypted_vault)
        .map_err(|_| "Failed to serialize vault file")?;
    fs::write(path, json_data).map_err(|_| "Failed to write vault file")?;

    *state.key.lock().unwrap() = Some(*derived.as_bytes());
    *state.salt.lock().unwrap() = Some(salt);
    *state.accounts.lock().unwrap() = empty_accounts;

    Ok(())
}

#[tauri::command]
pub async fn unlock_vault(
    password: String,
    state: State<'_, VaultState>,
    app: AppHandle,
) -> Result<bool, String> {
    let path = get_vault_path(&app)?;
    if !path.exists() {
        return Err("Vault does not exist".to_string());
    }

    let file_content = fs::read_to_string(path).map_err(|_| "Failed to read vault file")?;
    let vault: EncryptedVault =
        serde_json::from_str(&file_content).map_err(|_| "Invalid vault format")?;

    let raw_salt = data_encoding::BASE64
        .decode(vault.salt.as_bytes())
        .map_err(|_| "Invalid salt encoding")?;
    
    let salt_bytes = dpapi::unprotect_bytes(&raw_salt).unwrap_or(raw_salt);
    if salt_bytes.len() != kdf::SALT_LEN {
        return Err("Corrupted salt in vault".to_string());
    }

    let mut salt_arr = [0u8; kdf::SALT_LEN];
    salt_arr.copy_from_slice(&salt_bytes);

    let derived = kdf::derive_key(password.as_bytes(), &salt_arr).map_err(|e| e.to_string())?;

    let nonce_bytes = data_encoding::BASE64
        .decode(vault.nonce.as_bytes())
        .map_err(|_| "Invalid nonce")?;
    let ciphertext_bytes = data_encoding::BASE64
        .decode(vault.ciphertext.as_bytes())
        .map_err(|_| "Invalid ciphertext")?;

    if nonce_bytes.len() != cipher::NONCE_LEN {
        return Err("Corrupted nonce".to_string());
    }

    let mut nonce_arr = [0u8; cipher::NONCE_LEN];
    nonce_arr.copy_from_slice(&nonce_bytes);

    let decrypted = match cipher::decrypt(derived.as_bytes(), &ciphertext_bytes, &nonce_arr) {
        Ok(data) => data,
        Err(_) => return Ok(false), // Incorrect password
    };

    let accounts: Vec<Account> =
        serde_json::from_slice(&decrypted).map_err(|_| "Corrupted account payload")?;

    *state.key.lock().unwrap() = Some(*derived.as_bytes());
    *state.salt.lock().unwrap() = Some(salt_arr);
    *state.accounts.lock().unwrap() = accounts;

    Ok(true)
}

#[tauri::command]
pub async fn lock_vault(state: State<'_, VaultState>) -> Result<(), String> {
    *state.key.lock().unwrap() = None;
    *state.salt.lock().unwrap() = None;
    state.accounts.lock().unwrap().clear();
    Ok(())
}

#[tauri::command]
pub async fn get_account_codes(
    state: State<'_, VaultState>,
) -> Result<Vec<AccountDisplay>, String> {
    let key_guard = state.key.lock().unwrap();
    if key_guard.is_none() {
        return Err("Vault is locked".to_string());
    }

    let accounts = state.accounts.lock().unwrap();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut displays = Vec::new();

    for acc in accounts.iter() {
        let secret_bytes = decode_base32(&acc.secret)?;

        let (counter, remaining) = match acc.otp_type {
            OtpType::Totp => {
                let period = if acc.period == 0 { 30 } else { acc.period };
                let current_counter = now / (period as u64);
                let rem = period - ((now % (period as u64)) as u32);
                (current_counter, rem)
            }
            OtpType::Hotp => (acc.counter, 0),
        };

        let code = generate_otp_code(&secret_bytes, counter, acc.digits, &acc.algorithm)?;

        displays.push(AccountDisplay {
            id: acc.id.clone(),
            issuer: acc.issuer.clone(),
            account_name: acc.account_name.clone(),
            otp_type: acc.otp_type.clone(),
            digits: acc.digits,
            period: acc.period,
            counter: acc.counter,
            current_code: code,
            seconds_remaining: remaining,
        });
    }

    Ok(displays)
}

#[tauri::command]
pub async fn add_account(
    account: Account,
    state: State<'_, VaultState>,
    app: AppHandle,
) -> Result<(), String> {
    let key_guard = state.key.lock().unwrap();
    let key = match *key_guard {
        Some(k) => k,
        None => return Err("Vault is locked".to_string()),
    };

    // Validate secret is valid Base32
    decode_base32(&account.secret)?;

    let salt = state
        .salt
        .lock()
        .unwrap()
        .ok_or_else(|| "Vault salt missing from session".to_string())?;

    let mut accounts = state.accounts.lock().unwrap();
    accounts.push(account);

    persist_vault(&key, &salt, &accounts, &app)?;
    Ok(())
}

#[tauri::command]
pub async fn delete_account(
    id: String,
    state: State<'_, VaultState>,
    app: AppHandle,
) -> Result<(), String> {
    let key_guard = state.key.lock().unwrap();
    let key = match *key_guard {
        Some(k) => k,
        None => return Err("Vault is locked".to_string()),
    };

    let salt = state
        .salt
        .lock()
        .unwrap()
        .ok_or_else(|| "Vault salt missing from session".to_string())?;

    let mut accounts = state.accounts.lock().unwrap();
    accounts.retain(|a| a.id != id);

    persist_vault(&key, &salt, &accounts, &app)?;
    Ok(())
}

fn persist_vault(
    key: &[u8; 32],
    salt: &[u8; kdf::SALT_LEN],
    accounts: &[Account],
    app: &AppHandle,
) -> Result<(), String> {
    let path = get_vault_path(app)?;
    let plaintext = serde_json::to_vec(accounts).map_err(|_| "Serialization error")?;

    // Reuse the SAME salt that derived `key` — regenerating it here would silently
    // orphan the vault, since the key in memory would no longer match the salt
    // written to disk on the next unlock.
    let (ciphertext, nonce) = cipher::encrypt(key, &plaintext).map_err(|e| e.to_string())?;
    let dpapi_protected_salt = dpapi::protect_bytes(salt).unwrap_or_else(|_| salt.to_vec());

    let encrypted_vault = EncryptedVault {
        version: 1,
        salt: data_encoding::BASE64.encode(&dpapi_protected_salt),
        nonce: data_encoding::BASE64.encode(&nonce),
        ciphertext: data_encoding::BASE64.encode(&ciphertext),
    };

    let json_data = serde_json::to_string_pretty(&encrypted_vault)
        .map_err(|_| "Failed to serialize vault file")?;
    fs::write(path, json_data).map_err(|_| "Failed to write vault file")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rfc6238_totp_vectors() {
        // RFC 6238 Appendix A Test Vector secret (12345678901234567890 in ASCII)
        // Base32 for ASCII "12345678901234567890" -> GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ
        let secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";
        let secret_bytes = decode_base32(secret).unwrap();

        // Time T = 59s -> Counter = 59 / 30 = 1
        let code1 = generate_otp_code(&secret_bytes, 1, 8, &HashAlgorithm::SHA1).unwrap();
        assert_eq!(code1, "94287082");

        // Time T = 1111111109s -> Counter = 37037036
        let code2 = generate_otp_code(&secret_bytes, 37037036, 8, &HashAlgorithm::SHA1).unwrap();
        assert_eq!(code2, "07081804");
    }

    #[test]
    fn test_rfc4226_hotp_vectors() {
        // Secret "12345678901234567890"
        let secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";
        let secret_bytes = decode_base32(secret).unwrap();

        let expected_codes = [
            "755224", "287082", "359152", "969429", "338314",
            "254676", "287922", "162583", "399871", "520489"
        ];

        for (i, expected) in expected_codes.iter().enumerate() {
            let code = generate_otp_code(&secret_bytes, i as u64, 6, &HashAlgorithm::SHA1).unwrap();
            assert_eq!(code, *expected);
        }
    }
}