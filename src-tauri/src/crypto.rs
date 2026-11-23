use aes_gcm::{Aes256Gcm, Key, Nonce, aead::{Aead, KeyInit, OsRng}};
use rand::RngCore;

/// Generates a random 256-bit (32-byte) key for AES-256-GCM.
pub fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

/// Encrypts plaintext using AES-256-GCM with a random 12-byte nonce.
/// Returns nonce || ciphertext (12-byte nonce prepended).
pub fn encrypt(plaintext: &[u8], key: &[u8; 32]) -> anyhow::Result<Vec<u8>> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

    let mut result = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

/// Decrypts ciphertext where the first 12 bytes are the nonce and the remainder is the encrypted data.
/// Returns the decrypted plaintext.
pub fn decrypt(ciphertext: &[u8], key: &[u8; 32]) -> anyhow::Result<Vec<u8>> {
    if ciphertext.len() < 12 {
        anyhow::bail!("Ciphertext too short: must be at least 12 bytes for nonce");
    }

    let (nonce_bytes, encrypted_data) = ciphertext.split_at(12);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, encrypted_data)
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_key() {
        let key1 = generate_key();
        let key2 = generate_key();
        assert_eq!(key1.len(), 32);
        assert_eq!(key2.len(), 32);
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let key = generate_key();
        let plaintext = b"Hello, AES-256-GCM encryption!";
        let ciphertext = encrypt(plaintext, &key).expect("encryption should succeed");

        // Nonce (12) + Plaintext + Tag (16)
        assert_eq!(ciphertext.len(), 12 + plaintext.len() + 16);

        let decrypted = decrypt(&ciphertext, &key).expect("decryption should succeed");
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_decrypt_empty() {
        let key = generate_key();
        let plaintext = b"";
        let ciphertext = encrypt(plaintext, &key).expect("encryption should succeed");
        assert_eq!(ciphertext.len(), 12 + 16);

        let decrypted = decrypt(&ciphertext, &key).expect("decryption should succeed");
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_unique_nonces() {
        let key = generate_key();
        let plaintext = b"constant message";
        let c1 = encrypt(plaintext, &key).unwrap();
        let c2 = encrypt(plaintext, &key).unwrap();
        assert_ne!(c1, c2);
        assert_ne!(&c1[..12], &c2[..12]);
    }

    #[test]
    fn test_decrypt_invalid_key() {
        let key1 = generate_key();
        let key2 = generate_key();
        let plaintext = b"confidential payload";
        let ciphertext = encrypt(plaintext, &key1).unwrap();

        let result = decrypt(&ciphertext, &key2);
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_tampered_ciphertext() {
        let key = generate_key();
        let plaintext = b"tamper test payload";
        let mut ciphertext = encrypt(plaintext, &key).unwrap();
        let last_idx = ciphertext.len() - 1;
        ciphertext[last_idx] ^= 0x01;

        let result = decrypt(&ciphertext, &key);
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_too_short() {
        let key = generate_key();
        let short_data = vec![0u8; 11];
        let result = decrypt(&short_data, &key);
        assert!(result.is_err());
    }
}
