mod crypto;
mod vault;
mod window;

use crate::vault::*;
use crate::window::*;

use std::io;
use std::sync::{Arc, Mutex};
use std::{fs, path::Path};

static FILENAME: &str = "vault.bin";

fn main() -> () {
    let mut vault: Vault = Vault::new();
    let mut salt: [u8; 16] = [0u8; 16];

    let vault_password;
    if Path::new(FILENAME).exists() {
        vault_password = get_input("Enter our vault password c:");
        let table = fs::read(FILENAME).unwrap();
        salt = table[..16].try_into().unwrap();
        let nonce: [u8; 12] = table[16..28].try_into().unwrap();
        let data_crypt = &table[28..];
        let data = crypto::decrypt_data(&vault_password.as_str(), data_crypt.to_vec(), nonce, salt);
        vault = bincode::deserialize(&data).unwrap();
    } else {
        vault_password = get_input("Enter our new vault password c:");
        crypto::generate_salt(&mut salt);
        vault.set_master(get_input("Enter new our master password c:"));
    }

    let shared_vault = Arc::new(Mutex::new(vault));

    let app: Window = Window::new(shared_vault.clone(), String::new());

    let option = eframe::NativeOptions::default();
    let _ = eframe::run_native("Password Manager", option, Box::new(|_| Box::new(app)));

    let vault_out = shared_vault.lock().unwrap();

    let data_serialized = bincode::serialize(&*vault_out).unwrap();
    let (data_encrypt, nonce_2) =
        crypto::encrypt_data(&vault_password.as_str(), &data_serialized, salt);
    let mut ouput = Vec::new();

    ouput.extend_from_slice(&salt);
    ouput.extend_from_slice(&nonce_2);
    ouput.extend_from_slice(&data_encrypt);

    fs::write(FILENAME, ouput).unwrap();
}

pub fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    input.trim().to_string()
}
