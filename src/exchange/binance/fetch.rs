use reqwest::blocking::Response;
use reqwest::{blocking, Url};
use serde_json::Value;
use std::{collections::HashMap, error::Error};

use super::models::ApiParams;

/* ["https://api.binance.com", "https://api.binance.us"] */
pub fn data(api_params: ApiParams) -> Result<Response, Box<dyn Error>> {
    let mut url = Url::parse(api_params.base_url)?;
    url.set_path(api_params.endpoint);

    for (key, value) in api_params.params {
        url.query_pairs_mut().append_pair(key, value);
    }

    let url_str = url.as_str();

    let resp = blocking::get(url_str)?;

    if resp.status().is_success() {
        let headers = resp.headers();
        println!("x-mbx-used-weight: {:?}", headers.get("x-mbx-used-weight"));
        println!(
            "x-mbx-used-weight-1m: {:?}",
            headers.get("x-mbx-used-weight-1m")
        );

        Ok(resp)
    } else if resp.status() == 404 {
        Err(format!("Resource not found at url: {}", url_str).into())
    } else {
        let error_body = resp.json::<HashMap<String, Value>>()?;
        let code = error_body.get("code").and_then(Value::as_i64);
        let msg = error_body.get("msg").and_then(Value::as_str);

        Err(format!("Error: code {:?}, message {:?}", code, msg).into())
    }
}
