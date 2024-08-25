//! You may have to adapt the `FONT_DIRS` and `DEFAULT_FONT_NAME` constants for your system so that
//! these files exist:
//! - `{FONT_DIR}/{DEFAULT_FONT_NAME}-Regular.ttf`
//! - `{FONT_DIR}/{DEFAULT_FONT_NAME}-Bold.ttf`
//! - `{FONT_DIR}/{DEFAULT_FONT_NAME}-Italic.ttf`
//! - `{FONT_DIR}/{DEFAULT_FONT_NAME}-BoldItalic.ttf`
//!
//! These fonts must be metrically identical to the built-in PDF sans-serif font (Helvetica/Arial).

use crate::Rect;
use crate::Response;
use chrono::prelude::*;
use genpdf::elements::TableLayoutRow;
use genpdf::Alignment;
use genpdf::Element as _;
use genpdf::{elements, fonts, style};

use crate::Contact;
use crate::Customer;

pub fn generate_invoice(
    font_dir: &str,
    logo_path: &str,
    file_name: String,
    contact_info: Contact,
    customer_info: Customer,
    table: Vec<(String, (usize, i32), (Rect, Response))>,
    estimate_number: i32,
    grand_total: f64,
) {
    // wasn't sure how to get system name in global variables so doing this for now
    // let account_name: String = whoami::username().to_string();
    // let dir_name: String = format!("/Users/{account_name}/Library/Fonts/").to_string();

    let font_dirs: &[String] = &[font_dir.to_string()];
    let collection: Vec<&str> = (font_dir.split(r"/")).collect::<Vec<&str>>();

    // this part is kind of unnecessary, simplify in the future
    let font_name_index: usize = 1;
    let font_name: &String = &collection[font_name_index].to_string();

    let output_file = file_name;

    let font_dir = font_dirs
        .iter()
        .filter(|path| std::path::Path::new(path).exists())
        .next()
        .expect("Could not find font directory");
    let default_font = fonts::from_files(font_dir, font_name, Some(fonts::Builtin::Helvetica))
        .expect("Failed to load the default font family");

    // fonts loaded
    let mut doc = genpdf::Document::new(default_font);

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    decorator.set_header(|page| {
        let mut layout = elements::LinearLayout::vertical();
        if page > 1 {
            layout.push(
                elements::Paragraph::new(format!("Page {}", page)).aligned(Alignment::Center),
            );
            layout.push(elements::Break::new(1));
        }
        layout.styled(style::Style::new().with_font_size(10))
    });
    doc.set_page_decorator(decorator);

    doc.push(elements::Break::new(1.5));

    // #[cfg(feature = "images")]
    // images::place_image(&mut doc);
    let mut address_table = elements::TableLayout::new(vec![1]);
    address_table.set_cell_decorator(elements::FrameCellDecorator::new(false, false, false));

    address_table
        .row()
        .element(elements::Paragraph::new(contact_info.company).aligned(Alignment::Left))
        .push()
        .expect("Invalid table row");
    address_table
        .row()
        .element(elements::Paragraph::new(contact_info.address).aligned(Alignment::Left))
        .push()
        .expect("Invalid table row");
    address_table
        .row()
        .element(elements::Paragraph::new(contact_info.city).aligned(Alignment::Left))
        .push()
        .expect("Invalid table row");
    address_table
        .row()
        .element(elements::Paragraph::new(contact_info.postal_code).aligned(Alignment::Left))
        .push()
        .expect("Invalid table row");
    address_table
        .row()
        .element(elements::Paragraph::new(contact_info.country).aligned(Alignment::Left))
        .push()
        .expect("Invalid table row");
    address_table
        .row()
        .element(elements::Paragraph::new(contact_info.name).aligned(Alignment::Left))
        .push()
        .expect("Invalid table row");
    address_table
        .row()
        .element(elements::Paragraph::new(contact_info.telephone).aligned(Alignment::Left))
        .push()
        .expect("Invalid table row");
    address_table
        .row()
        .element(elements::Paragraph::new(contact_info.email).aligned(Alignment::Left))
        .push()
        .expect("Invalid table row");
    address_table
        .row()
        .element(elements::Paragraph::new(contact_info.website).aligned(Alignment::Left))
        .push()
        .expect("Invalid table row");

    let mut top_header_table = elements::TableLayout::new(vec![1, 1]);
    top_header_table.set_cell_decorator(elements::FrameCellDecorator::new(false, false, false));
    top_header_table
        .row()
        .element(address_table)
        .element(
            elements::Image::from_path(logo_path)
                .expect("Unable to load image")
                .with_alignment(Alignment::Right),
        )
        .push()
        .expect("Invalid header table");

    doc.push(top_header_table);
    doc.push(elements::Break::new(1.5));

    doc.push(elements::Paragraph::new("FOR").styled(style::Effect::Bold));

    let mut customer_info_table = elements::TableLayout::new(vec![1]);
    customer_info_table.set_cell_decorator(elements::FrameCellDecorator::new(false, false, false));

    customer_info_table
        .row()
        .element(
            elements::Paragraph::new(customer_info.company)
                .aligned(Alignment::Left)
                .styled(style::Effect::Bold),
        )
        .push()
        .expect("Invalid table row");
    customer_info_table
        .row()
        .element(
            elements::Paragraph::new(customer_info.address)
                .aligned(Alignment::Left)
                .styled(style::Effect::Bold),
        )
        .push()
        .expect("Invalid table row");
    customer_info_table
        .row()
        .element(
            elements::Paragraph::new(format!(
                "{}, {}",
                customer_info.city, customer_info.postal_code
            ))
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold),
        )
        .push()
        .expect("Invalid table row");
    customer_info_table
        .row()
        .element(
            elements::Paragraph::new(customer_info.country)
                .aligned(Alignment::Left)
                .styled(style::Effect::Bold),
        )
        .push()
        .expect("Invalid table row");

    let current_date = Local::now();
    let valid_until_date: DateTime<Local> = current_date + chrono::Days::new(7);
    let mut date_table = elements::TableLayout::new(vec![1, 1]);
    /*
    TODO: remove hardcoded estiamte element
    */
    date_table
        .row()
        .element(elements::Paragraph::new("Estimate No.:"))
        .element(elements::Paragraph::new(estimate_number.to_string()).aligned(Alignment::Left))
        .push()
        .expect("Invalid header table");
    date_table
        .row()
        .element(elements::Paragraph::new("Issue Date:"))
        .element(
            elements::Paragraph::new(current_date.format("%B %d, %Y").to_string())
                .aligned(Alignment::Left),
        )
        .push()
        .expect("Invalid header table");
    date_table
        .row()
        .element(elements::Paragraph::new("Valid Until:"))
        .element(
            elements::Paragraph::new(valid_until_date.format("%B %d, %Y").to_string())
                .aligned(Alignment::Left),
        )
        .push()
        .expect("Invalid header table");

    let mut bottom_header_table = elements::TableLayout::new(vec![1, 1]);
    bottom_header_table
        .row()
        .element(customer_info_table)
        .element(date_table)
        .push()
        .expect("Invalid header table");

    doc.push(bottom_header_table);
    doc.push(elements::Break::new(1.5));

    // table length will be dependant variable based on the number of columns necessary

    let mut item_table = elements::TableLayout::new(vec![1; 4]);
    item_table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
    let max_cell = table.iter().max_by_key(|&&(_, y, _)| y).unwrap();
    let max_row = max_cell.1 .0 + 1;
    let max_col = max_cell.1 .1 + 1;
    item_table
        .row()
        .element(
            elements::Paragraph::new("Description")
                .aligned(Alignment::Left)
                .styled(style::Effect::Bold)
                .padded(2),
        )
        .element(
            elements::Paragraph::new("Quantity")
                .aligned(Alignment::Left)
                .styled(style::Effect::Bold)
                .padded(2),
        )
        .element(
            elements::Paragraph::new("Price")
                .aligned(Alignment::Left)
                .styled(style::Effect::Bold)
                .padded(2),
        )
        .element(
            elements::Paragraph::new("Total")
                .aligned(Alignment::Left)
                .styled(style::Effect::Bold)
                .padded(2),
        )
        .push()
        .expect("Invalid header table");
    for i in 0..max_row {
        let mut table_row: TableLayoutRow = item_table.row();
        for j in 0..max_col {
            for item in &table {
                if item.1 == (i, 0) {
                    // this is just the row number so we can skip it
                } else if item.1 == (i, j) {
                    table_row.push_element(
                        elements::Paragraph::new(item.0.clone())
                            .aligned(Alignment::Left)
                            .padded(2),
                    );
                }
            }
        }
        table_row.push().expect("Invalid Row");
    }
    doc.push(item_table);

    doc.push(
        elements::Paragraph::new(format!("Grand Total: ${}", grand_total))
            .styled(style::Effect::Bold),
    );

    doc.render_to_file(output_file)
        .expect("Failed to write output file");
}

// Only import the images if the feature is enabled. This helps verify our handling of feature toggles.
// #[cfg(feature = "images")]
mod images {
    //     use super::*;
    //
    //     // const IMAGE_PATH_JPG: &'static str = "images/farbalogo.jpg";
    //
    //     // pub fn place_image(doc: &mut genpdf::Document) {
    //     //     doc.push(elements::Image::from_path(IMAGE_PATH_JPG).expect("Unable to load image"));
    //     //     doc.push(elements::Break::new(1.5));
    //     // }
}
