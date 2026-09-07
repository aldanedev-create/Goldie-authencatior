#[cfg(target_os = "windows")]
use windows_sys::Win32::Foundation::HLOCAL;
#[cfg(target_os = "windows")]
use windows_sys::Win32::Security::Cryptography::{
    CryptProtectData, CryptUnprotectData, CRYPT_INTEGER_BLOB,
};
#[cfg(target_os = "windows")]
use zeroize::Zeroize;

#[derive(Debug)]
pub enum DpapiError {
    ProtectFailed,
    UnprotectFailed,
}

impl std::fmt::Display for DpapiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DpapiError::ProtectFailed => write!(f, "DPAPI protection failed"),
            DpapiError::UnprotectFailed => write!(f, "DPAPI unprotection failed"),
        }
    }
}

impl std::error::Error for DpapiError {}

/// Encrypts input bytes using Windows DPAPI bound to the current user context.
#[cfg(target_os = "windows")]
pub fn protect_bytes(data: &[u8]) -> Result<Vec<u8>, DpapiError> {
    let mut in_blob = CRYPT_INTEGER_BLOB {
        cbData: data.len() as u32,
        pbData: data.as_ptr() as *mut u8,
    };
    let mut out_blob = CRYPT_INTEGER_BLOB {
        cbData: 0,
        pbData: std::ptr::null_mut(),
    };

    let result = unsafe {
        CryptProtectData(
            &mut in_blob,
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null_mut(),
            std::ptr::null(),
            0,
            &mut out_blob,
        )
    };

    if result == 0 {
        return Err(DpapiError::ProtectFailed);
    }

    let protected_data = unsafe {
        std::slice::from_raw_parts(out_blob.pbData, out_blob.cbData as usize).to_vec()
    };

    unsafe {
        windows_sys::Win32::Foundation::LocalFree(out_blob.pbData as HLOCAL);
    }

    Ok(protected_data)
}

/// Decrypts DPAPI protected bytes using current user credentials.
#[cfg(target_os = "windows")]
pub fn unprotect_bytes(data: &[u8]) -> Result<Vec<u8>, DpapiError> {
    let mut in_blob = CRYPT_INTEGER_BLOB {
        cbData: data.len() as u32,
        pbData: data.as_ptr() as *mut u8,
    };
    let mut out_blob = CRYPT_INTEGER_BLOB {
        cbData: 0,
        pbData: std::ptr::null_mut(),
    };

    let result = unsafe {
        CryptUnprotectData(
            &mut in_blob,
            std::ptr::null_mut(),
            std::ptr::null(),
            std::ptr::null_mut(),
            std::ptr::null(),
            0,
            &mut out_blob,
        )
    };

    if result == 0 {
        return Err(DpapiError::UnprotectFailed);
    }

    let mut unprotected_data = unsafe {
        std::slice::from_raw_parts(out_blob.pbData, out_blob.cbData as usize).to_vec()
    };

    unsafe {
        windows_sys::Win32::Foundation::LocalFree(out_blob.pbData as HLOCAL);
    }

    let result_copy = unprotected_data.clone();
    unprotected_data.zeroize();
    Ok(result_copy)
}

/// Non-Windows dummy fallback for unit testing environments.
#[cfg(not(target_os = "windows"))]
pub fn protect_bytes(data: &[u8]) -> Result<Vec<u8>, DpapiError> {
    let mut obfuscated = data.to_vec();
    for byte in obfuscated.iter_mut() {
        *byte ^= 0xAA;
    }
    Ok(obfuscated)
}

#[cfg(not(target_os = "windows"))]
pub fn unprotect_bytes(data: &[u8]) -> Result<Vec<u8>, DpapiError> {
    let mut restored = data.to_vec();
    for byte in restored.iter_mut() {
        *byte ^= 0xAA;
    }
    Ok(restored)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dpapi_roundtrip() {
        let secret = b"WindowsUserSpecificSecret_12345";
        let protected = protect_bytes(secret).expect("Protect failed");
        let unprotected = unprotect_bytes(&protected).expect("Unprotect failed");

        assert_eq!(secret.to_vec(), unprotected);
    }
}