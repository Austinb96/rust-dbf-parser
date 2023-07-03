use std::fs;
use std::error::Error;
use serde_derive::Deserialize;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub fields: Vec<String>,
    pub parse_to_csv: bool,
    pub from_path: String,
    pub output_path: String,
    pub upload_to_google_sheets: bool,
    pub google_sheet_id: String,
}

pub fn read_config() -> Result<Config, Box<dyn Error>> {
    let contents = fs::read_to_string("config.toml")?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}