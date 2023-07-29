use crate::exchange::{
    binance::{
        models::ApiParams,
        //klines::{klines, remote_to_file},
        klines,
    },
    coinbase::{candles}
};

use std::collections::HashMap;
use std::error::Error;
use std::io;
use serde::{Deserialize, Serialize, };

/// Represents time intervals in seconds.
///
/// This enumeration includes intervals from one second (`S1`) up to one week (`W1`).
/// The values are represented in seconds. For instance, `M1` represents one minute, which is 60 seconds.
#[derive(Debug, Clone, Copy)]
pub enum Intervals {
    S1  = 1,
    M1  = 60,
    M3  = 180,
    M5  = 300,
    M15 = 900,
    M30 = 1800,
    H1  = 3600,
    H2  = 7200,
    H4  = 14400,
    H6  = 21600,
    H8  = 28800,
    H12 = 43200,
    D1  = 86400,
    D3  = 259200,
    W1  = 604800,
}

impl Intervals {
    /// Returns the value of the time interval in seconds.
    /// # Example
    /// ```
    /// use crate::oscillatorsetups::exchange::chart_data;
    /// let interval = chart_data::klines::Intervals::M1;
    /// assert_eq!(interval.value(), 60);
    /// ```
    pub fn value(&self) -> u32 {
        *self as u32
    }

    /// Returns the time interval as a string.
    ///
    /// The returned string includes the value and the unit of time,
    /// such as "1s" for one second, "1m" for one minute, and so on.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::oscillatorsetups::exchange::chart_data;
    /// let interval = chart_data::klines::Intervals::H1;
    /// assert_eq!(interval.as_string(), "1h".to_string());
    /// ```
    pub fn as_string(&self) -> String {
        match *self {
            Intervals::S1   => "1s".to_string(),
            Intervals::M1   => "1m".to_string(),
            Intervals::M3   => "3m".to_string(),
            Intervals::M5   => "5m".to_string(),
            Intervals::M15  => "15m".to_string(),
            Intervals::M30  => "30m".to_string(),
            Intervals::H1   => "1h".to_string(),
            Intervals::H2   => "2h".to_string(),
            Intervals::H4   => "4h".to_string(),
            Intervals::H6   => "6h".to_string(),
            Intervals::H8   => "8h".to_string(),
            Intervals::H12  => "12h".to_string(),
            Intervals::D1   => "1d".to_string(),
            Intervals::D3   => "3d".to_string(),
            Intervals::W1   => "1w".to_string(),
        }
    }
}

/// Represents the parameters needed to make a request to obtain K-Lines data.
/// # Fields
/// * `base_asset`  - First currency in the trading pair.
/// * `quote_asset` - Second currency in the trading pair.
/// * `interval`    - The desired time interval for the K-Lines. See [`Intervals`] for possible values.
/// * `limit`       - The desired number of K-Lines to retrieve.
/// * `base_url`    - The base URL of the exchange API. Defaults to Binance (https://api.binance.us) or Coinbase (https://api.exchange.coinbase.com).
/// * `source`      - The desired source of the K-Lines data. Can be `"api"` to make a request to the exchange, or `"file"` to load the data from a file. Default `"api"`
///
/// If `source` is `"file"`, the program will attempt to load the data from a file. If the data is not available, it will make a request to the exchange and save the retrieved data to a file for future use.
///
/// # Examples
/// ```
/// use oscillatorsetups::exchange::chart_data::klines::{KlineParams,Intervals,};
/// let params = KlineParams {
///     base_asset  : "ETH",
///     quote_asset : "USD",
///     interval    : Intervals::H1,
///     limit       : 1000,
///     base_url    : Some("https://api.binance.us"),
///     source      : Some("api"),
/// };
/// ```
/// This example creates a `KlineParams` instance to request the last 1000 hourly K-Lines for the ETH/USDT trading pair from the Binance.US API.
#[derive(Debug, Clone)]
pub struct KlineParams<'a> {
    pub base_asset  : &'a str,
    pub quote_asset : &'a str,
    pub interval    : Intervals,
    pub limit       : u16,
    pub base_url    : Option<&'a str>,
    pub source      : Option<&'a str>
}
impl<'a> KlineParams<'a> {
    /// Returns source of the K-Lines data. Default `"api"`
    fn get_source(&self) -> &str {
        match &self.source {
            Some(source) => source,
            None => "api",
        }
    }
    /// Returns interval string representation. See [`Intervals::as_string()`]
    fn get_interval(&self)  -> String { self.interval.as_string() }
}
/// A subset of a K-line (candlestick) data from the Exchange API.
///
/// This structure contains the key data points of a K-line, including
/// the opening price, lowest price, highest price, closing price, closing time, and volume.
///
/// # Fields
/// * `time_open`   - The open price
/// * `price_open`  - The opening price of the asset.
/// * `price_low`   - The lowest price of the asset during the interval.
/// * `price_high`  - The highest price of the asset during the interval.
/// * `price_close` - The closing price of the asset.
/// * `close_time`  - The closing time of the interval in milliseconds since the Unix epoch.
/// * `volume`      - The volume of the asset traded during the interval.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KlinesSubset {
    pub time_open   : u64,
    pub price_open  : f64,
    pub price_low   : f64,
    pub price_high  : f64,
    pub price_close : f64,
    pub time_close  : u64,
    pub volume      : f64,
}


/// Fetches K-lines data from the Binance API.
///
/// This function takes a [KlineParams] object as input and returns a vector of [KlinesSubset] objects.
/// Each [KlinesSubset] object represents a single K-line from the Binance API.
///
/// The actual fetching of the K-lines data is performed by calling the [`klines`].
/// The choice between fetching the data from the API or from a file is determined by the `source` field of the `kline_params` argument.
///
/// # Arguments
/// * `kline_params` - A [KlineParams] object that specifies the parameters of the request.
///
/// # Returns
/// A `Result` containing either a `Vec<KlinesSubset>` if the request was successful, or a `Box<dyn Error>` if the request failed.
///
/// # Example
///
/// ```rust
/// use crate::oscillatorsetups::exchange::chart_data::klines::{KlineParams,Intervals,binance,};
///
/// let klines = binance(KlineParams {
///     base_asset  : "ETH",
///     quote_asset : "USD",
///     interval    : Intervals::M15,
///     limit       : 10,
///     base_url    : Some("https://api.binance.us"),
///     source      : Some("api"),
/// });
/// match klines {
///     Ok(data) => println!("Received {} K-lines.", data.len()),
///     Err(e) =>   println!("Error: {}", e),
/// }
/// ```
#[allow(dead_code)]
pub fn binance(kline_params: KlineParams) -> Result<Vec<KlinesSubset>, Box<dyn Error>> {
    let base_url= kline_params.base_url.unwrap_or("https://api.binance.us");

    let interval= kline_params.get_interval();
    let limit = (kline_params.limit + 1).to_string(); // increasing limit, so we can remove latest
    let symbol = format!("{}{}", kline_params.base_asset, kline_params.quote_asset);

    let params = HashMap::from([
        ("interval" , interval.as_str()),
        ("limit"    , limit.as_str()),
        ("symbol"   , symbol.as_str())
    ]);

    let api_params = ApiParams { base_url, endpoint: "/api/v3/klines", params: &params, };

    let klines_res = klines::klines(kline_params.get_source(), api_params)
        .or_else(|error| match error.downcast_ref::<io::Error>() {
            Some(io_error) if io_error.kind() == io::ErrorKind::NotFound => {
                println!("File not found. Pulling data from remote");
                klines::remote_to_file(api_params)
            },
            _ => Err(error),
        })?;

    let mut klines_data = klines_res;
    klines_data.pop(); // removing last tik index, since tik hasn't yet completed

    let kline_subset:Vec<KlinesSubset> = klines_data.into_iter().map(| kline| KlinesSubset {
        time_open   : kline.open_time,
        price_open  : kline.open_price,
        price_low   : kline.low_price,
        price_high  : kline.high_price,
        price_close : kline.close_price,
        time_close  : kline.close_time,
        volume      : kline.volume,

    }).collect();

    Ok(kline_subset)

}

/// Fetches K-lines data from the Coinbase API.
///
/// This function takes a [KlineParams] object as input and returns a vector of [KlinesSubset] objects.
/// Each [KlinesSubset] object represents a single K-line from the Binance API.
///
/// The actual fetching of the K-lines data is performed by calling the [`klines`].
/// The choice between fetching the data from the API or from a file is determined by the `source` field of the `kline_params` argument.
///
/// # Arguments
/// * `kline_params` - A [KlineParams] object that specifies the parameters of the request.
///
/// # Returns
/// A `Result` containing either a `Vec<KlinesSubset>` if the request was successful, or a `Box<dyn Error>` if the request failed.
///
/// # Example
///
/// ```rust
/// use crate::oscillatorsetups::exchange::chart_data::klines::{KlineParams,Intervals,coinbase,};
///
/// let klines = coinbase(KlineParams {
///     base_asset  : "ETH",
///     quote_asset : "USD",
///     interval    : Intervals::M15,
///     limit       : 10,
///     base_url    : Some("https://api.exchange.coinbase.com"),
///     source      : Some("api"),
/// });
/// match klines {
///     Ok(data) => println!("Received {} K-lines.", data.len()),
///     Err(e) =>   println!("Error: {}", e),
/// }
/// ```
pub fn coinbase(kline_params: KlineParams) -> Result<Vec<KlinesSubset>, Box<dyn Error>> {
    let source = kline_params.get_source();

    let base_url= kline_params.base_url.unwrap_or("https://api.exchange.coinbase.com");

    let granularity = kline_params.interval.value();
    let limit = kline_params.limit + 1;
    let symbol = format!("{}-{}",kline_params.base_asset, kline_params.quote_asset);

    let klines_res = candles::candles(source, base_url, granularity, limit, &symbol)
        .or_else(|error| match error.downcast_ref::<io::Error>() {
            Some(io_error) if io_error.kind() == io::ErrorKind::NotFound => {
                println!("{}", io_error);
                println!("File not found. Pulling data from remote");
                candles::remote_to_file(&base_url, granularity, limit, &symbol)
            },
            _ => Err(error),
        })?;

    let mut klines_data = klines_res;
    klines_data.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    klines_data.pop(); // removing last tik index, since tik hasn't yet completed

    let time_stamp_offset = granularity as u64 * 1000;
    let kline_subset:Vec<KlinesSubset> = klines_data.into_iter().map(| kline| KlinesSubset {
        time_open   : kline.timestamp * 1000,
        price_open  : kline.price_open,
        price_low   : kline.price_low,
        price_high  : kline.price_high,
        price_close : kline.price_close,
        time_close  : kline.timestamp * 1000 + time_stamp_offset - 1,
        volume      : kline.volume,
    }).collect();

    Ok(kline_subset)
}
