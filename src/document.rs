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

use chrono::prelude::*;
use csv::StringRecord;
use genpdf::elements::TableLayoutRow;
use genpdf::Alignment;
use genpdf::Element as _;
use genpdf::{elements, fonts, style};
use std::env;

pub fn generate_document(
    font_dir: &str,
    logo_path: &str,
    personal_info_vector: Vec<StringRecord>,
    customer_info_vector: Vec<StringRecord>,
    item_vector: Vec<StringRecord>,
    estimate_number: i32,
) {
    // wasn't sure how to get system name in global variables so doing this for now
    // let account_name: String = whoami::username().to_string();
    // let dir_name: String = format!("/Users/{account_name}/Library/Fonts/").to_string();
    let font_dirs: &[String] = &[font_dir.to_string()];
    // On windows machine use line below for now

    let collection: Vec<&str> = if cfg!(windows) {
        (font_dir.split(r"\")).collect::<Vec<&str>>()
    } else if cfg!(macos) {
        (font_dir.split(r"\")).collect::<Vec<&str>>()
    } else {
        (font_dir.split(r"\")).collect::<Vec<&str>>()
    };

    let font_index: usize = if cfg!(windows) {
        1
    } else if cfg!(macos) {
        1
    } else {
        1
    };
    let font_name: &String = &collection[font_index].to_string();

    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 1 {
        panic!("Missing argument: output file");
    }
    let output_file = &args[0];

    let font_dir = font_dirs
        .iter()
        .filter(|path| std::path::Path::new(path).exists())
        .next()
        .expect("Could not find font directory");
    let default_font = fonts::from_files(font_dir, font_name, Some(fonts::Builtin::Helvetica))
        .expect("Failed to load the default font family");

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

    for item in personal_info_vector {
        address_table
            .row()
            .element(elements::Paragraph::new(&*item.get(0).unwrap()).aligned(Alignment::Left))
            .push()
            .expect("Invalid table row");
    }

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

    for item in customer_info_vector {
        customer_info_table
            .row()
            .element(
                elements::Paragraph::new(&*item.get(0).unwrap())
                    .aligned(Alignment::Left)
                    .styled(style::Effect::Bold),
            )
            .push()
            .expect("Invalid table row");
    }

    let current_date = Local::now();
    let valid_until_date: DateTime<Local> = current_date + chrono::Days::new(7);
    let mut date_table = elements::TableLayout::new(vec![1, 1]);
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

    let mut item_table = elements::TableLayout::new(vec![1; item_vector[0].len() - 1]);
    item_table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
    for item in item_vector {
        let mut table_row: TableLayoutRow = item_table.row();
        for i in 0..(item.len() - 1) {
            table_row.push_element(
                elements::Paragraph::new(&*item.get(i).unwrap())
                    .aligned(Alignment::Left)
                    .styled(style::Effect::Bold)
                    .padded(2),
            );
        }
        table_row.push().expect("Invalid Row");
    }

    doc.push(item_table);

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
