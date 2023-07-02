use std::collections::HashMap;
use dbase::{Record, FieldValue};

#[derive(Debug)]
pub struct MyRecord {
    pub fields: HashMap<String, String>,
}

impl MyRecord {
    pub fn from_record(record: Record, fields: &[String]) -> Self {
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

