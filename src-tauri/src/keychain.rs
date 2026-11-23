use super::crypto;
use keyring::Entry;

pub const SERVICE_NAME: &str = "com.tuanldas.statusbar";
const ENCRYPTION_KEY_NAME: &str = "encryption_key";

/// Stores a secret value in the OS keychain.
pub fn store_secret(key: &str, value: &str) -> anyhow::Result<()> {
    let entry = Entry::new(SERVICE_NAME, key)?;
    entry.set_password(value)?;
    Ok(())
}

/// Retrieves a secret value from the OS keychain.
/// Returns None if the secret is not found (does not error on missing).
pub fn get_secret(key: &str) -> anyhow::Result<Option<String>> {
    let entry = Entry::new(SERVICE_NAME, key)?;
    match entry.get_password() {
        Ok(password) => Ok(Some(password)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(err) => Err(err.into()),
    }
}

/// Deletes a secret entry from the OS keychain.
pub fn delete_secret(key: &str) -> anyhow::Result<()> {
    let entry = Entry::new(SERVICE_NAME, key)?;
    entry.delete_credential()?;
    Ok(())
}

/// Stores or retrieves the 32-byte encryption key in the OS keychain.
/// - Checks if encryption key exists in keychain (key="encryption_key")
/// - If exists, decodes from hex and returns it
/// - If not, generates a new key via `crypto::generate_key()`, stores it as hex, and returns it
pub fn store_encryption_key() -> anyhow::Result<[u8; 32]> {
    if let Some(existing_hex) = get_secret(ENCRYPTION_KEY_NAME)? {
        return decode_hex_key(&existing_hex);
    }

    let key = crypto::generate_key();
    let hex_key = encode_hex_key(&key);
    store_secret(ENCRYPTION_KEY_NAME, &hex_key)?;
    Ok(key)
}

/// Retrieves the 32-byte encryption key from the OS keychain, decoding from hex.
/// If not found, calls `store_encryption_key()` to generate and store one.
pub fn get_encryption_key() -> anyhow::Result<[u8; 32]> {
    match get_secret(ENCRYPTION_KEY_NAME)? {
        Some(hex_key) => decode_hex_key(&hex_key),
        None => store_encryption_key(),
    }
}

/// Encodes a 32-byte array into a 64-character lowercase hex string.
fn encode_hex_key(key: &[u8; 32]) -> String {
    let mut hex = String::with_capacity(64);
    for byte in key {
        use std::fmt::Write;
        let _ = write!(hex, "{:02x}", byte);
    }
    hex
}

/// Decodes a 64-character hex string into a 32-byte array.
fn decode_hex_key(hex_str: &str) -> anyhow::Result<[u8; 32]> {
    let trimmed = hex_str.trim();
    if trimmed.len() != 64 {
        anyhow::bail!(
            "Invalid hex encryption key length: expected 64 hex characters, got {}",
            trimmed.len()
        );
    }

    let mut key = [0u8; 32];
    for (i, byte) in key.iter_mut().enumerate() {
        let chunk = &trimmed[i * 2..i * 2 + 2];
        *byte = u8::from_str_radix(chunk, 16)
            .map_err(|e| anyhow::anyhow!("Invalid hex character in encryption key: {}", e))?;
    }
    Ok(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_name_constant() {
        assert_eq!(SERVICE_NAME, "com.tuanldas.statusbar");
    }

    #[test]
    fn test_hex_encoding_decoding_roundtrip() {
        let original_key: [u8; 32] = [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
            0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
            0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
        ];

        let hex = encode_hex_key(&original_key);
        assert_eq!(hex.len(), 64);
        assert_eq!(
            hex,
            "0123456789abcdeffedcba987654321000112233445566778899aabbccddeeff"
        );

        let decoded = decode_hex_key(&hex).expect("Failed to decode valid hex key");
        assert_eq!(decoded, original_key);
    }

    #[test]
    fn test_decode_hex_key_invalid_length() {
        let short_hex = "0123456789abcdef";
        assert!(decode_hex_key(short_hex).is_err());

        let long_hex = "00".repeat(33);
        assert!(decode_hex_key(&long_hex).is_err());
    }

    #[test]
    fn test_decode_hex_key_invalid_chars() {
        let invalid_hex = "zz".repeat(32);
        assert!(decode_hex_key(&invalid_hex).is_err());
    }

    #[test]
    fn test_decode_hex_key_with_whitespace() {
        let key = [0x42u8; 32];
        let hex = format!("  {} \n", encode_hex_key(&key));
        let decoded = decode_hex_key(&hex).expect("Failed to decode trimmed hex key");
        assert_eq!(decoded, key);
    }

    #[test]
    fn test_decode_hex_key_uppercase() {
        let key = [0xa5u8; 32];
        let hex = "A5".repeat(32);
        let decoded = decode_hex_key(&hex).expect("Failed to decode uppercase hex key");
        assert_eq!(decoded, key);
    }
}
