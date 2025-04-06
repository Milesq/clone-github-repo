mod keyring;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use std::{collections::HashMap, fs, io, path::Path};

#[derive(Default)]
pub struct AppData {
    config_file: String,
    pub data: HashMap<String, String>,
}

impl AppData {
    pub fn new() -> Option<Self> {
        let default_path = dirs::config_dir()?.join("clone.bin");
        Some(Self::with_path(default_path.to_str()?.to_string()))
    }

    pub fn with_path(path: impl ToString) -> Self {
        let config_file = path.to_string();

        Self {
            config_file,
            ..Default::default()
        }
        .read()
    }

    pub fn read(mut self) -> Self {
        self.data = if Path::new(&self.config_file).exists() {
            let data = fs::read(&self.config_file).unwrap();

            let deseralized: Result<HashMap<String, String>, _> =
                bincode::deserialize(data.as_slice());

            deseralized.map(Some).unwrap_or(None)
        } else {
            None
        }
        .unwrap_or_default();

        self
    }

    pub fn save(&self) -> io::Result<()> {
        let data = bincode::serialize(&self.data).expect("Cannot open file");

        let cred = keyring::generate_app_secret_key().unwrap();
        let cipher = Aes256Gcm::new(&cred.key);

        let ciphertext = cipher.encrypt(&cred.nonce, data.as_slice()).expect("encryption error");

        fs::write(self.config_file.as_str(), ciphertext)
    }

    pub fn set(&mut self, key: &str, val: &str) {
        self.data.insert(key.to_string(), val.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(&key.to_string()).map(|val| val.as_str())
    }
}
