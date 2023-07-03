use csv::{Writer, StringRecord};
use std::error::Error;
use crate::my_record::MyRecord;

pub fn write_to_csv(path: &str, records: &Vec<MyRecord>, fields: &Vec<String>) -> Result<(), Box<dyn Error>>{
    let mut wtr = Writer::from_path(path)?;

    // Write headers
    wtr.write_record(fields)?;

    for record in records {
        let mut string_record = StringRecord::new();
        for field in fields {
            let value = record.fields.get(field).unwrap_or(&String::from("")).to_string();
            string_record.push_field(&value);
        }


        wtr.write_record(&string_record)?;
    }

    wtr.flush()?;

    Ok(())
}