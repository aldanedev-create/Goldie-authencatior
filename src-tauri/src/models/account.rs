use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OtpType {
    Totp,
    Hotp,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HashAlgorithm {
    SHA1,
    SHA256,
    SHA512,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub issuer: String,
    pub account_name: String,
    pub secret: String, // Base32 encoded string
    pub otp_type: OtpType,
    pub algorithm: HashAlgorithm,
    pub digits: u32,
    pub period: u32,   // For TOTP (default: 30)
    pub counter: u64,  // For HOTP
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedVault {
    pub version: u32,
    pub salt: String,       // Base64 encoded 32-byte salt
    pub nonce: String,      // Base64 encoded 12-byte nonce
    pub ciphertext: String, // Base64 encoded encrypted JSON array of Accounts
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDisplay {
    pub id: String,
    pub issuer: String,
    pub account_name: String,
    pub otp_type: OtpType,
    pub digits: u32,
    pub period: u32,
    pub counter: u64,
    pub current_code: String,
    pub seconds_remaining: u32,
}