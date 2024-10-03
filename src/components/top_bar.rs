use std::{fs, path::PathBuf};

use crate::functions::validate_text_input;
use crate::structs::{Contact, Customer};
use crate::Invoicy;
use eframe::egui::{self, Window};
use egui::Ui;
use rusqlite::params;

// functions related to Top Bar UI
impl Invoicy {
    pub fn show_form(&mut self, ui: &mut Ui) {
        if self.contact_form {
            Window::new("Contact Form").show(ui.ctx(), |ui| {
                ui.label("Fill out the required data below");
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Company Name: ");
                        let response = ui.text_edit_singleline(&mut self.contact.company);
                        if response.changed() {
                            self.company_error_contact = validate_text_input(&self.contact.company);
                        }
                        if let Some(error) = &self.company_error_contact {
                            ui.colored_label(egui::Color32::RED, error);
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Address: ");
                        ui.text_edit_singleline(&mut self.contact.address);
                    });
                    ui.horizontal(|ui| {
                        ui.label("City: ");
                        ui.text_edit_singleline(&mut self.contact.city);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Postal Code: ");
                        ui.text_edit_singleline(&mut self.contact.postal_code);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Country: ");
                        ui.text_edit_singleline(&mut self.contact.country);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Name: ");
                        ui.text_edit_singleline(&mut self.contact.name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Telephone Number: ");
                        ui.text_edit_singleline(&mut self.contact.telephone);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Email: ");
                        ui.text_edit_singleline(&mut self.contact.email);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Website: ");
                        ui.text_edit_singleline(&mut self.contact.website);
                    });
                    if ui.button("Save Contact").clicked() {
                        // can add checks for same contact later on
                        self.contacts.push(self.contact.clone());
                        self.add_contact();
                        self.contact_form = false;
                    };
                    ui.separator();
                    if ui.button("Close").clicked() {
                        self.contact_form = false;
                    }
                });
            });
        }
        if self.customer_form {
            Window::new("Customer Form").show(ui.ctx(), |ui| {
                ui.label("Fill out the required data below");
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Company Name: ");
                        let response = ui.text_edit_singleline(&mut self.customer.company);
                        if response.changed() {
                            self.company_error_customer =
                                validate_text_input(&self.customer.company);
                        }
                        if let Some(error) = &self.company_error_customer {
                            ui.colored_label(egui::Color32::RED, error);
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Address: ");
                        ui.text_edit_singleline(&mut self.customer.address);
                    });
                    ui.horizontal(|ui| {
                        ui.label("City: ");
                        ui.text_edit_singleline(&mut self.customer.city);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Postal Code: ");
                        ui.text_edit_singleline(&mut self.customer.postal_code);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Country: ");
                        ui.text_edit_singleline(&mut self.customer.country);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Email: ");
                        ui.text_edit_singleline(&mut self.customer.email);
                    });
                    if ui.button("Save Customer").clicked() {
                        // can add checks for same contact later on

                        self.customers.push(self.customer.clone());
                        self.add_customer();
                        self.customer_form = false;
                    };
                    ui.separator();
                    if ui.button("Close").clicked() {
                        self.customer_form = false;
                    }
                });
            });
        }
    }
    pub fn add_contact(&mut self) {
        let updated = &self.connection.execute(
            "INSERT OR REPLACE INTO contacts (company, address, city, postal_code, country, name, telephone, email, website )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ",
            params![
                self.contact.company.clone(),
                self.contact.address.clone(),
                self.contact.city.clone(),
                self.contact.postal_code.clone(),
                self.contact.country.clone(),
                self.contact.name.clone(),
                self.contact.telephone.clone(),
                self.contact.email.clone(),
                self.contact.website.clone()
            ],
        );
        match updated {
            Ok(value) => println!("LOG: Contact Added Succesfully {}", value),
            Err(e) => println!("ERROR: Contact unable to be Added {}", e),
        }
    }
    pub fn add_customer(&mut self) {
        let updated = &self.connection.execute(
            "INSERT OR REPLACE INTO customers (company, address, city, postal_code, country, email, estimate_number) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                self.customer.company.clone(),
                self.customer.address.clone(),
                self.customer.city.clone(),
                self.customer.postal_code.clone(),
                self.customer.country.clone(),
                self.customer.email.clone(),
                self.current_row_value.estimate_number.clone()
            ],
        );
        match updated {
            Ok(value) => println!("LOG: Customer Added Successfully: {}", value),
            Err(e) => println!("ERROR: Customer unable to be Added {}", e),
        }
    }
    pub fn upload_logo(&mut self, ui: &mut Ui) {
        if ui.button("upload logo").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("jpg", &["jpg"])
                .pick_file()
            {
                let metadata = fs::metadata(&path).unwrap();
                if metadata.len() as usize <= self.max_file_size {
                    self.image_file_path = Some(path);
                } else {
                    ui.label("File size exceeds the limit.");
                }
            }
            if let Some(ref path) = self.image_file_path {
                // currently only supports one logo option.
                // This would change with the template feature.
                ui.label(format!("Selected file: {:?}", path));
                let destination = PathBuf::from("support/images/logo.jpg");
                let result = fs::copy(path, destination);
                match result {
                    Ok(value) => println!("LOG: Logo Upload Successful {}", value),
                    Err(e) => println!("ERROR: Failure to Load Logo {}", e),
                }
            }
        }
    }
    pub fn customer_and_contact_buttons(&mut self, ui: &mut Ui) {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
            if ui.button("+ contact").clicked() {
                self.contact_form = true
            }
            if ui.button("+ customer").clicked() {
                self.customer_form = true
            }
        });
    }
}

// functions related to Top Bar actions
impl Invoicy {
    pub fn get_contacts(&mut self) -> Result<String, rusqlite::Error> {
        let mut stmt = self.connection.prepare("SELECT * FROM contacts")?;
        let rows = stmt.query_map([], |row| {
            Ok(Contact {
                company: row.get(0)?,
                address: row.get(1)?,
                city: row.get(2)?,
                postal_code: row.get(3)?,
                country: row.get(4)?,
                name: row.get(5)?,
                telephone: row.get(6)?,
                email: row.get(7)?,
                website: row.get(8)?,
            })
        })?;
        for contact_row in rows {
            self.contacts.push(contact_row.unwrap())
        }
        Ok("Contacts Initialized from DB.".to_string())
    }
    pub fn get_customers(&mut self) -> Result<String, rusqlite::Error> {
        let mut stmt = self.connection.prepare("SELECT * FROM customers")?;
        let rows = stmt.query_map([], |row| {
            Ok(Customer {
                company: row.get(0)?,
                address: row.get(1)?,
                city: row.get(2)?,
                postal_code: row.get(3)?,
                country: row.get(4)?,
                email: row.get(5)?,
            })
        })?;
        for customer_row in rows {
            self.customers.push(customer_row.unwrap())
        }
        Ok("Customers Initialized from DB.".to_string())
    }
}
