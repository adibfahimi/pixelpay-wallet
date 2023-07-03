use secp256k1::rand::rngs::OsRng;
use secp256k1::Secp256k1;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub wallet: u32,
    pub address: String,
    pub private_key: String,
    pub node_uri: String,
}

pub fn load_config() -> Option<Config> {
    let config_path = get_config_path()?;
    if let Ok(config_file) = std::fs::read_to_string(config_path) {
        if let Ok(config) = serde_json::from_str(&config_file) {
            return Some(config);
        }
    }
    None
}

pub fn generate_config() -> Config {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng::default());

    let config = Config {
        wallet: 0,
        address: hex::encode(public_key.serialize()),
        private_key: hex::encode(secret_key.secret_bytes()),
        node_uri: "http://localhost:8080".to_string(),
    };

    if let Ok(config_json) = serde_json::to_string_pretty(&config) {
        if let Some(config_path) = get_config_path() {
            std::fs::write(config_path, config_json).unwrap();
        }
    }

    config
}

pub fn save_config(config: &Config) {
    if let Ok(config_json) = serde_json::to_string_pretty(&config) {
        if let Some(config_path) = get_config_path() {
            std::fs::write(config_path, config_json).unwrap();
        }
    }
}

fn get_config_path() -> Option<PathBuf> {
    let mut config_dir = dirs::home_dir()?;
    config_dir.push(".config");
    config_dir.push("pixelpay");
    std::fs::create_dir_all(&config_dir).ok()?;
    config_dir.push("config.json");
    Some(config_dir)
}

#[cfg(target_os = "windows")]
fn get_config_path() -> Option<PathBuf> {
    let mut config_dir = dirs::config_dir()?;
    config_dir.push("pixelpay");
    std::fs::create_dir_all(&config_dir).ok()?;
    config_dir.push("config.json");
    Some(config_dir)
}

