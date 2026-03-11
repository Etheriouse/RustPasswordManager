use argon2::{
    Argon2,
    password_hash::{
        rand_core::{OsRng, RngCore},
    },
};
use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce, aead::{Aead, KeyInit}
};

use chacha20poly1305::aead::Error;

pub fn generate_salt(salt: &mut [u8; 16]) {
    OsRng.fill_bytes(salt);
}

pub fn encrypt_data(master: &str, json: &[u8], salt: [u8; 16]) -> Result<(Vec<u8>, [u8; 12]), Error>  {
    let mut key_b = [0u8; 32];
    Argon2::default().hash_password_into(master.as_bytes(), &salt, &mut key_b).unwrap();
    let key = Key::from_slice(&key_b);
    let cipher = ChaCha20Poly1305::new(key);
    let mut nonce_b = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_b);
    let nonce = Nonce::from_slice(&nonce_b);
    let encrypted = cipher.encrypt(nonce, json)?;
    return Ok((encrypted, nonce_b))
}

pub fn decrypt_data(master: &str, locked_data: Vec<u8>, nonce_b: [u8; 12], salt: [u8; 16]) -> Result<Vec<u8>, Error> {
    let mut key_b = [0u8; 32];
    Argon2::default().hash_password_into(master.as_bytes(), &salt, &mut key_b).unwrap();
    let key = Key::from_slice(&key_b);
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = Nonce::from_slice(&nonce_b);
    return cipher.decrypt(nonce, locked_data.as_ref());
}
