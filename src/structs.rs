use eframe::egui::{Rect, Response, Style};
use rusqlite::Connection;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Invoicy {
    pub max_file_size: usize,
    pub image_file_path: Option<PathBuf>,
    pub company_error_contact: Option<String>,
    pub company_error_customer: Option<String>,
    pub initialized: bool,
    pub connection: Connection,
    pub progress: f32,
    pub style: Style,
    pub file_name: String,
    pub customer_selected: usize,
    pub contact_selected: usize,
    pub row_count: usize,
    pub last_updated_row: usize,
    pub table_data: Vec<(String, (usize, i32), (Rect, Response))>,
    pub database_data_vec: Vec<DatabaseData>,
    pub new_database_data_vec: Vec<DatabaseData>,
    pub contact: Contact,
    pub contacts: Vec<Contact>,
    pub contact_form: bool,
    pub customer: Customer,
    pub customers: Vec<Customer>,
    pub customer_form: bool,
    pub current_row_value: DatabaseData,
    pub totals: Vec<Total>,
    pub grand_total: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Total {
    pub value: f64,
    pub position: (usize, i32),
}
#[derive(Clone, Debug, PartialEq)]
pub struct Customer {
    pub company: String,
    pub address: String,
    pub city: String,
    pub postal_code: String,
    pub country: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DatabaseData {
    pub entry_id: String,
    pub estimate_number: usize,
    pub cust_id: String,
    pub row_number: usize,
    pub description: String,
    pub quantity: f64,
    pub price: f64,
    pub total: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Contact {
    pub company: String,
    pub address: String,
    pub city: String,
    pub postal_code: String,
    pub country: String,
    pub name: String,
    pub telephone: String,
    pub email: String,
    pub website: String,
}
