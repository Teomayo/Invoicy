use bcrypt::{hash, DEFAULT_COST};
use eframe::egui::{menu, Align2, Ui, Window};
use rusqlite::params;

use crate::Invoicy;

impl Invoicy {
    pub fn show_menu(&mut self, ui: &mut Ui) {
        menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Add Email").clicked() {
                    self.auth_email_dialog = true
                }
                if ui.button("Quit").clicked() {
                    self.password_for_auth = "".to_string();
                    std::process::exit(0);
                }
            });
        });
        self.credentials_dialog(ui);
    }

    fn credentials_dialog(&mut self, ui: &mut Ui) {
        if self.auth_email_dialog {
            Window::new("Email Credentials")
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ui.ctx(), |ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Email");
                            ui.text_edit_singleline(&mut self.email_for_auth);
                        });
                        ui.horizontal(|ui| {
                            ui.label("Password");
                            ui.text_edit_singleline(&mut self.password_for_auth);
                        });
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("Submit").clicked() {
                            let password_hash = hash(self.password_for_auth.clone(), DEFAULT_COST)
                                .expect("Failed to hash password");
                            // Insert the credentials into the database
                            let result = self.connection.execute(
                                "INSERT INTO credentials (email, password, password_hash) VALUES (?1, ?2, ?3)",
                                params![self.email_for_auth, self.password_for_auth, password_hash],
                            );
                            match result {
                                Ok(value) => {
                                    println!("LOG: Credentials Added to Database {}", value)
                                }
                                Err(e) => println!("ERROR: Credentials Failed to be added {}", e),
                            }
                            self.password_for_auth = "".to_string();
                            self.auth_email_dialog = false;
                        };
                        if ui.button("Close").clicked() {
                            self.password_for_auth = "".to_string();
                            self.auth_email_dialog = false;
                        };
                    });
                });
        }
    }
}
