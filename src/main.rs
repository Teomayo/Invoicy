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
use std::fmt::format;

use eframe::egui;

const IMAGE_PATH_JPG: &'static str = r"images/farbalogo.jpg";
const DIR_NAME: &str = r"fonts/JetbrainsMono/";
// const PERSONAL_INFO: Vec;
// const CUSTOMER_INFO: Vec;
// const ITEMS: Vec;

// find way to differentiate unix paths and windows paths

// windows
// const PERSONAL_CSV: &str = r"src\csv\personal.csv";
// const CUSTOMER_CSV: &str = r"src\csv\customer.csv";
// const ITEMS_CSV: &str = r"src\csv\items.csv";

// unix
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
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
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
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("My egui Application");
            ui.horizontal(|ui| {
                if ui.button("+ template").clicked() {}
                if ui.button("+ customer").clicked() {}
                if ui.button("+ contact").clicked() {}
            });

            // ui.horizontal( |ui| {
            //     let mut selected = CustomerSelect::First;
            //     egui::ComboBox::from_label("Select one!")
            //     .selected_text(format!("{:?}", selected))
            //     .show_ui(ui, |ui| {
            //         ui.selectable_value(&mut selected, CustomerSelect::First, "First");
            //         ui.selectable_value(&mut selected, CustomerSelect::Second, "Second");
            //         ui.selectable_value(&mut selected, CustomerSelect::Third, "Third");
            //     })}
            // );
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("Select one!")
                    .selected_text(format!("{:?}", &self.customer_selected))
                    .show_ui(ui, |ui| {
                        for i in 0..self.customer_selected_vec.len() {
                            let value = ui.selectable_value(
                                &mut &self.customer_selected_vec[i],
                                &self.customer_selected_vec[self.customer_selected],
                                &self.customer_selected_vec[i],
                            );
                        }
                    })
            });
            // ui.image(egui::include_image!(
            //     "../../../crates/egui/assets/ferris.png"
            // ));
        });
    }
}
#[derive(PartialEq, Debug)]
enum CustomerSelect {
    First,
    Second,
    Third,
}
struct MyApp {
    name: String,
    age: u32,
    customer_selected: usize,
    customer_selected_vec: Vec<String>,
    customers: CustomerSelect,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            customer_selected: 0,
            customers: CustomerSelect::First,
            customer_selected_vec: get_vec(),
        }
    }
}

fn get_vec() -> Vec<String> {
    let vecs = ["1".to_string(), "2".to_string(), "3".to_string()].to_vec();
    return vecs;
}

// pub struct PersonalInfoLayout {
//     name: &str,
//     address: &str,
//     company: &str,
//     city: &str,
//     postal_code: &str,
//     country: &str,
//     telephone: i32,
//     email: &str,
//     website: &str,
// }

// impl Default for PersonalInfoLayout {
//     fn default() -> Self {
//         Self {
//             name: Some('…'),
//             address: Some('…'),
//             company: Some('…'),
//             city: Some('…'),
//             postal_code: Some('…'),
//             country: Some('…'),
//             telephone: 2183244356,
//             email: Some('…'),
//             website: Some('…'),
//         }
//     }
// }
