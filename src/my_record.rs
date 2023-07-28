use std::{collections::HashMap, error::Error};
use dbase::{Record, FieldValue};
use serde_json::json;

#[derive(Debug)]
pub struct MyRecord {
    pub fields: HashMap<String, String>,
}

impl MyRecord {
    pub fn from_record(record: Record, fields: &[String]) -> Result<MyRecord, Box<dyn Error>> {
        let mut map = HashMap::new();
        for field in fields {
            let value = get_field(&record, field)?;
            if let Some(v) = value {
                map.insert(field.clone(), v);
            }
        }
        Ok(MyRecord{
            fields: map,
        })
    }

    pub fn to_values(&self, ordered_keys: &[String]) -> Vec<serde_json::Value> {
        ordered_keys.iter().filter_map(|key| {
            self.fields.get(key).map(|value| json!(value.clone()))
        }).collect()
    }
}

fn get_field(record: &Record, field: &str) -> Result<Option<String>, Box<dyn Error>> {
    match record.get(field) {
        Some(value) => match value {
            FieldValue::Character(s) => Ok(s.as_ref().cloned()),
            FieldValue::Numeric(n) => Ok(n.map(|n| n.to_string())),
            _ => Err(format!("Field {} is of an unsupported type", field).into()),
        },
        None => Err(format!("Field {} not found in record", field).into()),
    }
}


pub fn get_my_records_from_record(records: Vec<Record>, fields: &[String]) -> Result<Vec<MyRecord>, Box<dyn Error>> {
    let mut my_records = Vec::new();
    for record in records {
        let my_record = MyRecord::from_record(record, fields)?;
        my_records.push(my_record);
    }

    Ok(my_records)
}
