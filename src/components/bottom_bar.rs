use std::convert::TryInto;

use eframe::egui::{Align, Layout, ProgressBar, Ui};
use rfd::FileDialog;
use rusqlite::params;

use crate::document::generate_invoice;
use crate::functions::sanitize_string;
use crate::structs::DatabaseData;
use crate::Invoicy;

// Functions related to Bottom Bar UI
impl Invoicy {
    pub fn file_namer(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            // TODO: needs way to strip special chars and make all one case
            ui.text_edit_singleline(&mut self.file_name);
            ui.label(".pdf")
        });
    }

    pub fn generate_invoice(&mut self, ui: &mut Ui) {
        if ui.button("Generate Invoice").clicked() {
            if let Some(path) = FileDialog::new()
                .set_file_name(format!("{}.pdf", self.file_name.clone()))
                .save_file()
            {
                // Handle the file path here
                let result = generate_invoice(
                    &path,
                    self.contact.clone(),
                    self.customer.clone(),
                    self.table_data.clone(),
                    self.current_row_value
                        .estimate_number
                        .clone()
                        .try_into()
                        .unwrap(),
                    self.grand_total,
                );
                println!("{:?}", result);
                println!("File saved to: {:?}", &path);
                self.progress = 100.0;
                ui.add(ProgressBar::new(self.progress).show_percentage());
                self.add_data();
                self.add_customer();
            }
        }
    }

    pub fn send_report(&mut self, ui: &mut Ui) {
        ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
            if ui.button("?").clicked() {
                let to = "tode.crnobrnja@example.com";
                let subject = "Report Issue";
                let mailto = format!("mailto:{}?subject={}", to, subject,);
                open::that(mailto).unwrap();
            }
        });
    }
}

// Functiona related to Bottom Bar Actions
impl Invoicy {
    pub fn update_file_name(&mut self) {
        self.file_name = format!(
            "{}-{:?}",
            sanitize_string(&self.customers[self.customer_selected].company.clone()),
            self.current_row_value.estimate_number
        );
    }

    pub fn add_data(&mut self) {
        for i in 0..self.row_count {
            let mut data: DatabaseData = DatabaseData {
                entry_id: "".to_string(),
                cust_id: self.generate_customer_id(self.customer_selected),
                estimate_number: self.current_row_value.estimate_number,
                row_number: 0,
                description: "".to_string(),
                quantity: 1.0,
                price: 1.0,
                total: 1.0,
            };
            for _ in 0..5 {
                for item in &self.table_data {
                    if item.1 == (i, 0) {
                        data.row_number = i;
                    } else if item.1 == (i, 1) {
                        data.description = item.0.clone()
                    } else if item.1 == (i, 2) {
                        if item.0.parse::<f64>().is_ok() {
                            data.quantity = item.0.parse().unwrap();
                        } else {
                            data.quantity = 0.0
                        };
                    } else if item.1 == (i, 3) {
                        if item.0.parse::<f64>().is_ok() {
                            data.price = item.0.parse().unwrap();
                        } else {
                            data.price = 0.0
                        };
                    } else if item.1 == (i, 4) {
                        if item.0.parse::<f64>().is_ok() {
                            data.total = item.0.parse().unwrap();
                        } else {
                            data.total = 0.0
                        };
                    }
                }
            }

            data.entry_id = format!(
                "{}-{:?}-{:?}",
                self.generate_customer_id(self.customer_selected),
                data.estimate_number,
                data.row_number
            );
            self.new_database_data_vec.push(data);
        }
        for item in &self.new_database_data_vec {
            let updated = &self.connection.execute(
                        "INSERT OR REPLACE INTO data (entry_id, estimate_number, cust_id, row_number, description, quantity, price, total) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                        params![
                            item.entry_id,
                            item.estimate_number,
                            item.cust_id,
                            item.row_number,
                            item.description,
                            item.quantity,
                            item.price,
                            item.total,

                        ],
                    );
            match updated {
                Ok(value) => println!("LOG: Data Added Successfully: {}", value),
                Err(e) => println!("LOG: Error in Data Addition: {}", e),
            }
        }
        self.database_data_vec
            .extend_from_slice(&self.new_database_data_vec);
        self.new_database_data_vec.clear();
    }
}
