use std::error::Error;
use exporters::csv_exporter::write_to_csv;

mod exporters;
mod config;
mod my_record;

fn main() -> Result<(), Box<dyn Error>> {
    let config = config::read_config()?;
    let mut reader = dbase::Reader::from_path(&config.from_path)?;
    let records = reader.read()?;

    if config.parse_to_csv{
        write_to_csv(&config.output_path, records, &config.fields)?;
    }

    Ok(())
}

