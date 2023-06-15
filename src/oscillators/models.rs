/// Represents a single tik price data.
/// Each instance has a combination of high, low, and close prices.
///
/// # Fields
/// - `price_high`: The highest price reached during the period of the kline/candlestick.
/// - `price_low`:  The lowest price reached during the period of the kline/candlestick.
/// - `price_close`:The price at the closing of the kline/candlestick.
///
/// # Examples
///
/// ```
/// use oscillatorsetups::oscillators::models::Hlc;
///
/// let tick = Hlc {
///     price_high: 1792.95,
///     price_low: 1764.02,
///     price_close: 1778.47,
/// };
/// ```
#[derive(Debug)]
pub struct Hlc {
    pub price_high: f64,
    pub price_low: f64,
    pub price_close: f64,
}
