// 'models.rs' defines the data structures used for the Coinbase API requests and responses.

use std::collections::HashMap;
use serde::{Deserialize, Serialize,};

/// The `ApiParams` struct holds the parameters for the Coinbase API request.
///
/// # Examples
/// ```
/// use crate::oscillatorsetups::exchange::coinbase::models::ApiParams;
/// use std::collections::HashMap;
///
/// let params = HashMap::from([("start", "2021-09-14T20:00:00Z"),("end", "2021-09-15T20:00:00Z"),]);
///
/// let api_params = ApiParams {
///     base_url    : "https://api.exchange.coinbase.com",
///     product_id  : Some("ETH-USD"),
///     resource    : Some("candles"),
///     params      : Some(params),
///     limit       : 300,
///     granularity : 3600,
/// };
/// ```
#[derive(Debug)]
pub struct ApiParams<'a> {
    /// Coinbase API hostname.
    pub base_url    : &'a str,
    /// Trading pair as ETH-USD
    pub product_id  : Option<&'a str>,
    pub resource    : Option<&'a str>,
    /// URL query params
    pub params: Option<HashMap<&'a str, &'a str>>,
    pub limit: u16,
    pub granularity: u32
}

/// The `Klines` data structure for a single kline (candlestick) data point from the Coinbase API.
/// Includes the timestamp, open, high, low, close price, and the volume.
/// Each field is annotated with serde to rename the fields when deserializing from the JSON response.
///
/// # Examples
///
/// ```
/// use crate::oscillatorsetups::exchange::coinbase::models::Klines;
/// use serde_json::json;
///
/// let kline_data = json!({
///     "0": 1633728000,
///     "1": 0.01891,
///     "2": 0.01897,
///     "3": 0.01892,
///     "4": 0.01895,
///     "5": 92.051
/// });
///
/// let kline: Klines = serde_json::from_value(kline_data).unwrap();
/// ```
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Klines {
    #[serde(rename = "0")]
    pub timestamp: u64,

    #[serde(rename = "1")]
    pub price_low: f64,

    #[serde(rename = "2")]
    pub price_high: f64,

    #[serde(rename = "3")]
    pub price_open: f64,

    #[serde(rename = "4")]
    pub price_close: f64,

    #[serde(rename = "5")]
    pub volume: f64,
}