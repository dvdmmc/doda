use ring::{aead, rand::{self, SecureRandom}};
use std::error::Error;

pub struct CryptoManager {
    key: aead::LessSafeKey,
}

pub trait Encryption {
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>>;
    fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>>;
}

impl CryptoManager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let rng = rand::SystemRandom::new();
        let mut key_bytes = [0u8; 32];
        rng.fill(&mut key_bytes)
            .map_err(|_| "Failed to generate random bytes")?;
        
        let unbound_key = aead::UnboundKey::new(&aead::CHACHA20_POLY1305, &key_bytes)
            .map_err(|_| "Failed to create unbound key")?;
        let key = aead::LessSafeKey::new(unbound_key);
        
        Ok(Self { key })
    }
}

pub fn generate_nonce() -> aead::Nonce {
    let rng = rand::SystemRandom::new();
    let mut nonce_bytes = [0u8; 12];
    rng.fill(&mut nonce_bytes)
        .expect("Failed to generate nonce");
    aead::Nonce::assume_unique_for_key(nonce_bytes)
}
