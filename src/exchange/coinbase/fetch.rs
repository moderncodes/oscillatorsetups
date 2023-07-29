// 'fetch.rs' provides utility functions for fetching product data from the Coinbase API.
use super::models::ApiParams;
use chrono::{Duration, Utc};
use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue, USER_AGENT};
use reqwest::Url;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration as StdDuration;
use sysinfo::{System, SystemExt};


// Constants for package name and version are fetched from the environment.
const PKG_NAME      : &str = env!("CARGO_PKG_NAME");
const PKG_VERSION   : &str = env!("CARGO_PKG_VERSION");

/// `get_headers` is a utility function which prepares the request headers.
/// Formats the user-agent string, including system information, and inserts it into a new `HeaderMap`.
fn get_headers() -> Result<HeaderMap, InvalidHeaderValue> {
    let mut system = System::new_all();
    system.refresh_all();

    let mut headers = HeaderMap::new();

    let user_agent_name = format!(
        "{}/{} ({} {} +{})",
        PKG_NAME,
        PKG_VERSION,
        system.name().unwrap_or("Unknown OS".to_string()),
        system.os_version().unwrap_or("Unknown Version".to_string()),
        system.host_name().unwrap_or("Unknown Host".to_string())
    );
    let user_agent_value = HeaderValue::from_str(&user_agent_name)?;

    headers.insert(USER_AGENT, user_agent_value);

   Ok(headers)
}


/// Main function to fetch product data from the Coinbase API.
/// The `products` function fetches a vector of product data represented as floating point tuples from the Coinbase API.
/// It calculates the number of chunks to fetch based on the granularity and limit provided.
/// The function then fetches the product data in chunks, with a sleep period of 1 second in between fetches to avoid rate limit issues.
/// The fetched data is truncated to the size of the limit provided.
#[tokio::main]
pub async fn products(api_params:ApiParams) -> Result<Vec<[f64; 6]>, Box<dyn Error>> {
    const KLINE_MAX: i64 = 300;

    let headers = get_headers()?;

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    // URL: host name
    let mut url = Url::parse(api_params.base_url)?;

    // URL: path
    if let Some(product_id) = api_params.product_id {
        url.path_segments_mut().unwrap().push("products").push(product_id);
    }
    if let Some(resource) = api_params.resource {
        url.path_segments_mut().unwrap().push(resource);
    }

    // URL: query
    let mut query_param_static_arr: Vec<String> = api_params.params.as_ref().map_or(Vec::new(), |params| {
        params.iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect()
    });

    query_param_static_arr.push(format!("{}={}", "granularity", api_params.granularity));
    let query_param_static_str = query_param_static_arr.join("&");


    let chunks = (api_params.limit as f64 / KLINE_MAX as f64).ceil() as i32;
    let mut end_time = Utc::now();
    let cp_granularity= api_params.granularity as i64;
    let mut klines: Vec<[f64; 6]> = vec![];
    for _ in 0..chunks {
        let start_time = end_time - Duration::seconds(cp_granularity * KLINE_MAX);

        url.set_query(Some(&query_param_static_str));

        url.query_pairs_mut()
            .append_pair("start", start_time.to_rfc3339().as_str())
            .append_pair("end", end_time.to_rfc3339().as_str());

        let resp: Vec<[f64; 6]> = client.get(url.clone()).send().await?.json().await?;
        klines.extend(resp);

        sleep(StdDuration::from_secs(1)); // Sleep for 1 second to prevent rate limit issues

        end_time = start_time;
    }

    klines.truncate(api_params.limit as usize); // Truncates to exactly kline_count size

    Ok(klines)
}
