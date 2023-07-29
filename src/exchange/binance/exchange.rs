use super::{
    fetch,
    models::ApiParams
};
use crate::utils::{data_from_json, data_to_json,string_to_f64, get_folder_path};
use serde::Deserialize;
use serde_json::from_str;
use std::error::Error;

/// Represents a single filter applied to trading symbol data from the Binance API.
///
/// Filters provide constraints for trading, such as minimum and maximum price and quantity.
/// The filter type is represented as a string (`filter_type`), and can be either "PRICE_FILTER" or "LOT_SIZE".
///
/// When deserializing, `min_price`, `max_price`, `min_qty`, and `max_qty` are parsed from strings to `f64` using the custom [string_to_f64](string_to_f64) function.
///
/// ## Fields
/// - `filter_type`: The type of the filter, either "PRICE_FILTER" or "LOT_SIZE".
/// - `min_price`: The minimum price for the filter. Only relevant for "PRICE_FILTER".
/// - `max_price`: The maximum price for the filter. Only relevant for "PRICE_FILTER".
/// - `min_qty`: The minimum quantity for the filter. Only relevant for "LOT_SIZE".
/// - `max_qty`: The maximum quantity for the filter. Only relevant for "LOT_SIZE".
#[derive(Deserialize, Debug)]
pub struct Filter {
    #[serde(rename = "filterType")]
    filter_type: String,
    #[serde(rename = "minPrice", deserialize_with = "string_to_f64", default)]
    min_price: f64,
    #[serde(rename = "maxPrice", deserialize_with = "string_to_f64", default)]
    max_price: f64,
    #[serde(rename = "minQty", deserialize_with = "string_to_f64", default)]
    min_qty: f64,
    #[serde(rename = "maxQty", deserialize_with = "string_to_f64", default)]
    max_qty: f64,
}

/// An enumeration of trading symbol price filters.
///
/// This enum is used to deserialize the relevant information from the Binance API.
///
/// The `filterType` field from the Binance API response is converted into this enum.
/// The variant names in this enum match the `filterType` values in UPPER_SNAKE_CASE.
///
/// # Examples
///
/// Below is an example of how to deserialize a JSON string into this `TradingSymbolFilters` enum:
///
/// ```ignore
/// use serde_json::json;
/// use oscillatorsetups::exchange::binance::exchange::TradingSymbolFilters;
///
/// let json = json!({
///     "filterType": "PRICE_FILTER",
///     "minPrice": "0.01000000",
///     "maxPrice": "1000000.00000000",
/// });
///
/// let price_filter: TradingSymbolFilters = serde_json::from_value(json).unwrap();
/// match price_filter {
///     TradingSymbolFilters::PriceFilter { min_price, max_price } => {
///         println!("Min price: {}, Max price: {}", min_price, max_price);
///     }
///     _ => {}
/// }
/// ```
#[derive(Deserialize, Debug)]
pub enum TradingSymbolFilters {
    /// A price filter defines the price rules for a symbol.
    #[serde(rename = "PRICE_FILTER")]
    PriceFilter {
        /// The minimum price allowed; disabled on minPrice == 0
        #[serde(rename = "minPrice", deserialize_with = "string_to_f64")]
        min_price: f64,
        /// The maximum price allowed; disabled on maxPrice == 0
        #[serde(rename = "maxPrice", deserialize_with = "string_to_f64")]
        max_price: f64,
    },
    /// A lot size filter defines the quantity rules for a symbol.
    #[serde(rename = "LOT_SIZE")]
    LotSize {
        /// The minimum quantity allowed.
        #[serde(rename = "minQty", deserialize_with = "string_to_f64")]
        min_qty: f64,
        /// The maximum quantity/icebergQty allowed.
        #[serde(rename = "maxQty", deserialize_with = "string_to_f64")]
        max_qty: f64,
    },
}

/// Represents a trading symbol with various attributes and filters from the Binance API.
///
/// Each `TradingSymbol` has an associated symbol string, status, asset information, order types, and a set of filters.
/// The filters are represented as a vector of `Filter` structs, which provide constraints for trading.
///
/// ## Fields
/// - `symbol`:  The symbol as "BASE/QUOTE" (ex. "ETHUSD")
/// - `status`:  The status of the symbol [Enum Definitions(REST)](https://docs.binance.us/#enum-definitions-rest) see **Symbol status (status)**
/// - `base_asset`:  The first currency in the trading pair (ex. "ETH")
/// - `base_asset_precision`: The number of decimal places of the base asset (!it is not the smallest unit that you can trade)
/// - `quote_asset`: The second currency in the trading pair (ex. "USD")
/// - `quote_precision`: The number of decimal places of the quote asset (!it is not the smallest unit that you can trade)
///  - `order_types`: Order types available for this symbol [Enum Definitions(REST)](https://docs.binance.us/#enum-definitions-rest) see **Order types (orderTypes, type)**
/// - `filters`: **Some unexpected Binance API feature** Trading rules for a symbol, used to calculate min|max base/quote asset to trade
#[derive(Deserialize, Debug)]
pub struct TradingSymbol {
    pub symbol: String,
    pub status: String,

    #[serde(rename = "baseAsset")]
    pub base_asset: String,

    #[serde(rename = "baseAssetPrecision")]
    pub base_asset_precision: u64,

    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,

    #[serde(rename = "quotePrecision")]
    pub quote_precision: u64,

    #[serde(rename = "orderTypes")]
    pub order_types: Vec<String>,

    pub filters: Vec<Filter>,
}

impl TradingSymbol {
    pub fn filters(&self) -> Vec<TradingSymbolFilters> {
        self.filters
            .iter()
            .filter_map(|filter| match filter.filter_type.as_str() {
                "PRICE_FILTER" => Some(TradingSymbolFilters::PriceFilter {
                    min_price: filter.min_price,
                    max_price: filter.max_price,
                }),
                "LOT_SIZE" => Some(TradingSymbolFilters::LotSize {
                    min_qty: filter.min_qty,
                    max_qty: filter.max_qty,
                }),
                _ => None, // ignore unknown filters
            })
            .collect()
    }
}

/// Exchange information returned from /api/v3/exchangeInfo --https://docs.binance.us/#price-filter
/// ## Fields
/// - `server_time`: Current server time
/// - `symbols`: List of symbols and their info
#[derive(Deserialize, Debug)]
pub struct ExchangeInfo {
    #[serde(rename = "serverTime")]
    pub server_time: u64,
    pub symbols: Vec<TradingSymbol>,
}

/// Retrieves `/api/v3/exchangeInfo` data from the Binance API and saves the data into a JSON file.
/// The function operates similarly to [`info`](info) with parameter `api` source,
/// but instead of returning the [`ExchangeInfo`](ExchangeInfo) data
/// it saves the data to a file in the `/files/exchangeInfo/data.json` directory.
///
/// # Examples
/// ```no_run
/// use crate::oscillatorsetups::exchange::binance::{models::ApiParams,exchange::remote_to_file,};
/// use std::collections::HashMap;
///
/// let api_params = ApiParams {
///     base_url: "https://api.exchange.coinbase.com",
///     endpoint: "/api/v3/exchangeInfo",
///     params  : &HashMap::from([("symbol", "ETHUSD")]),
/// };
/// remote_to_file(api_params).expect("File was not saved!");
/// ```
#[allow(dead_code)]
pub fn remote_to_file(api_params: ApiParams) -> Result<(), Box<dyn Error>> {
    let response = fetch::data(api_params)?;
    let exchange_info = response.text()?;
    let folder_path= get_folder_path(api_params.base_url, "exchangeInfo");
    data_to_json(folder_path.as_str(), "info", exchange_info.as_str(), )
}

/// Fetches the data from the specified source and returns [ExchangeInfo](ExchangeInfo)
///
/// # Arguments
/// * `source` - The source from where the data should be fetched. It can be either "api" or "file"
/// * `api_params` - The [ApiParams](ApiParams) parameters for the API request
///
/// # Examples
/// ```no_run
/// use crate::oscillatorsetups::exchange::binance::{models::ApiParams, exchange::info,};
/// use std::collections::HashMap;
///
/// let api_params = ApiParams {
///     base_url: "https://api.exchange.coinbase.com",
///     endpoint: "/api/v3/exchangeInfo",
///     params  : &HashMap::from([("symbol", "ETHUSD")]),
/// };
/// let exchange_info = info("api", api_params);
/// println!("{:?}", exchange_info);
/// ```
#[allow(dead_code)]
pub fn info(source: &str, api_params: ApiParams) -> Result<ExchangeInfo, Box<dyn Error>> {
    match source {
        "api" => {
            let response = fetch::data(api_params)?;
            let exchange_info: ExchangeInfo = response.json()?;
            Ok(exchange_info)
        }
        "file" => {
            let folder_path= get_folder_path(api_params.base_url, "exchangeInfo");
            let data = data_from_json(folder_path.as_str(), "info")?;
            let exchange_info: ExchangeInfo = from_str(&data)?;
            Ok(exchange_info)
        }
        _ => Err("Invalid source".into()),
    }
}
