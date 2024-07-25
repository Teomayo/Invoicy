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
use eframe::egui;

const IMAGE_PATH_JPG: &'static str = r"images\farbalogo.jpg";
const DIR_NAME: &str = r"fonts\JetbrainsMono\";
// const PERSONAL_INFO: Vec;
// const CUSTOMER_INFO: Vec;
// const ITEMS: Vec;

// find way to differentiate unix paths and windows paths
const PERSONAL_CSV: &str = r"src\csv\personal.csv";
const CUSTOMER_CSV: &str = r"src\csv\customer.csv";
const ITEMS_CSV: &str = r"src\csv\items.csv";
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
struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("My egui Application");
            ui.horizontal(|ui| {
                if ui.button("+ Customer").clicked() {}
                if ui.button("+ Contact").clicked() {}
            });

            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            // ui.image(egui::include_image!(
            //     "../../../crates/egui/assets/ferris.png"
            // ));
        });
    }
}

// pub struct PersonalInfoLayout {
//     name: &str,
//     address: &str,
//     company: &str,
//     city: &str,
//     postal_code: &str,
//     coutnry: &str,
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
//             coutnry: Some('…'),
//             telephone: 2183244356,
//             email: Some('…'),
//             website: Some('…'),
//         }
//     }
// }
