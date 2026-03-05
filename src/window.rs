use crate::vault::*;

use eframe::egui;
use std::sync::{Arc, Mutex};

pub struct Window {
    vault: Arc<Mutex<Vault>>,
    master: String,
    unlocked: bool,
    new_entry_name: String,
    new_entry_user: String,
    new_entry_password: String,
    new_entry_location: String,
}

impl Window {
    pub fn new(vault: Arc<Mutex<Vault>>, master: String) -> Self {
        Self {
            vault,
            master,
            unlocked: false,
            new_entry_name: String::new(),
            new_entry_user: String::new(),
            new_entry_password: String::new(),
            new_entry_location: String::new(),
        }
    }
}

impl eframe::App for Window {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::SidePanel::left("sidebar").show(ctx, |ui| {
            ui.heading("🔒 Vault");
            ui.separator();

            if !self.unlocked {
                ui.label("Master Password:");
                ui.text_edit_singleline(&mut self.master);

                if ui.button("Unlock").clicked() {
                    if self.vault.lock().unwrap().verify_master(self.master.clone()) {
                        self.unlocked = true;
                    } else {
                        ui.text_edit_singleline(&mut "Invalide password");
                    }
                }
            } else {
                if ui.button("Add New Entry").clicked() {
                    self.new_entry_name.clear();
                    self.new_entry_user.clear();
                    self.new_entry_password.clear();
                    self.new_entry_location.clear();
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.unlocked {
                ui.vertical_centered(|ui| {
                    ui.label("Enter your master password to unlock the vault");
                });
                return;
            }

            ui.horizontal(|ui| {
                // Liste des entrées
                ui.vertical(|ui| {
                    ui.heading("Vault Entries");
                    ui.separator();

                    let vault_guard = self.vault.lock().unwrap();
                    egui::ScrollArea::vertical()
                        .max_height(400.0)
                        .show(ui, |ui| {
                            for (i, entry) in vault_guard.entry.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.label(format!("{} - {}", i + 1, entry.0));
                                    if ui.button("Show").clicked() {
                                        vault_guard.unlock(self.master.as_str(), entry.0.clone());
                                        // Déchiffrer et afficher si nécessaire
                                    }
                                    if ui.button("Delete").clicked() {
                                        // supprimer entrée
                                    }
                                });
                            }
                        });
                });

                ui.separator();

                // Formulaire Add Entry
                ui.vertical(|ui| {
                    ui.heading("Add New Entry");
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut self.new_entry_name);
                    ui.label("User:");
                    ui.text_edit_singleline(&mut self.new_entry_user);
                    ui.label("Password:");
                    ui.text_edit_singleline(&mut self.new_entry_password);
                    ui.label("Location:");
                    ui.text_edit_singleline(&mut self.new_entry_location);

                    if ui.button("Save Entry").clicked() {
                        let data = Data::new(
                            self.new_entry_user.as_str(),
                             self.new_entry_password.as_str(),
                             self.new_entry_location.as_str()
                        );

                        let mut vault_guard = self.vault.lock().unwrap();
                        vault_guard.lock(self.master.as_str(), data, self.new_entry_name.clone());
                        drop(vault_guard);

                        self.new_entry_name.clear();
                        self.new_entry_user.clear();
                        self.new_entry_password.clear();
                        self.new_entry_location.clear();
                    }
                });
            });
        });
    }
}
