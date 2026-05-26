use clap;

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes128Gcm, Nonce,
    aead::rand_core::RngCore,
};

#[derive(Debug)]
#[derive(clap::ValueEnum, Clone)]
pub enum Algo {
    Xor,
    Aes,
}

pub enum Key {
    Xor(u8),
    Aes([u8; 16]),
}

pub fn parse_hex_bytes(s: &str) -> Result<[u8; 16], String> {
    let cleaned = s.trim_start_matches("0x").trim_start_matches("0X");
    if cleaned.len() != 32 {
        return Err(format!("AES key must be 32 hex chars, got {}", cleaned.len()));
    }
    let mut key = [0u8; 16];
    for (i, chunk) in cleaned.as_bytes().chunks(2).enumerate() {
        let byte_str = std::str::from_utf8(chunk).map_err(|e| e.to_string())?;
        key[i] = u8::from_str_radix(byte_str, 16)
            .map_err(|_| format!("Invalid hex byte: {}", byte_str))?;
    }
    Ok(key)
}

pub fn xor_crypt(code: &mut Vec<u8>, key: u8){
        for byte in code.iter_mut() {
            *byte ^= key;
        }
    }

pub fn parse_hex(s: &str) -> Result<u8, String> {
    let cleaned = s.trim_start_matches("0x").trim_start_matches("0X");
    u8::from_str_radix(cleaned, 16)
        .map_err(|_| format!("Invalid hex value: {}", s))
}

pub trait Cipher {
    fn encrypt(&self, data: &mut Vec<u8>);
    fn decrypt(&self, data: &mut Vec<u8>);
}

pub struct Xor { key: u8 }
pub struct Aes { key: [u8; 16] }

impl Cipher for Xor {
    fn encrypt(&self, data: &mut Vec<u8>) { 
        xor_crypt(data, self.key);
    }
    fn decrypt(&self, data: &mut Vec<u8>) {
        xor_crypt(data, self.key);
    }
}

impl Cipher for Aes {

    fn encrypt(&self, data: &mut Vec<u8>) {
        let cipher = Aes128Gcm::new_from_slice(&self.key).unwrap();
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher.encrypt(nonce, data.as_ref()).unwrap();
        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);
        *data = result;
    }

    fn decrypt(&self, data: &mut Vec<u8>) {
        let cipher = Aes128Gcm::new_from_slice(&self.key).unwrap();
        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        let plaintext = cipher.decrypt(nonce, ciphertext).unwrap();
        *data = plaintext;
    }

}

pub fn create_cipher(key: &Key) -> Box<dyn Cipher> {
    match key {
        Key::Xor(k) => Box::new(Xor { key: *k }),
        Key::Aes(k) => Box::new(Aes { key: *k }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_crypt_restores_original_data() {
        let original = vec![0x41, 0x42, 0x43];
        let mut data = original.clone();

        xor_crypt(&mut data, 0xAA);
        xor_crypt(&mut data, 0xAA);

        assert_eq!(data, original);
    }

    #[test]
    fn test_parse_hex_returns_correct_byte() {
        assert_eq!(0xAA, parse_hex("AA").unwrap());
    }

    #[test]
    fn test_parse_hex_returns_error_on_invalid_input() {
        assert!(parse_hex("ZZ").is_err());
    }
}
