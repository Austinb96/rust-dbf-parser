use google_sheets4 as sheets4;
use hyper::{client::HttpConnector};
use serde_json::json;
use sheets4::{
    api::{ValueRange, ClearValuesRequest},
    oauth2::{self, authenticator::Authenticator},
    Sheets,
    Error
};

use crate::{my_record::MyRecord, config::GSheet};


type HttpClient = Authenticator<sheets4::hyper_rustls::HttpsConnector<HttpConnector>>;

pub async fn upload_to_google_sheets(sheet: &GSheet, records: &Vec<MyRecord>, fields: &Vec<String>) -> Result<(), Error> {
    let hub = get_hub().await?;

    clear_sheet(&hub, &sheet).await?;
    append_sheet(&hub, &sheet, records, fields).await?;

    Ok(())
}

async fn read_service_account_key() -> Result<oauth2::ServiceAccountKey, Error> {
    oauth2::read_service_account_key("serviceaccount.json")
        .await
        .map_err(|_| Error::Io(std::io::Error::new(std::io::ErrorKind::Other, "Failed to read service account key")))
}

async fn create_authenticator(creds:oauth2::ServiceAccountKey) -> Result<HttpClient, Error> {
    let sa = oauth2::ServiceAccountAuthenticator::builder(creds)
        .build()
        .await
        .map_err(|_| Error::Io(std::io::Error::new(std::io::ErrorKind::Other, "Failed to build connection with authenticator")))?;

    Ok(sa)
}

async fn get_hub() -> Result<Sheets<hyper_rustls::HttpsConnector<HttpConnector>>, Error> {
    let creds: oauth2::ServiceAccountKey = read_service_account_key().await?;

    let auth: HttpClient = create_authenticator(creds).await?;

    let hub: Sheets<_> = Sheets::new(hyper::Client::builder()
            .build(hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .enable_http2()
                .build()
            ),
            auth,
    );

    Ok(hub)
}

async fn append_sheet(hub: &Sheets<hyper_rustls::HttpsConnector<HttpConnector>>, sheet: &GSheet, records: &Vec<MyRecord>, fields: &Vec<String>) -> Result<(), Error> {
    let req = my_records_to_value_range(records, fields)?;

    let range = format!("{}!{}", sheet.sheet_name, sheet.range);
    let result = hub.spreadsheets()
        .values_append(req, &sheet.id, &range)
        .value_input_option("USER_ENTERED")
        .doit().await;

    match result {
        Err(e) => {
            println!("{}", e);
            Err(e)
        },
        Ok(_res) => {
            // println!("Success: {:?}", _res);
            Ok(())
        },
    }
}
async fn clear_sheet(hub: &Sheets<hyper_rustls::HttpsConnector<HttpConnector>>, sheet: &GSheet) -> Result<(), Error> {
    let req = ClearValuesRequest::default();

    let range = format!("{}!{}", sheet.sheet_name, sheet.range);
    let result = hub.spreadsheets()
        .values_clear(req, &sheet.id, &range)
        .doit().await;

    match result {
        Err(e) => {
            println!("{}", e);
            Err(e)
        },
        Ok(_res) => {
            // println!("Success: {:?}", _res);
            Ok(())
        },
    }
}

fn my_records_to_value_range(records: &Vec<MyRecord>, ordered_keys: &[String]) -> Result<ValueRange, Error> {
    let mut values = Vec::new();
    let header: Vec<serde_json::Value> = ordered_keys.iter().map(|key| json!(key.clone())).collect();
    values.push(header);

    let data_rows: Vec<Vec<serde_json::Value>> = records.into_iter().map(|record| {
        record.to_values(ordered_keys)
    }).collect();
    values.extend(data_rows);

    let range = ValueRange {
        major_dimension: None,
        range: None,
        values: Some(values),
    };

    Ok(range)
}