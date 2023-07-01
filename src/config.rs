use secp256k1::rand::rngs::OsRng;
use secp256k1::Secp256k1;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub wallet: i32,
    pub address: String,
    pub private_key: String,
}

pub fn load_config() -> Option<Config> {
    if let Ok(config_file) = std::fs::read_to_string("config.json") {
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
    };

    if let Ok(config_json) = serde_json::to_string_pretty(&config) {
        std::fs::write("config.json", config_json).unwrap();
    }

    config
}

pub fn save_config(config: &Config) {
    if let Ok(config_json) = serde_json::to_string_pretty(&config) {
        std::fs::write("config.json", config_json).unwrap();
    }
}
