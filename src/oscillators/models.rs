/// Single tik price data.
/// Each instance has a combination of high, low, and close prices.
///
/// # Fields
/// - `price_high`: The highest price reached during the period of the kline/candlestick.
/// - `price_low`:  The lowest price reached during the period of the kline/candlestick.
/// - `price_close`:The price at the closing of the kline/candlestick.
///
/// See [Hlc::new] for creating a new instance of `Hlc`.
#[derive(Debug)]
pub struct Hlc {
    pub price_high: f64,
    pub price_low: f64,
    pub price_close: f64,
}

impl Hlc {
    /// Creates a new instance of (high, low, and close) tik price data
    /// # Examples
    /// ```
    /// use crate::oscillatorsetups::oscillators::models::Hlc;
    ///
    /// let hlc = Hlc::new(1792.95, 1764.02, 1778.47);
    /// ```
    pub fn new(price_high: f64, price_low: f64, price_close: f64) -> Self {
        Self {
            price_high,
            price_low,
            price_close,
        }
    }
}