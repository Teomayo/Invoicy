// SPDX-FileCopyrightText: 2021 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: CC0-1.0

//! This example generates a minimal PDF document and writes it to the path that was passed as the
//! first command-line argument.  The size of the generated document should be 2.0 KB.
//!
//! You may have to adapt the `FONT_DIRS` and `DEFAULT_FONT_NAME` constants for your system so that
//! these files exist:
//! - `{FONT_DIR}/{DEFAULT_FONT_NAME}-Regular.ttf`
//! - `{FONT_DIR}/{DEFAULT_FONT_NAME}-Bold.ttf`
//! - `{FONT_DIR}/{DEFAULT_FONT_NAME}-Italic.ttf`
//! - `{FONT_DIR}/{DEFAULT_FONT_NAME}-BoldItalic.ttf`
//!
//! These fonts must be metrically identical to the built-in PDF sans-serif font (Helvetica/Arial).

mod document;

use eframe::egui::{self, FontId, Rect, Response, RichText, TextEdit, Ui, Window};
use egui::{Style, Vec2};
use egui_extras::{Column, TableBuilder};
use rusqlite::{params, Connection};

const IMAGE_PATH_JPG: &'static str = r"images/farbalogo.jpg";
const DIR_NAME: &str = r"fonts/JetbrainsMono/";

const ESTIMATE_NUMBER: i32 = 1;
fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([450.0, 320.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Invoicy",
        options,
        Box::new(|_cc| {
            // This gives us image support:
            Ok(Box::<MyApp>::default())
        }),
    );
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.initialized {
            self.setup_tables();
            // sets up tables to have one customer and contact as place holders
            self.add_contact();
            self.add_customer(ESTIMATE_NUMBER);
            self.totals.push(Total {
                value: 0.0,
                position: (0, 4),
            });

            let _ = self.get_contacts();
            let _ = self.get_customers();

            self.style.spacing.button_padding = Vec2::splat(5.0); // Set horizontal and vertical margins

            self.file_name = format!(
                "{}-{:?}",
                sanitize_string(&self.customer.company),
                ESTIMATE_NUMBER
            );
            self.initialized = true;
        }

        // form buttons
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(2.0);
            ui.style_mut().spacing.button_padding = self.style.spacing.button_padding;
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(false, egui::Button::new("+ template"))
                    .clicked()
                {
                    println!("{:?}", "template button not yet functional");
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
                                self.file_name = format!(
                                    "{}-{:?}",
                                    &self.customers[i].company.clone(),
                                    ESTIMATE_NUMBER
                                );
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

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        // TODO: needs way to strip special chars and make all one case
                        ui.text_edit_singleline(&mut self.file_name);
                        ui.label(".pdf")
                    });
                    if ui.button("Generate Invoice").clicked() {
                        document::generate_invoice(
                            DIR_NAME,
                            IMAGE_PATH_JPG,
                            format!("{}.pdf", self.file_name.clone()),
                            self.contact.clone(),
                            self.customer.clone(),
                            self.table_data.clone(),
                            ESTIMATE_NUMBER,
                            self.grand_total,
                        );
                    }
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    if ui.button("?").clicked() {
                        println!("report something");
                    }
                });
            });
            ui.add_space(2.0);
        });
    }
}

#[derive(Debug)]
struct MyApp {
    initialized: bool,
    connection: Connection,
    style: Style,
    file_name: String,
    customer_selected: usize,
    contact_selected: usize,
    row_count: usize,
    last_updated_row: usize,
    table_data: Vec<(String, (usize, i32), (Rect, Response))>,
    contact: Contact,
    contacts: Vec<Contact>,
    contact_form: bool,
    customer: Customer,
    customers: Vec<Customer>,
    customer_form: bool,
    current_row_value: RowValues,
    totals: Vec<Total>,
    grand_total: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct Total {
    value: f64,
    position: (usize, i32),
}

#[derive(Clone, Debug, PartialEq)]
struct Customer {
    company: String,
    address: String,
    city: String,
    postal_code: String,
    country: String,
}
#[derive(Clone, Debug)]
struct RowValues {
    description: String,
    quantity: f64,
    price: f64,
    total: f64,
}
#[derive(Clone, Debug, PartialEq)]

struct Contact {
    company: String,
    address: String,
    city: String,
    postal_code: String,
    country: String,
    name: String,
    telephone: String,
    email: String,
    website: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            initialized: false,
            connection: Connection::open("invoicy.db").unwrap(),
            style: Style::default(),
            file_name: "invoice.pdf".to_string(),
            customer_selected: 0,
            contact_selected: 0,
            table_data: [].to_vec(),
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
            current_row_value: RowValues {
                description: "".to_string(),
                quantity: 1.0,
                price: 10.0,
                total: 10.0,
            },
            grand_total: 0.0,
            totals: [].to_vec(),
        }
    }
}

fn contains_field(vec: &Vec<Total>, position: &(usize, i32)) -> bool {
    vec.iter().any(|s| s.position == *position)
}
fn sanitize_string(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() {
                '_'
            } else {
                '_'
            }
        })
        .collect()
}
impl MyApp {
    fn calculate_grand_total(&mut self) {
        self.grand_total = self.totals.iter().map(|item| item.value).sum();
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
            Ok(value) => println!("Success: {}", value),
            Err(e) => println!("Error: {}", e),
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
            Ok(value) => println!("Success: {}", value),
            Err(e) => println!("Error: {}", e),
        }
        let data_result = self.connection.execute(
            "CREATE TABLE IF NOT EXISTS data (
                    id INTEGER PRIMARY KEY,
                    cust_id INTEGER NOT NULL,
                    description TEXT NOT NULL,
                    quantity REAL,
                    price REAL,
                    total REAL,
                    FOREIGN KEY(cust_id) REFERENCES Customer(id)
                )",
            [],
        );
        match data_result {
            Ok(value) => println!("Success: {}", value),
            Err(e) => println!("Error: {}", e),
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
            Ok(value) => println!("Success: {}", value),
            Err(e) => println!("Error: {}", e),
        }
    }
    fn add_customer(&mut self, estimate_number: i32) {
        let updated = &self.connection.execute(
            "INSERT OR REPLACE INTO customers (company, address, city, postal_code, country, estimate_number) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                self.customer.company.clone(),
                self.customer.address.clone(),
                self.customer.city.clone(),
                self.customer.postal_code.clone(),
                self.customer.country.clone(),
                estimate_number
            ],
        );
        match updated {
            Ok(value) => println!("Success: {}", value),
            Err(e) => println!("Error: {}", e),
        }
    }
    // fn add_data(&mut self) {
    //     for idx in 0..self.row_count {
    //             // currently hardcoded until there is a plan for table customizability
    //             for column_count in 0..5 {
    //                 for cell in &mut self.table_data {

    //                     if column_count == 1 {

    //                     } else if column_count == 2 {

    //                     } else if column_count == 3 {

    //                     } else if column_count == 4 {

    //                     } else {

    //                     }
    //                     let updated = &self.connection.execute(
    //                         "INSERT INTO data (description, quantity, price, total, cust_id) VALUES (?1, ?2, ?3, ?4, ?5)",
    //                         params![
    //                             cell.0,

    //                         ],
    //                     );
    //                     match updated {
    //                         Ok(value) => println!("Success: {}", value),
    //                         Err(e) => println!("Error: {}", e),
    //                     }
    //             }
    //         }
    //     }
    // }
    fn show_form(&mut self, ui: &mut Ui) {
        if self.contact_form {
            Window::new("Contact Form").show(ui.ctx(), |ui| {
                ui.label("Fill out the required data below");
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Company Name: ");
                        ui.text_edit_singleline(&mut self.contact.company);
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
                        ui.text_edit_singleline(&mut self.customer.company);
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
                        self.add_customer(ESTIMATE_NUMBER);
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
