use reqwest;
use std::error::Error;
use std::collections::HashMap;
use serde_json::{Value,from_str};
use std::fs::File;
use std::io::{Write, Read};

use crate::binance_api::models::{Klines};

const API_END_POINT: [&str; 2] = ["https://api.binance.com", "https://api.binance.us"];
const FILE_JSON:&str = "data.json";

/// Represents the parameters to make a request to the Binance API
///
/// # Examples
///
/// ```rust
/// use oscillatorsetups::binance_api::klines::ApiParams;
///
/// let params = ApiParams {
///     limit: 100,
///     trading_pair: "ETHUSD".to_string(),
///     interval: "15m".to_string(),
///     endpoint: 1,
/// };
/// ```
/// ...
#[derive(Debug)]
pub struct ApiParams {
    pub limit: u16,
    pub trading_pair: String,
    pub interval: String,
    pub endpoint: usize,
}


/// Makes a request to the Binance API and returns the response from a given URL as a `reqwest::Response`.
///
/// The `reqwest::Response` object includes:
/// - `url: reqwest::Url`: The URL of the requested resource.
/// - `status: reqwest::StatusCode`: The status code returned by the server.
/// - `headers: reqwest::header::HeaderMap`: A map of response headers.
///
/// Please refer to the [`reqwest::Response`] documentation for more details.
///
/// # Arguments
///
/// * `params` - The parameters for the API request
///
/// ['reqwest::Response'](https://docs.rs/reqwest/latest/reqwest/struct.Response.html)
async fn get_api_response(params: &ApiParams) -> Result<reqwest::Response, Box<dyn Error>> {
    let url = format!(
        "{}/api/v3/klines?interval={}&limit={}&symbol={}",
        API_END_POINT[params.endpoint], &params.interval, params.limit, &params.trading_pair
    );

    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        /*let headers = response.headers();
        println!("x-mbx-used-weight: {:?}", headers.get("x-mbx-used-weight"));
        println!("x-mbx-used-weight-1m: {:?}", headers.get("x-mbx-used-weight-1m"));*/
        Ok(response)
    } else {
        let error_body = response.json::<HashMap<String, Value>>().await?;
        let code = error_body.get("code").and_then(Value::as_i64);
        let msg = error_body.get("msg").and_then(Value::as_str);
        Err(format!("Error: code {:?}, message {:?}", code, msg).into())
    }
}

/// Makes a request to the Binance API and returns the response as Klines objects
///
/// # Arguments
///
/// * `params` - The parameters for the API request
///
/// # Examples
///
/// ```no_run
/// # use oscillatorsetups::binance_api::klines::{ApiParams,remote};
/// # async {
/// let params = ApiParams {
///     limit: 100,
///     trading_pair: "ETHUSD".to_string(),
///     interval: "1m".to_string(),
///     endpoint: 1,
/// };
/// let klines = remote(&params).await.unwrap();
/// # };
/// ```
/// ...
pub async fn remote(params: &ApiParams) -> Result<Vec<Klines>, Box<dyn Error>> {
    let response = get_api_response(params).await?;
    let klines: Vec<Klines> = response.json().await?;
    Ok(klines)
}

/// Makes a request to the Binance API and saves the response to a file
///
/// # Arguments
///
/// * `params` - The parameters for the API request
///
/// # Examples
///
/// ```no_run
/// # use oscillatorsetups::binance_api::klines::{ApiParams,remote_to_file};
/// # async {
/// let params = ApiParams {
///     limit: 100,
///     trading_pair: "ETHUSD".to_string(),
///     interval: "15m".to_string(),
///     endpoint: 1,
/// };
/// remote_to_file(&params).await.unwrap();
/// # };
/// ```
/// ...
#[allow(dead_code)]
pub async fn remote_to_file(params: &ApiParams) -> Result<(), Box<dyn Error>> {
    let response = get_api_response(params).await?;
    let klines: String = response.text().await?;

    let mut file = File::create(FILE_JSON)?;
    file.write_all(klines.as_bytes())?;

    Ok(())
}

/// Fetches the data from the specified source and returns as Klines objects
///
/// # Arguments
///
/// * `source` - The source from where the data should be fetched. It can be either "api" or "file"
/// * `params` - The parameters for the API request
///
/// # Examples
///
/// ```no_run
/// # use oscillatorsetups::binance_api::klines::{ApiParams,get_data_from_source};
/// # async {
/// let params = ApiParams {
///     limit: 100,
///     trading_pair: "ETHUSDT".to_string(),
///     interval: "15m".to_string(),
///     endpoint: 1,
/// };
/// let klines = get_data_from_source("api", &params).await.unwrap();
/// # };
/// ```
/// ...
#[allow(dead_code)]
pub async fn get_data_from_source(source: &str, params: &ApiParams) -> Result<Vec<Klines>, Box<dyn Error>> {
    match source {
        "api" => {
            remote(params).await
        },
        "file" => {
            let mut file = File::open(FILE_JSON)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let klines: Vec<Klines> = from_str(&contents)?;
            Ok(klines)
        },
        _ => Err("Invalid source".into())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    // Prepare a shared `ApiParams` for the tests
    fn get_test_params() -> ApiParams {
        ApiParams {
            limit: 10,
            trading_pair: "ETHUSD".to_string(),
            interval: "15m".to_string(),
            endpoint: 1,
        }
    }

    #[tokio::test]
    async fn test_get_api_response(){
        let params = get_test_params();

        let response = get_api_response(&params).await;
        assert!(response.is_ok());
    }
}
