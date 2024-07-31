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

mod csv_test;
mod document;

use eframe::egui::{self, TextEdit};
use egui_extras::{Column, TableBuilder};

const IMAGE_PATH_JPG: &'static str = r"images/farbalogo.jpg";

const DIR_NAME: &str = r"fonts/JetbrainsMono/";

const PERSONAL_CSV: &str = r"src/csv/personal.csv";
const CUSTOMER_CSV: &str = r"src/csv/customer.csv";
const ITEMS_CSV: &str = r"src/csv/items.csv";

const ESTIMATE_NUMBER: i32 = 1;
fn main() {
    let result = csv_test::main(PERSONAL_CSV, CUSTOMER_CSV, ITEMS_CSV);
    let records = result.unwrap();
    let (personal_customer_records, customer_records, item_records) = records;

    document::generate_document(
        DIR_NAME,
        IMAGE_PATH_JPG,
        personal_customer_records,
        customer_records,
        item_records,
        ESTIMATE_NUMBER,
    );

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

        // form buttons
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("+ template").clicked() {
                    println!("{:?}", "hello template");
                }
                ui.separator();
                if ui.button("+ customer").clicked() {
                    println!("{:?}", "hello customer");
                }
                if ui.button("+ contact").clicked() {
                    println!("{:?}", "hello contact");
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // customer and contact selection
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("Select Customer")
                    .selected_text(format!(
                        "{}",
                        &self.customer_selected_vec[self.customer_selected]
                    ))
                    .show_ui(ui, |ui| {
                        for i in 0..self.customer_selected_vec.len() {
                            let value = ui.selectable_value(
                                &mut &self.customer_selected_vec[i],
                                &self.customer_selected_vec[self.customer_selected],
                                &self.customer_selected_vec[i],
                            );
                            if value.clicked() {
                                self.customer_selected = i;
                            }
                        }
                    });

                egui::ComboBox::from_label("Select Contact")
                    .selected_text(format!(
                        "{}",
                        &self.contact_selected_vec[self.contact_selected]
                    ))
                    .show_ui(ui, |ui| {
                        for i in 0..self.contact_selected_vec.len() {
                            let value = ui.selectable_value(
                                &mut &self.contact_selected_vec[i],
                                &self.contact_selected_vec[self.contact_selected],
                                &self.contact_selected_vec[i],
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
                    for idx in 0..self.row_count {
                        body.row(30.0, |mut row| {
                            // currently hardcoded until there is a plan for table customizability
                            for column_count in 0..4 {
                                let mut new_val =
                                    (format!("{:?}", [idx, column_count]), (idx, column_count));
                                if column_count == 0 {
                                    row.col(|ui| {
                                        ui.label(idx.to_string());
                                    });
                                } else {
                                    row.col(|ui| {
                                        ui.add(TextEdit::singleline(&mut new_val.0));
                                        ui.end_row();
                                    });
                                }
                            }
                        });
                    }
                    self.last_updated_row += 1;
                    self.initialized = true;
                });
            if ui.button("+ Add Row").clicked() {
                self.new_row_added = true;
                self.row_count += 1;
                // println!("{:?}", self.table);
            }
        });
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            if ui.button("Generate Invoice").clicked() {
                println!("Feature not ready yet!")
            }
        });
    }
}

#[derive(Clone, PartialEq, Debug)]
struct MyApp {
    customer_selected: usize,
    customer_selected_vec: Vec<String>,
    contact_selected: usize,
    contact_selected_vec: Vec<String>,
    text: String,
    row_count: usize,
    current_row_count: usize,
    last_updated_row: usize,
    table_data: Vec<String>,
    new_row_added: bool,
    initialized: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            customer_selected: 0,
            customer_selected_vec: Self::get_vec(),
            contact_selected: 0,
            contact_selected_vec: Self::get_vec(),
            table_data: [].to_vec(),
            text: "".to_string(),
            row_count: 1,
            last_updated_row: 0,
            current_row_count: 0,
            new_row_added: false,
            initialized: false,
        }
    }
}

impl MyApp {
    fn get_vec() -> Vec<String> {
        let vecs = ["1".to_string(), "2".to_string(), "3".to_string()].to_vec();
        return vecs;
    }
}
