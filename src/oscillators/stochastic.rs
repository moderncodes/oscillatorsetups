use crate::oscillators::{
    models::Hlc,
    sma::{sma_for_tick, sma_for_ticks},
};

/// Calculates the raw stochastic value (%K) for a single tick.
///
/// # Arguments
/// * `price_data` - A slice of `Hlc` structs representing the price data.
/// * `index` - The index of the tick for which to calculate the %K.
/// * `k_length` - The lookback period length over which to calculate the %K.
///
/// # Returns
/// An `Option<f64>` containing the calculated %K if it can be determined,
/// `None` if there is insufficient data.
///
/// # Examples
///
/// ```
/// use oscillatorsetups::oscillators::stochastic::k_for_tick;
/// use oscillatorsetups::oscillators::models::Hlc;
///
/// let price_data = vec![
///     Hlc { price_high: 1.0, price_low: 0.9, price_close: 0.95 },
///     Hlc { price_high: 1.1, price_low: 1.0, price_close: 1.05 },
///     Hlc { price_high: 1.2, price_low: 1.1, price_close: 1.15 },
/// ];
///
/// assert_eq!(k_for_tick(&price_data, 2, 3), Some(83.33333333333331));
/// ```
pub fn k_for_tick(price_data: &[Hlc], index: usize, k_length: u16) -> Option<f64> {
    if index < k_length as usize - 1 {
        None
    } else {
        let low_prices: Vec<f64> = price_data[index + 1 - k_length as usize..=index]
            .iter()
            .map(|hlc| hlc.price_low)
            .collect();

        let high_prices: Vec<f64> = price_data[index + 1 - k_length as usize..=index]
            .iter()
            .map(|hlc| hlc.price_high)
            .collect();
        let close_price = price_data[index].price_close;

        let low: Option<&f64> = low_prices.iter().min_by(|a, b| a.partial_cmp(b).unwrap());
        let high: Option<&f64> = high_prices.iter().max_by(|a, b| a.partial_cmp(b).unwrap());

        if let (Some(low_val), Some(high_val)) = (low, high) {
            if high_val - low_val != 0.0 {
                Some(100.0 * (close_price - low_val) / (high_val - low_val))
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// Calculates the raw stochastic value (%K) for a slice of price data.
///
/// # Arguments
/// * `price_data` - A slice of `Hlc` structs representing the price data.
/// * `k_length` - The lookback period length over which to calculate the %K.
///
/// # Returns
/// A vector of `Option<f64>`, each containing a calculated %K for corresponding tick if it can be determined,
/// `None` if there was insufficient data for that tick.
///
/// # Examples
///
/// ```
/// use oscillatorsetups::oscillators::{
///     stochastic::k_for_ticks,
///     models::Hlc
/// };
///
/// let price_data = vec![
///     Hlc { price_high: 1.0, price_low: 0.9, price_close: 0.95 },
///     Hlc { price_high: 1.1, price_low: 1.0, price_close: 1.05 },
///     Hlc { price_high: 1.2, price_low: 1.1, price_close: 1.15 },
/// ];
///
/// assert_eq!(k_for_ticks(&price_data, 3), vec![None, None, Some(83.33333333333331)]);
/// ```
pub fn k_for_ticks(price_data: &[Hlc], k_length: u16) -> Vec<Option<f64>> {
    let result = price_data
        .iter()
        .enumerate()
        .map(|(i, ..)| k_for_tick(price_data, i, k_length))
        .collect();
    result
}

/// Calculates the Simple Moving Average (SMA) of %K values for a single tick.
///
/// # Arguments
/// * `k_values` - A slice of `Option<f64>` representing the %K values.
/// * `index` - The index of the tick for which to calculate the SMA.
/// * `d_length` - The period length over which to calculate the SMA.
///
/// # Returns
/// An `Option<f64>` containing the calculated SMA if it can be determined.
/// Returns `None` if there is insufficient data to calculate the SMA.
///
/// # Examples
/// ```
/// use oscillatorsetups::oscillators::stochastic::d_for_tick;
///
/// let k_values = vec![Some(10.0), Some(20.0), Some(30.0), Some(40.0)];
///
/// assert_eq!(d_for_tick(&k_values, 3, 3), Some(30.0));
/// ```
pub fn d_for_tick(k_values: &[Option<f64>], index: usize, d_length: u16) -> Option<f64> {
    if index < d_length as usize - 1 {
        None
    } else {
        sma_for_tick(k_values, index, d_length)
    }
}

/// Calculates the Simple Moving Average (SMA) of %K values for a slice of price data.
/// # Arguments
/// * `k_values` - A slice of `Option<f64>` representing the %K values.
/// * `d_length` - The period length over which to calculate the SMA.
///
/// # Returns
/// A `Vec<Option<f64>>` where each element is the SMA of the `d_length` elements in `k_values` preceding it,
/// or `None` if there's not enough preceding data to compute an SMA.
///
/// # Examples
/// ```
/// use oscillatorsetups::oscillators::stochastic::d_for_ticks;
///
/// let k_values = vec![Some(10.0), Some(20.0), Some(30.0), Some(40.0), Some(50.0), Some(60.0)];
/// let d_length = 3;
/// let d_values = d_for_ticks(&k_values, d_length);
///
/// assert_eq!(d_values, vec![None, None, Some(20.0), Some(30.0), Some(40.0), Some(50.0)]);
/// ```
pub fn d_for_ticks(k_values: &[Option<f64>], d_length: u16) -> Vec<Option<f64>> {
    let result = k_values
        .iter()
        .enumerate()
        .map(|(i, ..)| d_for_tick(k_values, i, d_length))
        .collect();

    result
}

/// Represents the Stochastic Oscillator values at a single tick.
///
/// # Examples
/// ```
/// use oscillatorsetups::oscillators::stochastic::StochValues;
///
/// let stoch = StochValues {
///     k_line: Some(80.0),
///     d_line: Some(70.0),
/// };
/// ```
#[derive(PartialEq, Debug)]
pub struct StochValues {
    pub k_line: Option<f64>,
    pub d_line: Option<f64>,
}

/// Generates the Stochastic Oscillator values for a slice of price data.
///
/// # Arguments
/// * `price_data` - A slice of `Hlc` representing the price data.
/// * `k_length` - The lookback period length over which to calculate the raw %K.
/// * `k_smoothing` - The period length over which to smooth the raw %K values.
/// * `d_smoothing` - The period length over which to smooth the %D values.
///
/// # Returns
/// A vector of [StochValues], each representing the Stochastic Oscillator values at a corresponding tick.
///
/// # Examples
/// ```
/// use crate::oscillatorsetups::oscillators::{models::Hlc, stochastic::{stochastic, StochValues}};
///
/// let price_data = vec![
///     Hlc::new(1768.34, 1763.93, 1768.34),
///     Hlc::new(1769.47, 1767.37, 1769.00),
///     Hlc::new(1768.99, 1767.99, 1767.99),
///     Hlc::new(1769.46, 1767.99, 1768.11),
///     Hlc::new(1768.49, 1764.74, 1766.35),
///     Hlc::new(1766.99, 1764.22, 1765.24),
///     Hlc::new(1766.49, 1764.30, 1765.40),
///     Hlc::new(1765.43, 1763.26, 1764.61),
///     Hlc::new(1767.02, 1764.85, 1765.11),
///     Hlc::new(1767.02, 1764.05, 1766.90),
///     Hlc::new(1766.97, 1763.61, 1764.50),
///     Hlc::new(1765.28, 1762.07, 1763.58),
///     Hlc::new(1763.44, 1761.71, 1761.90),
///     Hlc::new(1763.49, 1760.01, 1763.49),
///     Hlc::new(1765.00, 1761.00, 1765.00),
///     Hlc::new(1763.96, 1760.40, 1763.91),
/// ];
/// let k_length = 14;
/// let k_smoothing = 1;
/// let d_smoothing = 3;
///
/// let stoch_values = stochastic(&price_data, k_length, k_smoothing, d_smoothing);
/// let expected_stoch_values = vec![
///     StochValues { k_line: None, d_line: None },
///     StochValues { k_line: None, d_line: None },
///     StochValues { k_line: None, d_line: None },
///     StochValues { k_line: None, d_line: None },
///     StochValues { k_line: None, d_line: None },
///     StochValues { k_line: None, d_line: None },
///     StochValues { k_line: None, d_line: None },
///     StochValues { k_line: None, d_line: None },
///     StochValues { k_line: None, d_line: None },
///     StochValues { k_line: None, d_line: None },
///     StochValues { k_line: None, d_line: None },
///     StochValues { k_line: None, d_line: None },
///     StochValues { k_line: None, d_line: None },
///
///     StochValues { k_line: Some(36.78646934460893), d_line: None },
///     StochValues { k_line: Some(52.74841437632124), d_line: None },
///
///     StochValues { k_line: Some(41.26984126984203), d_line:Some(43.601574996924064) },
/// ];
/// ```
pub fn stochastic(
    price_data: &[Hlc],
    k_length: u16,
    k_smoothing: u16,
    d_smoothing: u16,
) -> Vec<StochValues> {
    let k_line_raw = k_for_ticks(price_data, k_length);
    let k_line = sma_for_ticks(&k_line_raw, k_smoothing);
    let d_line = d_for_ticks(&k_line, d_smoothing);

    k_line
        .into_iter()
        .zip(d_line.into_iter())
        .map(|(k, d)| StochValues {
            k_line: k,
            d_line: d,
        })
        .collect()
}
