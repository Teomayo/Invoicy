use csv::{Error, StringRecord};

pub fn main(
    personal_csv_path: &str,
    customer_csv_path: &str,
    items_csv_path: &str,
) -> Result<(Vec<StringRecord>, Vec<StringRecord>, Vec<StringRecord>), Error> {
    let mut reader = csv::Reader::from_path(personal_csv_path)?;
    let mut personal_info_records: Vec<StringRecord> = vec![];
    for record in reader.records() {
        let record = record?;
        // println!("ITEM {}", &record[0]);
        personal_info_records.push(record);
    }

    let mut reader = csv::Reader::from_path(customer_csv_path)?;
    let mut customer_info_records: Vec<StringRecord> = vec![];
    for record in reader.records() {
        let record = record?;
        // println!("ITEM {}", &record[0]);
        customer_info_records.push(record);
    }

    let mut reader = csv::Reader::from_path(items_csv_path)?;
    let mut item_records: Vec<StringRecord> = vec![];
    item_records.push(reader.headers().unwrap().to_owned());
    for record in reader.records() {
        let record: csv::StringRecord = record?;
        // println!("{:?}", record);
        // println!(
        //     "ITEM {} {} {} {}",
        //     &record[0], &record[1], &record[2], &record[3]
        // );
        item_records.push(record);
    }

    Ok((personal_info_records, customer_info_records, item_records))
}
