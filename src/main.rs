use serde_derive::{Deserialize};
use std::{error::Error, collections::HashMap};
use dbase::{Record, FieldValue};
use csv::{Writer, StringRecord};
use std::fs;

#[derive(Debug)]
struct MyRecord {
    fields: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
struct Config {
    fields: Vec<String>,
    parse_to_csv: bool,
    from_path: String,
    output_path: String,
}

impl MyRecord {
    fn from_record(record: Record, fields: &[String]) -> Self {
        let mut map = HashMap::new();
        for field in fields {
            let value: Option<String> = get_field(&record, field);
            if let Some(v) = value {
                map.insert(field.clone(), v);
            }
        }
        MyRecord{
            fields: map,
        }
    }
}



fn get_field(record: &Record, field: &str) -> Option<String> {
    match record.get(field) {
        Some(value) => match value {
            FieldValue::Character(s) => s.as_ref().map(|s| s.clone()),
            FieldValue::Numeric(n) => n.map(|n| n.to_string()),
            _ => None,
        },
        None => None,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = read_config()?;
    let mut reader = dbase::Reader::from_path(&config.from_path)?;
    let records = reader.read()?;

    if config.parse_to_csv{
        write_to_csv(&config.output_path, records, &config.fields)?;
    }

    Ok(())
}

fn read_config() -> Result<Config, Box<dyn Error>> {
    let contents = fs::read_to_string("config.toml")?;
    let config = toml::from_str(&contents)?;
    println!("{:?}", config);
    Ok(config)
}

fn write_to_csv(path: &str, records: Vec<Record>, fields: &Vec<String>) -> Result<(), Box<dyn Error>>{
    let mut wtr = Writer::from_path(path)?;

    // Write headers
    wtr.write_record(fields)?;

    for record in records {
        let my_record = MyRecord::from_record(record, &fields);
        let mut string_record = StringRecord::new();
        for field in fields {
            let value = my_record.fields.get(field).unwrap_or(&String::from("")).to_string();
            string_record.push_field(&value);
        }


        wtr.write_record(&string_record)?;
    }

    wtr.flush()?;

    Ok(())
}


