use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub storage_location: StorageLocation,
    pub storage_type: StorageType,
    pub local_storage_dir: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            storage_location: StorageLocation::Local,
            storage_type: StorageType::Json,
            local_storage_dir: Some("./data".to_string()),
        }
    }
}

#[derive(Deserialize)]
pub enum StorageLocation {
    Local,
    Remote,
}

#[derive(Deserialize)]
pub enum StorageType {
    Json,
    SQL,
}

impl Config {
    pub fn load() -> Self {
        fs::read_to_string("./config/config.json")
            .map(|s| serde_json::from_str(&s).expect("Invalid config file"))
            .unwrap_or_default()
    }
}
