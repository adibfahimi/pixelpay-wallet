#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod config;
mod tx;
use config::save_config;
use eframe::egui;
use ureq::{get, json, post};
const NODE_URI: &str = "http://localhost:8080";
extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use config::{generate_config, load_config};

struct MyApp {
    wallet: u32,
    address: String,
    target_address: String,
    target_amount: u32,
    private_key: String,
    message: String,
}

impl MyApp {
    pub fn get_balance(&mut self) {
        let resp = get(&format!("{}/balance/{}", NODE_URI, self.address))
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

        let timestamp = chrono::Utc::now().timestamp();
        let mut new_tx = tx::Tx {
            sender: self.address.clone(),
            receiver: self.target_address.clone(),
            amount: self.target_amount,
            signature: "".to_string(),
            hash: "".to_string(),
            timestamp: timestamp as u64,
        };
        new_tx.sign(self.private_key.clone());

        let resp = post(&format!("{}/tx", NODE_URI))
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
        let config = config::Config {
            wallet: self.wallet,
            address: self.address.clone(),
            private_key: self.private_key.clone(),
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
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.message != *"" {
                egui::Window::new("Message").show(ui.ctx(), |ui| {
                    ui.label(self.message.clone());

                    if ui.button("Close").clicked() {
                        self.message = "".to_string();
                    }
                });
            }

            ui.heading("Pixelpay Wallet");
            ui.horizontal(|ui| {
                ui.label("Wallet: ");
                ui.label(format!("{} PXL", self.wallet));
                ui.horizontal(|ui| {
                    if ui.button("Refresh").clicked() {
                        self.get_balance();
                    }
                });
            });

            ui.horizontal(|ui| {
                ui.label("Address: ");
                ui.label(self.address.to_string().split_at(15).0.to_owned() + "...");
                if ui.button("Copy").clicked() {
                    self.copy_address();
                }
            });

            ui.separator();

            ui.heading("Send PXL");
            ui.horizontal(|ui| {
                ui.label("Target address: ");
                ui.text_edit_singleline(&mut self.target_address);
            });

            ui.horizontal(|ui| {
                ui.label("Amount: ");
                ui.add(egui::DragValue::new(&mut self.target_amount).speed(1.0));
            });

            ui.horizontal(|ui| {
                if ui.button("Send").clicked() {
                    self.send();
                }
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(360.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Pixelpay wallet",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}
