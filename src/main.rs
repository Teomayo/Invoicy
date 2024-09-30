mod components;
mod document;
mod functions;
mod structs;

use eframe::egui::{self, Rect, Response};
use egui::{Style, Vec2};
use functions::*;
use rusqlite::Connection;
use std::path::PathBuf;
use structs::*;

const LOGGER: bool = false;

fn main() {
    egui_logger::builder().init().unwrap();
    let options = eframe::NativeOptions {
        // with_icon causes crashes on application when using 'cargo build' or 'cargo release'
        viewport: egui::ViewportBuilder::default()
            .with_icon(load_icon("support/images/128x128.png"))
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
        self.initialize_application();

        // constantly updating actions
        self.update_estimate_number();
        self.update_file_name();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(2.0);
            ui.style_mut().spacing.button_padding = self.style.spacing.button_padding;

            ui.horizontal(|ui| {
                // template button disabled until ready to be worked on
                if ui
                    .add_enabled(false, egui::Button::new("+ template"))
                    .clicked()
                {
                    println!("{:?}", "template button not yet functional");
                }
                self.upload_logo(ui);
                self.customer_and_contact_buttons(ui);
            });
            self.show_form(ui);

            ui.add_space(2.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // customer and contact selection
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                self.customer_select(ui);
                self.contact_select(ui);
                // table area
            });
            ui.spacing();

            self.table(ui);

            ui.add_space(2.0);
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.add_space(2.0);
            self.progress = 0.0;
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    self.file_namer(ui);
                    self.generate_invoice(ui);
                });
                self.send_report(ui);
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
    fn update_estimate_number(&mut self) {
        let curr_estimate_num = self
            .database_data_vec
            .iter()
            .filter(|x| {
                x.cust_id
                    == self.customers[self.customer_selected]
                        .company
                        .clone()
                        .to_uppercase()
                        .get(0..4)
                        .unwrap()
            })
            .map(|item| item.estimate_number)
            .max();
        if curr_estimate_num == None {
            self.current_row_value.estimate_number = 1
        } else {
            self.current_row_value.estimate_number = curr_estimate_num.unwrap() + 1
        }
    }

    fn initialize_application(&mut self) {
        if !self.initialized {
            self.setup_tables();
            // sets up tables to have one customer and contact as place holders
            // should be phased out prior to v1
            self.add_contact();
            self.add_customer();

            self.totals.push(Total {
                value: 0.0,
                position: (0, 4),
            });

            let contact_log = self.get_contacts();
            let customer_log = self.get_customers();
            let data_log = self.get_data();

            println!("{:?}", contact_log.unwrap());
            println!("{:?}", customer_log.unwrap());
            println!("{:?}", data_log.unwrap());

            self.style.spacing.button_padding = Vec2::splat(5.0); // Set horizontal and vertical margins

            self.file_name = format!(
                "{}-{:?}",
                sanitize_string(&self.customer.company),
                self.current_row_value.estimate_number
            );

            self.initialized = true;
        }
    }
}
#[derive(Debug)]
struct Invoicy {
    max_file_size: usize,
    image_file_path: Option<PathBuf>,
    company_error_contact: Option<String>,
    company_error_customer: Option<String>,
    initialized: bool,
    connection: Connection,
    progress: f32,
    style: Style,
    file_name: String,
    customer_selected: usize,
    contact_selected: usize,
    row_count: usize,
    last_updated_row: usize,
    table_data: Vec<(String, (usize, i32), (Rect, Response))>,
    database_data_vec: Vec<DatabaseData>,
    new_database_data_vec: Vec<DatabaseData>,
    contact: Contact,
    contacts: Vec<Contact>,
    contact_form: bool,
    customer: Customer,
    customers: Vec<Customer>,
    customer_form: bool,
    current_row_value: DatabaseData,
    totals: Vec<Total>,
    grand_total: f64,
}
