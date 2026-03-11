use crate::crypto;
use argon2::{
    Argon2, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString},
};
use chacha20poly1305::aead::OsRng;
use pyo3::{pyclass, pymethods, pymodule};
use pyo3::prelude::*;
use std::{fs, path::Path};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct Vault {
    salt_key: [u8; 16],
    pub(crate) entry: HashMap<String, Entry>,
    hash: String,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct Data {
    user: String,
    password: String,
    location: String,
}

#[pymethods]
impl Data {
    #[new]
    pub fn new(user: &str, password: &str, location: &str) -> Self {
        Self {
            user: user.to_string(),
            password: password.to_string(),
            location: location.to_string(),
        }
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
    nonce_b: [u8; 12],
}

#[pymethods]
impl Vault {
    #[new]
    pub fn new() -> Self {
        Self {
            salt_key: [0u8; 16],
            entry: HashMap::new(),
            hash: String::new(),
        }
    }

    pub fn is_a_master(&self) -> bool {
        return self.hash.eq(&String::new());
    }

    pub fn set_master(&mut self, master: String) -> () {
        crypto::generate_salt(&mut self.salt_key);
        let mut rng = OsRng;
        let salt = SaltString::generate(&mut rng);
        let argon2 = Argon2::default();
        self.hash = argon2
            .hash_password(master.as_bytes(), &salt)
            .unwrap()
            .to_string();
    }

    pub fn verify_master(&self, master: String) -> bool {
        let argon2 = Argon2::default();
        let parsed_hash = argon2::PasswordHash::new(&self.hash).unwrap();
        return argon2
            .verify_password(master.as_bytes(), &parsed_hash)
            .is_ok();
    }

    pub fn lock(&mut self, master: &str, data: Data, name: String) -> () {
        let json = serde_json::to_string(&data).unwrap();
        let (encrypt, nonce_b) = crypto::encrypt_data(master, &json.as_bytes(), self.salt_key);
        self.entry.insert(
            name,
            Entry {
                nonce_b: nonce_b,
                locked_data: encrypt,
            },
        );
    }

    pub fn unlock(&self, master: &str, which: String) -> Data {
        let entry = match self.entry.get(&which) {
            Some(e) => e,
            None => panic!("Entry not found"),
        };
        let decrypt = crypto::decrypt_data(
            master,
            entry.locked_data.clone(),
            entry.nonce_b,
            self.salt_key,
        );
        let str = String::from_utf8(decrypt).unwrap();
        return serde_json::from_str(&str).unwrap();
    }

    pub fn get_name_password(&self) -> Vec<String> {
        return self.entry.keys().cloned().collect();
    }

    pub fn generate_salt(&self) -> [u8; 16] {
        let mut salt: [u8; 16] = [0u8; 16];
        crypto::generate_salt(&mut salt);
        return salt;
    }

    pub fn generate_random_password(&self) -> String {
        return "".to_string();
    }
}


static FILENAME: &str = "vault.bin";

#[pyfunction]
pub fn load_vault(path: String, vault_password: String) -> PyResult<(Vault, [u8; 16])> {
    let mut vault: Vault = Vault::new();
    let mut salt: [u8; 16] = [0u8; 16];

    let vault_path = Path::new(&path).join(FILENAME);

   
    if vault_path.exists() {
        let table = fs::read(vault_path).unwrap();
        salt = table[..16].try_into().unwrap();
        let nonce: [u8; 12] = table[16..28].try_into().unwrap();
        let data_crypt = &table[28..];

        let data = crypto::decrypt_data(&vault_password.as_str(), data_crypt.to_vec(), nonce, salt);
        vault = bincode::deserialize(&data).unwrap();
    }

    return Ok((vault, salt));
}

#[pyfunction]
pub fn save_vault(path: String, vault: &Vault, vault_password: String, salt: [u8; 16]) {
    
    let vault_path = Path::new(&path).join(FILENAME);

    let data_serialized = bincode::serialize(&vault).unwrap();
    let (data_encrypt, nonce_2) =
        crypto::encrypt_data(&vault_password.as_str(), &data_serialized, salt);
    let mut ouput = Vec::new();

    ouput.extend_from_slice(&salt);
    ouput.extend_from_slice(&nonce_2);
    ouput.extend_from_slice(&data_encrypt);

    fs::write(vault_path, ouput).unwrap();
}