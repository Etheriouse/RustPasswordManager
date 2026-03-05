use chacha20poly1305::aead::OsRng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use argon2::{Argon2, PasswordVerifier, password_hash::{PasswordHasher, SaltString}};
use crate::crypto;

#[derive(Serialize, Deserialize)]
pub struct Vault {
    salt_key: [u8; 16],
    pub (crate) entry: HashMap<String, Entry>,
    hash: String,
    salt_hash: [u8; 16]
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    user: String,
    password: String,
    location: String,
}

impl Data {
    pub fn new(user: &str, password: &str, location: &str) -> Self {
        Self {user: user.to_string(), password: password.to_string(), location: location.to_string() }
    }

    pub fn get_username(&self) -> String {
        return self.user.to_string();
    }

    pub fn get_password(&self) -> String {
        return self.password.to_string();
    }

    pub fn get_location(&self) -> String {
        return self.location.to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct Entry {
    locked_data: Vec<u8>,
    nonce_b: [u8; 12]
}

impl Vault {

    pub fn new() -> Self {
        Self { salt_key: [0u8; 16], entry: HashMap::new(), hash: String::new(), salt_hash: [0u8; 16] }
    }

    pub fn set_master(&mut self, master: String) -> () {
        crypto::generate_salt(&mut self.salt_key);
        let mut rng = OsRng;
        let salt = SaltString::generate(&mut rng);
        let argon2 = Argon2::default();
        self.hash = argon2.hash_password(master.as_bytes(), &salt).unwrap().to_string();
    }

    pub fn verify_master(&self, master: String) -> bool {
        let argon2 = Argon2::default();
        let parsed_hash = argon2::PasswordHash::new(&self.hash).unwrap();
        return argon2.verify_password(master.as_bytes(), &parsed_hash).is_ok()
    }

    pub fn lock(&mut self, master: &str, data: Data, name: String) -> () {
        let json = serde_json::to_string(&data).unwrap();
        let (encrypt,nonce_b) = crypto::encrypt_data(master, &json.as_bytes(), self.salt_key);
        self.entry.insert(name, Entry {nonce_b: nonce_b, locked_data: encrypt });
    }
    
    pub fn unlock(&self, master: &str, which: String) -> Data {
        let entry = &self.entry[&which]; 
        let decrypt = crypto::decrypt_data(master, entry.locked_data.clone(), entry.nonce_b, self.salt_key);
        let str = String::from_utf8(decrypt).unwrap();
        return serde_json::from_str(&str).unwrap();
    }
}