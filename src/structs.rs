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
