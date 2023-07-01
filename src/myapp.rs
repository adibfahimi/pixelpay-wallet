use crate::config::{generate_config, load_config, save_config, Config};
use crate::tx::Tx;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use ureq::{get, json, post};

pub struct MyApp {
    pub wallet: u32,
    pub address: String,
    pub target_address: String,
    pub target_amount: u32,
    pub private_key: String,
    pub message: String,
    pub node_uri: String,
}

impl MyApp {
    pub fn get_balance(&mut self) {
        let resp = get(&format!("{}/balance/{}", self.node_uri, self.address))
            .call()
            .unwrap()
            .into_string()
            .unwrap();
        let json = serde_json::from_str::<serde_json::Value>(&resp).unwrap();
        self.wallet = json["balance"].as_i64().unwrap() as u32;
        self.save();
    }

    pub fn copy_address(&self) {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(self.address.clone()).unwrap();
    }

    pub fn send(&mut self) {
        self.get_balance();

        if self.target_address == *"" {
            self.message = "Please enter a target address".to_string();
            return;
        }

        if self.target_amount == 0 {
            self.message = "Please enter an amount".to_string();
            return;
        }

        if self.wallet < self.target_amount {
            self.message = "Not enough balance".to_string();
            return;
        }

        let timestamp = chrono::Utc::now().timestamp();
        let mut new_tx = Tx {
            sender: self.address.clone(),
            receiver: self.target_address.clone(),
            amount: self.target_amount,
            signature: "".to_string(),
            hash: "".to_string(),
            timestamp: timestamp as u64,
        };
        new_tx.sign(self.private_key.clone());

        let resp = post(&format!("{}/tx", self.node_uri))
            .set("Content-Type", "application/json")
            .send_json(json!(&new_tx));

        match resp {
            Ok(resp) => {
                if resp.status() == 200 {
                    self.wallet -= self.target_amount;

                    self.save();
                    self.message =
                        "Transaction sent, please wait 12 min for confirmation".to_string();
                    self.target_amount = 0;
                    self.target_address = String::from("");
                } else {
                    self.message = "Transaction failed".to_string();
                }
            }
            Err(_) => {
                self.message = "Transaction failed".to_string();
            }
        }
    }

    fn save(&self) {
        let config = Config {
            wallet: self.wallet,
            address: self.address.clone(),
            private_key: self.private_key.clone(),
            node_uri: self.node_uri.clone(),
        };
        save_config(&config);
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let config = load_config().unwrap_or_else(generate_config);
        MyApp {
            wallet: config.wallet,
            address: config.address,
            target_address: String::new(),
            target_amount: 0,
            private_key: config.private_key,
            message: String::from(""),
            node_uri: config.node_uri,
        }
    }
}
