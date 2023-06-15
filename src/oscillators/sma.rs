/// Calculates the Simple Moving Average (SMA) for a given tick.
///
/// # Arguments
/// * `data` - A slice of `Option<f64>` where each `Option<f64>` is a possible price at a given tick.
/// * `index` - The index of the tick for which to calculate the SMA.
/// * `period` - The period length over which to calculate the SMA.
///
/// # Returns
/// An `Option<f64>` containing the calculated SMA if it can be determined.
/// Returns `None` if there is insufficient data to calculate the SMA, or if any data point in the period is `None`.
///
/// # Examples
/// ```
/// use oscillatorsetups::oscillators::sma::sma_for_tick;
/// let data = vec![Some(10.0), Some(20.0), Some(30.0), Some(40.0), Some(50.0), Some(60.0), Some(70.0)];
///
/// assert_eq!(sma_for_tick(&data, 3, 3), Some(30.0));
/// assert_eq!(sma_for_tick(&data, 2, 3), Some(20.0));
/// assert_eq!(sma_for_tick(&data, 1, 3), None);
/// ```
pub fn sma_for_tick(data: &[Option<f64>], index: usize, period: usize) -> Option<f64> {
    if index < period - 1 {
        None
    } else {
        let slice = &data[index + 1 - period..=index];
        if slice.iter().any(|&x| x.is_none()) {
            None
        } else {
            let sum: f64 = slice.iter().filter_map(|&x| x).sum();
            Some(sum / period as f64)
        }
    }
}

/// Calculates the Simple Moving Average (SMA) for a given vector of `Option<f64>` data over a specified period size.
///
/// This function implements O(n) time complexity by maintaining a running sum for the current period,
/// and as it moves forward in the data, it subtracts the first element going out of the period and adds the next element coming in.
///
/// # Arguments
/// * `data` - A vector of `Option<f64>` values for which the SMA should be calculated. Each `Option<f64>` represents a possible price at a given tick.
/// * `period` - An usize that defines the number of elements in the moving period.
///
/// # Returns
/// * `Vec<Option<f64>>` - A vector where each element is an Option that holds the SMA of the `period` elements in `data` preceding it,
///                        or None if there's not enough preceding data to compute an SMA or if any data point in the period is `None`.
///
/// # Examples
/// ```
/// use oscillatorsetups::oscillators::sma::sma_for_ticks;
///
/// let data = vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0), Some(5.0), Some(6.0)];
/// let period = 3;
/// let sma = sma_for_ticks(&data, period);
///
/// assert_eq!(sma, vec![None, None, Some(2.0), Some(3.0), Some(4.0), Some(5.0)]);
/// ```
pub fn sma_for_ticks(data: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let data_len: usize = data.len();
    let mut res = vec![None; data_len];

    for ix in 0..data_len {
        if ix >= period - 1 {
            let slice = &data[ix + 1 - period..=ix];
            // Skip if slice contains None
            if slice.iter().any(|&x| x.is_none()) {
                continue;
            }
            let sum: f64 = slice.iter().filter_map(|&x| x).sum();
            res[ix] = Some(sum / period as f64);
        }
    }
    res
}
