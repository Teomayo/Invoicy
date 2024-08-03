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

use eframe::egui::{self, Rect, Response, TextEdit, Ui, Window};
use egui_extras::{Column, TableBuilder};

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
        // ui.heading("My egui Application");

        if !self.initialized {
            self.customers.push(self.customer.clone());
            self.contacts.push(self.contact.clone());
            self.file_name = format!(
                "{:?}-{:?}.pdf",
                self.customer.company.to_string(),
                ESTIMATE_NUMBER
            );
            self.initialized = true;
        }

        // form buttons
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("+ template").clicked() {
                    println!("{:?}", "template button not yet functional");
                }
                ui.separator();
                if ui.button("+ customer").clicked() {
                    self.customer_form = true
                }
                if ui.button("+ contact").clicked() {
                    self.contact_form = true
                }
            });
            self.show_form(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // customer and contact selection
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
                        println!(
                            "{}",
                            format!(
                                "last updated row: {:?}\nrow count: {:?}",
                                self.last_updated_row, self.row_count
                            )
                        );
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
                                            self.current_row_value.total =
                                                self.current_row_value.quantity
                                                    * self.current_row_value.price;
                                            let total_val = self.current_row_value.total;
                                            cell.0 = total_val.to_string();
                                            row.col(|ui| {
                                                ui.label(format!("{:?}", total_val));
                                            });
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
            ui.horizontal(|ui| {
                if ui.button("+ Add Row").clicked() {
                    self.row_count += 1;
                }
                if ui.button("- Delete Row").clicked() {
                    self.row_count -= 1;
                    // can add loop to delete row based on latest index. For now the data persists in table_data
                }
            });
        });
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    // TODO: needs way to strip special chars and make all one case
                    ui.text_edit_singleline(&mut self.file_name);
                });
                if ui.button("Generate Invoice").clicked() {
                    document::generate_invoice(
                        DIR_NAME,
                        IMAGE_PATH_JPG,
                        self.file_name.clone(),
                        self.contact.clone(),
                        self.customer.clone(),
                        self.table_data.clone(),
                        ESTIMATE_NUMBER,
                    );
                }
            });
        });
    }
}

#[derive(Clone, Debug)]
struct MyApp {
    initialized: bool,
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
                email: "fake*fake.com".to_string(),
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
        }
    }
}

impl MyApp {
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
