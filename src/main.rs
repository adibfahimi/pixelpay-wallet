#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod config;
mod myapp;
mod tx;

use eframe::egui;
use myapp::MyApp;
extern crate clipboard;

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

            ui.horizontal(|ui| {
                ui.heading("Wallet: ");
                ui.label(
                    egui::RichText::new(format!("{} PXL", self.wallet))
                        .heading()
                        .color(egui::Color32::from_rgb(255, 255, 255)),
                );
                ui.horizontal(|ui| {
                    if ui.button("Refresh").clicked() {
                        self.get_balance();
                    }
                });
            });

            ui.horizontal(|ui| {
                ui.label("Address: ");
                ui.label(self.address.to_string().split_at(25).0.to_owned() + "...");
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

