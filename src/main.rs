use std::error::Error;
use exporters::csv_exporter::*;
use exporters::google_sheets_exporter::*;
use my_record::*;

mod exporters;
mod config;
mod my_record;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = config::read_config()?;
    let mut reader = dbase::Reader::from_path(&config.from_path)?;
    let records = reader.read()?;
    let my_records = get_my_records_from_record(records, &config.fields)?;

    if config.parse_to_csv{
        write_to_csv(&config.output_path, &my_records, &config.fields)?;
    }

    if config.upload_to_google_sheets{
        upload_to_google_sheets(&config.google_sheet_id, &my_records, &config.fields).await?;
    }

    Ok(())
}

