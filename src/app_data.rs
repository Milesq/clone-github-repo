use std::{collections::HashMap, fs::File, io, path::Path};

#[derive(Default)]
pub struct AppData {
    config_file: String,
    pub data: HashMap<String, String>,
}

impl AppData {
    pub fn new() -> Option<Self> {
        let default_path = dirs::home_dir()?.join("./clone-cfg.bin");
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
            match File::open(&self.config_file) {
                Ok(f) => {
                    let deseralized: Result<HashMap<String, String>, _> =
                        bincode::deserialize_from(f);

                    deseralized.map(Some).unwrap_or(None)
                }
                Err(_) => None,
            }
        } else {
            None
        }
        .unwrap_or_default();

        self
    }

    pub fn save(self) -> io::Result<()> {
        let f = open_or_create(self.config_file.as_str())?;
        bincode::serialize_into(f, &self.data).expect("Cannot open file");

        Ok(())
    }

    pub fn set(&mut self, key: &str, val: &str) {
        self.data.insert(key.to_string(), val.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(&key.to_string()).map(|val| val.as_str())
    }
}

fn open_or_create(path: &str) -> io::Result<File> {
    if Path::new(path).exists() {
        File::open(path)
    } else {
        File::create(path)
    }
}
