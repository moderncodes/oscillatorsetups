use serde::{
    de::{self, Deserializer, Visitor},
    Deserialize, Serialize,
};

//https://docs.rs/serde/latest/serde/de/trait.Visitor.html
struct F64Visitor;
impl<'de> Visitor<'de> for F64Visitor {
    type Value = f64;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string representation of f64")
    }

    fn visit_str<E>(self, value: &str) -> Result<f64, E>
    where
        E: de::Error,
    {
        value.parse::<f64>().map_err(E::custom)
    }
}
fn string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_string(F64Visitor)
}

/// Represents kline/candlestick data for a trading pair on Binance.
///
/// Each `Klines` instance represents a single kline/candlestick.
/// Klines are often used in financial analysis and trading strategies.
/// [Kline/Candlestick Data](https://binance-docs.github.io/apidocs/spot/en/#kline-candlestick-data)
///
/// # Fields
/// - `open_time`:  The time that the kline/candlestick opened, represented as a Unix timestamp.
/// - `open_price`: The price at the opening of the kline/candlestick.
/// - `high_price`: The highest price reached during the period of the kline/candlestick.
/// - `low_price`:  The lowest price reached during the period of the kline/candlestick.
/// - `close_price`:The price at the closing of the kline/candlestick.
/// - `volume`:     The volume traded during the period of the kline/candlestick.
/// - `close_time`: The time that the kline/candlestick closed, represented as a Unix timestamp.
/// - `quote_asset_volume`: The volume of the quote asset traded.
/// - `number_of_trades`:   The number of trades executed during the period of the kline/candlestick.
/// - `taker_buy_base_asset_volume`:    The volume of the base asset bought by takers.
/// - `taker_buy_quote_asset_volume`:   The volume of the quote asset bought by takers.
/// - `unused_field`:   This field is currently unused.
///
/// # Examples
///
/// Deserialization example:
///
/// ```rust
/// use serde_json::from_str;
/// use oscillatorsetups::binance_api::models::Klines;
///
/// let data = r#"
/// [
///   1685668560000,
///   "1862.40000000",
///   "1862.40000000",
///   "1861.64000000",
///   "1861.74000000",
///   "1.66780000",
///   1685668619999,
///   "3105.19194600",
///   10,
///   "0.87900000",
///   "1636.72362600",
///   "0"
/// ]
/// "#;
/// let klines: Klines = from_str(data).unwrap();
/// ```
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Klines {
    #[serde(rename = "0")]
    pub open_time: u64,
    #[serde(rename = "1", deserialize_with = "string_to_f64")]
    pub open_price: f64,
    #[serde(rename = "2", deserialize_with = "string_to_f64")]
    pub high_price: f64,
    #[serde(rename = "3", deserialize_with = "string_to_f64")]
    pub low_price: f64,
    #[serde(rename = "4", deserialize_with = "string_to_f64")]
    pub close_price: f64,
    #[serde(rename = "5", deserialize_with = "string_to_f64")]
    pub volume: f64,
    #[serde(rename = "6")]
    pub close_time: u64,
    #[serde(rename = "7", deserialize_with = "string_to_f64")]
    pub quote_asset_volume: f64,
    #[serde(rename = "8")]
    pub number_of_trades: u32,
    #[serde(rename = "9", deserialize_with = "string_to_f64")]
    pub taker_buy_base_asset_volume: f64,
    #[serde(rename = "10", deserialize_with = "string_to_f64")]
    pub taker_buy_quote_asset_volume: f64,
    #[serde(rename = "11")]
    pub unused_field: String,
}
