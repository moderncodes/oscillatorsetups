use super::{
    fetch,
    models::{ApiParams, Klines}
};

use crate::utils::{data_from_json, data_to_json};
use serde_json::from_str;
use std::error::Error;
use reqwest::Url;

/// Retrieves kline/candlestick data for a specific symbol from the Binance API
/// and saves the data into a JSON file. The name of the file is derived from the
/// "symbol" API parameter converted to lowercase with a .json extension.
///
/// The function operates similarly to [`klines`](klines),
/// but instead of returning the [`Klines`](Klines) data,
/// it saves the data to a file in the _/files/klines/_ directory.
///
/// # Examples
///
/// ```no_run
/// use crate::oscillatorsetups::exchange::binance::{models::ApiParams,klines::remote_to_file};
/// use std::collections::HashMap;
///
/// let api_params = ApiParams {
///     base_url: "https://api.binance.us",
///     endpoint: "/api/v3/klines",
///     params  : &HashMap::from([("interval", "15m"), ("limit", "10"), ("symbol", "ETHUSD")]),
/// };
///
/// let klines = remote_to_file(api_params);
/// ```
#[allow(dead_code)]
pub fn remote_to_file(api_params: ApiParams) -> Result<Vec<Klines>, Box<dyn Error>> {
    let mut folder_path= api_params.base_url.to_string();
    let symbol= api_params.params.get("symbol").ok_or("Symbol parameter not found")?;

    let response = fetch::data(api_params)?;
    let klines = response.text()?;

    let parsed_url = Url::parse(folder_path.as_str()).unwrap();
    if let Some(domain) = parsed_url.domain() {
        folder_path = format!("klines/{}/", domain);
    }
    let klines_res: Vec<Klines> = from_str(&klines)?;

    data_to_json(folder_path.as_str(), symbol, klines.as_str()).expect("TODO: panic message");

    Ok(klines_res)
}

/// Fetches the data from the specified source and returns as Klines objects
///
/// # Arguments
///
/// * `source` - The source from where the data should be fetched. It can be either "api" or "file"
/// * `api_params` - The [ApiParams](ApiParams) parameters for the API request
///
/// # Examples
///
/// ```no_run
/// use crate::oscillatorsetups::exchange::binance::{models::ApiParams,klines::klines};
/// use std::collections::HashMap;
///
/// let api_params = ApiParams {
///     base_url: "https://api.binance.us",
///     endpoint: "/api/v3/klines",
///     params  : &HashMap::from([("interval", "15m"), ("limit", "10"), ("symbol", "ETHUSD")]),
/// };
///
/// let klines = klines("api", api_params);
/// ```
#[allow(dead_code)]
pub fn klines(source: &str, api_params: ApiParams) -> Result<Vec<Klines>, Box<dyn Error>> {
    match source {
        "api" => {
            let response = fetch::data(api_params)?;
            let klines: Vec<Klines> = response.json()?;
            Ok(klines)
        }
        "file" => {
            let mut folder_path= api_params.base_url.to_string();
            let symbol= api_params.params.get("symbol").ok_or("Symbol parameter not found")?;

            let parsed_url = Url::parse(folder_path.as_str()).unwrap();
            if let Some(domain) = parsed_url.domain() {
                folder_path = format!("klines/{}/",domain);
            }

            let data = data_from_json(folder_path.as_str(), symbol)?;
            let klines: Vec<Klines> = from_str(&data)?;
            Ok(klines)
        }
        _ => Err("Invalid source".into()),
    }
}
