use hmac::{Hmac, Mac};
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum HashAlgorithm {
    SHA1,
    SHA256,
    SHA512,
}

pub fn decode_base32(input: &str) -> Result<Vec<u8>, String> {
    let clean = input.replace([' ', '-'], "").to_uppercase();
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut bits = 0u32;
    let mut value = 0u32;
    let mut output = Vec::new();

    for c in clean.chars() {
        if c == '=' { break; }
        let idx = alphabet.find(c).ok_or_else(|| format!("Invalid Base32 char: {}", c))?;
        value = (value << 5) | (idx as u32);
        bits += 5;

        if bits >= 8 {
            output.push(((value >> (bits - 8)) & 0xFF) as u8);
            bits -= 8;
        }
    }

    Ok(output)
}

fn compute_hmac(key: &[u8], msg: &[u8], algo: &HashAlgorithm) -> Result<Vec<u8>, String> {
    match algo {
        HashAlgorithm::SHA1 => {
            let mut mac = Hmac::<Sha1>::new_from_slice(key).map_err(|e| e.to_string())?;
            mac.update(msg);
            Ok(mac.finalize().into_bytes().to_vec())
        }
        HashAlgorithm::SHA256 => {
            let mut mac = Hmac::<Sha256>::new_from_slice(key).map_err(|e| e.to_string())?;
            mac.update(msg);
            Ok(mac.finalize().into_bytes().to_vec())
        }
        HashAlgorithm::SHA512 => {
            let mut mac = Hmac::<Sha512>::new_from_slice(key).map_err(|e| e.to_string())?;
            mac.update(msg);
            Ok(mac.finalize().into_bytes().to_vec())
        }
    }
}

pub fn generate_otp(
    secret: &str,
    counter: u64,
    digits: u32,
    algo: &HashAlgorithm,
) -> Result<String, String> {
    let key = decode_base32(secret)?;
    let msg = counter.to_be_bytes();
    let hash = compute_hmac(&key, &msg, algo)?;

    let offset = (hash[hash.len() - 1] & 0x0F) as usize;
    let binary = ((hash[offset] & 0x7F) as u32) << 24
        | ((hash[offset + 1] & 0xFF) as u32) << 16
        | ((hash[offset + 2] & 0xFF) as u32) << 8
        | ((hash[offset + 3] & 0xFF) as u32);

    let otp = binary % 10u32.pow(digits);
    Ok(format!("{:0 width$}", otp, width = digits as usize))
}

pub fn generate_totp(
    secret: &str,
    time_step: u64,
    digits: u32,
    algo: &HashAlgorithm,
) -> Result<(String, u64), String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    let counter = now / time_step;
    let seconds_remaining = time_step - (now % time_step);
    let code = generate_otp(secret, counter, digits, algo)?;

    Ok((code, seconds_remaining))
}