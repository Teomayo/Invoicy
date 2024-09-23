mod document;
mod functions;
mod structs;

use eframe::egui::{self, FontId, ProgressBar, Rect, Response, RichText, TextEdit, Ui, Window};
use egui::{Style, Vec2};
use egui_extras::{Column, TableBuilder};
use functions::*;
use rfd::FileDialog;
use rusqlite::{params, Connection};
use std::fs;
use std::{convert::TryInto, path::PathBuf};
use structs::*;

const LOGGER: bool = false;

fn main() {
    egui_logger::builder().init().unwrap();
    let options = eframe::NativeOptions {
        // with_icon causes crashes on application when using 'cargo build' or 'cargo release'
        viewport: egui::ViewportBuilder::default()
            // .with_icon(load_icon("support/images/128x128.png"))
            .with_inner_size([450.0, 320.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Invoicy",
        options,
        Box::new(|_cc| {
            // This gives us image support:
            Ok(Box::<Invoicy>::default())
        }),
    );
}

impl eframe::App for Invoicy {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if LOGGER == true {
            egui::Window::new("Log").show(ctx, |ui| {
                // draws the logger ui.
                egui_logger::logger_ui().show(ui);
            });
        }
        if !self.initialized {
            self.setup_tables();
            // sets up tables to have one customer and contact as place holders
            self.add_contact();
            self.add_customer();
            self.totals.push(Total {
                value: 0.0,
                position: (0, 4),
            });

            let _ = self.get_contacts();
            let _ = self.get_customers();
            let _ = self.get_data();

            self.style.spacing.button_padding = Vec2::splat(5.0); // Set horizontal and vertical margins

            self.file_name = format!(
                "{}-{:?}",
                sanitize_string(&self.customer.company),
                self.current_row_value.estimate_number
            );

            self.initialized = true;
        }
        // constantly updates based on the customer picked
        self.update_estimate_number();
        self.update_file_name();
        // form buttons
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(2.0);
            ui.style_mut().spacing.button_padding = self.style.spacing.button_padding;
            ui.horizontal(|ui| {
                // template button disabled until ready to be worked on
                // if ui
                //     .add_enabled(false, egui::Button::new("+ template"))
                //     .clicked()
                // {
                //     println!("{:?}", "template button not yet functional");
                // }
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
                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    if ui.button("+ contact").clicked() {
                        self.contact_form = true
                    }
                    if ui.button("+ customer").clicked() {
                        self.customer_form = true
                    }
                });
            });
            self.show_form(ui);
            ui.add_space(2.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // customer and contact selection
            ui.add_space(2.0);
            ui.horizontal(|ui| {
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

                // table area
            });

            ui.spacing();
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
                                            if !contains_field(
                                                &self.totals,
                                                &incoming_total.position,
                                            ) {
                                                self.totals.push(incoming_total.clone())
                                            }
                                            for idx in 0..self.totals.len() {
                                                if self.totals[idx].position
                                                    == incoming_total.position
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
            ui.add_space(2.0);
        });
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.add_space(2.0);
            self.progress = 0.0;
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        // TODO: needs way to strip special chars and make all one case
                        ui.text_edit_singleline(&mut self.file_name);
                        ui.label(".pdf")
                    });

                    if ui.button("Generate Invoice").clicked() {
                        if let Some(path) = FileDialog::new()
                            .set_file_name(format!("{}.pdf", self.file_name.clone()))
                            .save_file()
                        {
                            // Handle the file path here
                            let result = document::generate_invoice(
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
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    if ui.button("?").clicked() {
                        let to = "tode.crnobrnja@example.com";
                        let subject = "Report Issue";
                        let mailto = format!("mailto:{}?subject={}", to, subject,);
                        open::that(mailto).unwrap();
                    }
                });
            });
            ui.add_space(2.0);
        });
    }
}

impl Default for Invoicy {
    fn default() -> Self {
        Self {
            max_file_size: 15360,
            image_file_path: Some(PathBuf::new()),
            company_error_contact: Some("".to_string()),
            company_error_customer: Some("".to_string()),
            initialized: false,
            progress: 0.0,
            connection: Connection::open("invoicy.db").unwrap(),
            style: Style::default(),
            file_name: "invoice.pdf".to_string(),
            customer_selected: 0,
            contact_selected: 0,
            table_data: [].to_vec(),
            database_data_vec: [].to_vec(),
            new_database_data_vec: [].to_vec(),
            row_count: 1,
            last_updated_row: 0,
            contact: Contact {
                company: "Fake Co.".to_string(),
                address: "1111 Fake Ave.".to_string(),
                city: "Fakeston".to_string(),
                postal_code: "F4K 3E5".to_string(),
                country: "United Fakes".to_string(),
                name: "Fake Fake Smith".to_string(),
                telephone: "111-111-1111".to_string(),
                email: "fake@fake.com".to_string(),
                website: "fake.fake".to_string(),
            },
            contacts: [].to_vec(),
            contact_form: false,
            customer: Customer {
                company: "Fake Co. 2".to_string(),
                address: "1112 Fake Ave.".to_string(),
                city: "Fakeshire".to_string(),
                postal_code: "F4K 3A3".to_string(),
                country: "Fakeland".to_string(),
            },
            customers: [].to_vec(),
            customer_form: false,
            current_row_value: DatabaseData {
                entry_id: "FAKE-1-0".to_string(),
                cust_id: "FAKE".to_string(),
                row_number: 0,
                description: "write something down".to_string(),
                quantity: 1.0,
                price: 10.0,
                total: 10.0,
                estimate_number: 1,
            },
            grand_total: 0.0,
            totals: [].to_vec(),
        }
    }
}
impl Invoicy {
    fn calculate_grand_total(&mut self) {
        self.grand_total = self.totals.iter().map(|item| item.value).sum();
    }
    fn update_file_name(&mut self) {
        self.file_name = format!(
            "{}-{:?}",
            sanitize_string(&self.customers[self.customer_selected].company.clone()),
            self.current_row_value.estimate_number
        );
    }
    fn update_estimate_number(&mut self) {
        let curr_estimate_num = self
            .database_data_vec
            .iter()
            .filter(|x| x.cust_id == self.current_row_value.cust_id)
            .map(|item| item.estimate_number)
            .max();
        if curr_estimate_num == None {
            self.current_row_value.estimate_number = 1
        } else {
            self.current_row_value.estimate_number = curr_estimate_num.unwrap() + 1
        }
    }
    fn generate_customer_id(&mut self, idx: usize) -> String {
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
    fn get_contacts(&mut self) -> Result<String, rusqlite::Error> {
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
    fn get_customers(&mut self) -> Result<String, rusqlite::Error> {
        let mut stmt = self.connection.prepare("SELECT * FROM customers")?;
        let rows = stmt.query_map([], |row| {
            Ok(Customer {
                company: row.get(0)?,
                address: row.get(1)?,
                city: row.get(2)?,
                postal_code: row.get(3)?,
                country: row.get(4)?,
            })
        })?;
        for customer_row in rows {
            self.customers.push(customer_row.unwrap())
        }
        Ok("Customers Initialized from DB.".to_string())
    }
    fn get_data(&mut self) -> Result<String, rusqlite::Error> {
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

    fn setup_tables(&mut self) {
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

    fn add_contact(&mut self) {
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
    fn add_customer(&mut self) {
        let updated = &self.connection.execute(
            "INSERT OR REPLACE INTO customers (company, address, city, postal_code, country, estimate_number) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                self.customer.company.clone(),
                self.customer.address.clone(),
                self.customer.city.clone(),
                self.customer.postal_code.clone(),
                self.customer.country.clone(),
                self.current_row_value.estimate_number.clone()
            ],
        );
        match updated {
            Ok(value) => println!("LOG: Customer Added Successfully: {}", value),
            Err(e) => println!("ERROR: Customer unable to be Added {}", e),
        }
    }
    fn add_data(&mut self) {
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
    fn show_form(&mut self, ui: &mut Ui) {
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
}
