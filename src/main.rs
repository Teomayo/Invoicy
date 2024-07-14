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

const IMAGE_PATH_JPG: &'static str = r"images\farbalogo.jpg";
const DIR_NAME: &str = r"fonts\JetbrainsMono\";
// const PERSONAL_INFO: Vec;
// const CUSTOMER_INFO: Vec;
// const ITEMS: Vec;

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
}
