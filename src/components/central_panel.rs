use eframe::egui::{self, FontId, RichText, TextEdit, Ui};
use egui_extras::{Column, TableBuilder};

use crate::functions::*;
use crate::structs::{DatabaseData, Total};
use crate::Invoicy;

impl Invoicy {
    pub fn customer_select(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_label("Select Customer")
            .selected_text(&self.customers[self.customer_selected].company)
            .show_ui(ui, |ui| {
                for i in 0..self.customers.len() {
                    let value = ui.selectable_value(
                        &mut &self.customers[i],
                        &self.customers[self.customer_selected],
                        &self.customers[i].company,
                    );
                    if value.clicked() {
                        self.customer_selected = i;
                        self.current_row_value.cust_id = self.generate_customer_id(i);
                    }
                }
            });
    }

    pub fn contact_select(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_label("Select Contact")
            .selected_text(&self.contacts[self.contact_selected].company)
            .show_ui(ui, |ui| {
                for i in 0..self.contacts.len() {
                    let value = ui.selectable_value(
                        &mut &self.contacts[i],
                        &self.contacts[self.contact_selected],
                        &self.contacts[i].company,
                    );
                    if value.clicked() {
                        self.contact_selected = i;
                    }
                }
            });
    }

    pub fn table(&mut self, ui: &mut Ui) {
        TableBuilder::new(ui)
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::remainder())
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("Row #");
                });
                header.col(|ui| {
                    ui.heading("Description");
                });
                header.col(|ui| {
                    ui.heading("Quantity");
                });
                header.col(|ui| {
                    ui.heading("Unit Price");
                });
                header.col(|ui| {
                    ui.heading("Total");
                });
            })
            .body(|mut body| {
                if self.last_updated_row < self.row_count {
                    // sets up default rows and adds it to storage table
                    for idx in self.last_updated_row..self.row_count {
                        body.row(30.0, |mut row| {
                            // currently hardcoded until there is a plan for table customizability
                            for column_count in 0..5 {
                                let mut text = format!("{:?}", (idx, column_count));
                                if column_count == 0 {
                                    let output = row.col(|ui| {
                                        ui.label(idx.to_string());
                                    });
                                    self.table_data.push((text, (idx, column_count), output));
                                } else if column_count == 4 {
                                    let output = row.col(|ui| {
                                        ui.label(text.to_string());
                                    });
                                    self.table_data.push((text, (idx, column_count), output));
                                } else {
                                    let output = row.col(|ui| {
                                        ui.add(TextEdit::singleline(&mut text));
                                        ui.end_row();
                                    });
                                    // pushing all the table data into the requested amount of rows
                                    self.table_data.push((text, (idx, column_count), output));
                                }
                            }
                        });
                    }
                    self.last_updated_row = self.row_count;
                }
                // continuously updates tables based on table data
                for idx in 0..self.row_count {
                    body.row(30.0, |mut row| {
                        // currently hardcoded until there is a plan for table customizability
                        for column_count in 0..5 {
                            for cell in &mut self.table_data {
                                if (cell.1) == (idx, column_count) {
                                    if column_count == 0 {
                                        row.col(|ui| {
                                            ui.label(idx.to_string());
                                        });
                                    } else if column_count == 2 {
                                        self.current_row_value.quantity =
                                            if cell.0.parse::<f64>().is_ok() {
                                                cell.0.parse().unwrap()
                                            } else {
                                                0.0
                                            };
                                        row.col(|ui| {
                                            ui.add(TextEdit::singleline(&mut cell.0));
                                            ui.end_row();
                                        });
                                    } else if column_count == 3 {
                                        self.current_row_value.price =
                                            if cell.0.parse::<f64>().is_ok() {
                                                cell.0.parse().unwrap()
                                            } else {
                                                0.0
                                            };
                                        row.col(|ui| {
                                            ui.add(TextEdit::singleline(&mut cell.0));
                                            ui.end_row();
                                        });
                                    } else if column_count == 4 {
                                        let total_val = self.current_row_value.quantity.clone()
                                            * self.current_row_value.price.clone();
                                        cell.0 = total_val.to_string();
                                        row.col(|ui| {
                                            ui.label(format!("{:?}", total_val));
                                        });

                                        let incoming_total = Total {
                                            value: total_val,
                                            position: (idx, column_count),
                                        };
                                        if !contains_field(&self.totals, &incoming_total.position) {
                                            self.totals.push(incoming_total.clone())
                                        }
                                        for idx in 0..self.totals.len() {
                                            if self.totals[idx].position == incoming_total.position
                                            {
                                                self.totals[idx].value =
                                                    incoming_total.value.clone();
                                            }
                                        }
                                    } else {
                                        row.col(|ui| {
                                            ui.add(TextEdit::singleline(&mut cell.0));
                                            ui.end_row();
                                        });
                                    }
                                }
                            }
                        }
                    });
                }
            });
        self.calculate_grand_total();
        ui.label(
            RichText::new(format!("Grand Total: {}", self.grand_total.to_string()))
                .font(FontId::proportional(16.0)),
        );
        ui.horizontal(|ui| {
            if ui.button("+ Add Row").clicked() {
                self.row_count += 1;
            }
            if ui.button("- Delete Row").clicked() {
                self.row_count -= 1;
                // can add loop to delete row based on latest index. For now the data persists in table_data
            }
        });
    }
}

impl Invoicy {
    pub fn calculate_grand_total(&mut self) {
        self.grand_total = self.totals.iter().map(|item| item.value).sum();
    }

    pub fn generate_customer_id(&mut self, idx: usize) -> String {
        let cust_id = format!(
            "{}",
            &self.customers[idx]
                .company
                .clone()
                .to_uppercase()
                .get(0..4)
                .unwrap(),
        );
        return cust_id;
    }
}

impl Invoicy {
    pub fn get_data(&mut self) -> Result<String, rusqlite::Error> {
        let mut stmt = self.connection.prepare("SELECT * FROM data")?;
        let rows = stmt.query_map([], |row| {
            Ok(DatabaseData {
                entry_id: row.get(0)?,
                cust_id: row.get(1)?,
                estimate_number: row.get(2)?,
                row_number: row.get(3)?,
                description: row.get(4)?,
                quantity: row.get(5)?,
                price: row.get(6)?,
                total: row.get(7)?,
            })
        })?;
        // this will end up being slow as data builds up will need to figure out a better method
        for data_row in rows {
            self.database_data_vec.push(data_row.unwrap())
        }
        Ok("Data Initialized from DB.".to_string())
    }

    pub fn setup_tables(&mut self) {
        let customer_result = self.connection.execute(
            "CREATE TABLE IF NOT EXISTS customers (
                    company TEXT PRIMARY KEY,
                    address TEXT NOT NULL,
                    city TEXT NOT NULL,
                    postal_code TEXT NOT NULL,
                    country TEXT NOT NULL,
                    estimate_number INTEGER
                )",
            [],
        );
        match customer_result {
            Ok(value) => println!("LOG: Customer Table Setup was Successful: {}", value),
            Err(e) => println!("ERROR: Customer Table not setup Correctly {}", e),
        }
        let contact_result = self.connection.execute(
            "CREATE TABLE IF NOT EXISTS contacts (
                    company TEXT PRIMARY KEY,
                    address Text NOT NULL,
                    city TEXT NOT NULL,
                    postal_code TEXT NOT NULL,
                    country TEXT NOT NULL,
                    name TEXT NOT NULL,
                    telephone TEXT NOT NULL,
                    email TEXT NOT NULL,
                    website TEXT NOT NULL
                )",
            [],
        );
        match contact_result {
            Ok(value) => println!("LOG: Contact Table Setup was Successful {}", value),
            Err(e) => println!("ERROR: Contact Table not setup Correctly {}", e),
        }
        let data_result = self.connection.execute(
            "CREATE TABLE IF NOT EXISTS data (
                    entry_id TEXT PRIMARY KEY,
                    cust_id TEXT NOT NULL,
                    estimate_number INTEGER NOT NULL,
                    row_number INTEGER NOT NULL,
                    description TEXT NOT NULL,
                    quantity REAL,
                    price REAL,
                    total REAL
                )",
            [],
        );
        match data_result {
            Ok(value) => println!("LOG: Data Table Setup was Successful {}", value),
            Err(e) => println!("ERROR: Data Table not setup Correctly {}", e),
        }
    }
}
