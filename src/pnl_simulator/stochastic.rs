use std::{
    cmp::Ordering,
    collections::BTreeSet,
    error::Error,
    ops::RangeInclusive,
    sync::{Arc, Mutex},
};

use crate::exchange::chart_data::klines::{binance, coinbase, KlineParams, KlinesSubset};
use crate::oscillators::{models::Hlc, stochastic::stochastic};
use super::{
    models::{PnL,TriggerSignal},
    pnl::{simulate, SimulateParams}
};

use rayon::prelude::*;

/// `PnlParams` represents the configuration parameters used for Profit and Loss (PnL) simulations
/// when utilizing the stochastic oscillator. The stochastic oscillator is a momentum indicator that
/// uses support and resistance levels. `PnlParams` specifically encapsulates the lengths and smoothing
/// values required for its calculation.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct PnlParams {
    /// `k_length` denotes the number of periods used to calculate the %K value in the stochastic
    /// oscillator. It determines how sensitive the oscillator will be to market movements. A lower
    /// value makes it more sensitive.
    pub k_length    : u16,
    /// `k_smoothing` refers to the moving average period applied to the %K line, further smoothing
    /// out its values. This helps in reducing volatility and noise in the %K line.
    pub k_smoothing : u16,
    /// `d_length` denotes the number of periods used to smooth out the %D line, which is essentially
    /// a moving average of the %K line. This line acts as a signal line for potential trading signals.
    pub d_length    : u16,
}
impl PartialOrd for PnlParams {
    /// Provides a mechanism to compare two [`PnlParams`] based on their individual attributes in a
    /// specific sequence. This ensures that the structure can be sorted or compared to another
    /// structure of its kind.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for PnlParams {
    /// Compares two [`PnlParams`] for ordering. The comparison starts with `k_length`, followed by
    /// `k_smoothing`, and then `d_length`. This ensures a deterministic and consistent ordering
    /// for collections of `PnlParams`.
    fn cmp(&self, other: &Self) -> Ordering {
        self.k_length.cmp(&other.k_length)
            .then_with(|| self.k_smoothing.cmp(&other.k_smoothing))
            .then_with(|| self.d_length.cmp(&other.d_length))
    }
}

/// Defines the range of parameters for the stochastic oscillator used in the PnL simulations.
/// # Example
/// ```
/// use oscillatorsetups::pnl_simulator::stochastic::PnlRange;
///
/// let pnl_range = PnlRange {
///     k_length    : 3..=97,
///     k_smoothing : 3..=97,
///     d_length    : 3..=97,
/// };
///
/// assert_eq!(pnl_range.k_length   , 3..=97);
/// assert_eq!(pnl_range.k_smoothing, 3..=97);
/// assert_eq!(pnl_range.d_length   , 3..=97);
/// ```
#[derive(Debug)]
pub struct PnlRange {
    /// The inclusive range for k_length.
    pub k_length    : RangeInclusive<u16>,
    /// The inclusive range for k_smooth
    pub k_smoothing : RangeInclusive<u16>,
    /// The inclusive range for d_length.
    pub d_length    : RangeInclusive<u16>,
}

/// Represents a stochastic oscillator simulation for a given financial exchange.
/// A stochastic oscillator is a momentum indicator that uses support and resistance levels.
/// It predicts price turning points by comparing the closing price of a security to its price range.
///
/// # Parameters
/// * `exchange`: The name of the exchange to pull data from, e.g., "coinbase" or "binance".
/// * `klines`: Vec<[KlinesSubset]> Subset of K-line data representing certain attributes of the price candle in a time frame.
/// * `lhc`: Vec<[Hlc]> High, Low, Close (HLC) values derived from the K-line data.
/// * `capital`: The starting capital for the simulation.
/// * `exchange_fee`: The fee charged by the exchange for each transaction.
/// * `min_qty`: The minimum quantity of an asset that can be bought or sold.
/// * `min_price`: The minimum price at which an asset can be bought or sold.
/// * `asset_scale`: The precision with which assets are tracked.
/// * `funds_scale`: The precision with which funds are tracked.
/// * `pnl_fast` : TODO: Implement a faster Profit and Loss (PnL) computation method at the cost of precision. (Not yet implemented)
///
/// ## Reference for implements
/// * [`Stochastic::new`] - instance with default and derived values
/// * [`Stochastic::pnl`] - simple one config pnl request
/// * [`Stochastic::top_net_profit`] - computes the top net profits across a range of PnL parameters, then prints the top 100 configurations.
#[derive(Debug)]
pub struct Stochastic<'a> {
    pub exchange    : &'a str,
    pub klines  : Vec<KlinesSubset>,
    pub lhc     : Vec<Hlc>,

    pub capital     : f64,
    pub exchange_fee: Option<f64>,
    pub min_qty     : Option<f64>,
    pub min_price   : Option<f64>,
    pub asset_scale : u32,
    pub funds_scale : u32,

    pub pnl_fast    : bool
}

impl<'a> Stochastic<'a> {
    /// Creates a new instance of the `Stochastic` struct using data from the specified exchange.
    ///
    /// The method fetches K-line data based on the given exchange and then constructs a [`Stochastic`] instance with default and derived values.
    ///
    /// # Default Values
    /// - `capital`: 1000.0; Use [Stochastic::capital] method to set different amount
    /// - `exchange_fee`: None; Use [Stochastic::exchange_fee] method to set fee
    /// - `min_qty`: None; Use [Stochastic::min_qty] to update amount
    /// - `min_price`: None; Use [Stochastic::min_price] to update amount
    /// - `asset_scale`: 8; Use [Stochastic::asset_scale] to change
    /// - `funds_scale`: 8; Use [Stochastic::funds_scale] to change
    /// - `pnl_fast`: false; TODO (Not yet implemented)
    ///
    /// # Parameters
    /// - `exchange`: A string slice that represents the name of the exchange. Only "coinbase" and "binance" are currently supported.
    /// - `params`: An instance of [`KlineParams`] which provides parameters for fetching K-line data.
    ///
    /// # Returns
    /// - A `Result` which is `Ok` if a new [`Stochastic`] instance is successfully created. Returns an error if an invalid exchange name is provided or if there's an issue fetching the K-line data.
    ///
    /// # Examples
    ///
    /// ```
    /// use oscillatorsetups::exchange::chart_data::klines::{Intervals, KlineParams};
    /// use oscillatorsetups::pnl_simulator::stochastic::Stochastic;
    ///
    /// let kline_params = KlineParams {
    ///     base_asset: "ETH",
    ///     quote_asset: "USD",
    ///     interval: Intervals::H4,
    ///     limit: 1000,
    ///     base_url:None,
    ///     source: Some("api"),
    /// };
    ///
    /// let stochastic_instance = Stochastic::new("coinbase", kline_params);
    /// match stochastic_instance {
    ///     Ok(stochastic) => println!("{:?}", stochastic),
    ///     Err(e) => eprintln!("Failed to create Stochastic instance: {}", e),
    /// }
    /// ```
    ///
    /// # Errors
    /// - Returns an error if a non-supported exchange name is given.
    /// - May return other errors if there's an issue fetching the K-line data
    #[allow(dead_code)]
    pub fn new(exchange: &'a str, params: KlineParams,) -> Result<Self, Box<dyn Error>> {

        let klines = match exchange {
            "coinbase"  => coinbase(params)?,
            "binance"   => binance(params)?,
            _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid exchange"))),
        };
        let lhc: Vec<Hlc> = klines
            .iter()
            .map(|kline| Hlc {
                price_high: kline.price_high,
                price_low: kline.price_low,
                price_close: kline.price_close,
            })
            .collect();

        Ok(Self { exchange, klines, lhc,
            capital         : 1000.0,
            exchange_fee    : None,

            min_qty         : None,
            min_price       : None,

            asset_scale     : 8,
            funds_scale     : 8,

            pnl_fast        : false
        })
    }

    pub fn capital(mut self, capital: f64) -> Self { self.capital = capital;self }

    pub fn exchange_fee(mut self, exchange_fee: f64) -> Self { self.exchange_fee = Some(exchange_fee);self }

    pub fn min_qty(mut self, min_qty: f64) -> Self {self.min_qty = Some(min_qty); self }

    pub fn min_price(mut self, min_price: f64) -> Self {self.min_price = Some(min_price); self }

    pub fn asset_scale(mut self, asset_scale: u32) -> Self {self.asset_scale = asset_scale; self }

    pub fn funds_scale(mut self, funds_scale: u32) -> Self {self.funds_scale = funds_scale; self }

    pub fn pnl_fast(mut self, pnl_fast: bool) -> Self {self.pnl_fast = pnl_fast; self }

    /// Calculates the Profit and Loss ([PnL]) based on the given parameters for the stochastic oscillator.
    ///
    /// This method first computes the values of the stochastic oscillator using the provided parameters.
    /// It then identifies data points where both the %K line and %D line are available.
    /// These points are then used to generate trigger signals which are subsequently fed into a simulation to determine the [PnL].
    ///
    /// # Parameters
    /// - `pnl_params`: An instance of [`PnlParams`] which contains parameters (like `k_length`, `k_smoothing`, and `d_length`) to compute the stochastic oscillator values.
    ///
    /// # Returns
    /// - An instance of [`PnL`] representing the result of the simulation based on the derived trigger signals.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use oscillatorsetups::exchange::chart_data::klines::{Intervals, KlineParams};
    /// use oscillatorsetups::pnl_simulator::stochastic::{PnlParams, Stochastic};
    ///
    /// let kline_params = KlineParams {
    ///     base_asset: "ETH",
    ///     quote_asset: "USD",
    ///     interval: Intervals::H4,
    ///     limit: 1000,
    ///     base_url:None,
    ///     source: Some("api"),
    /// };
    /// let stochastic = Stochastic::new("coinbase", kline_params).unwrap();
    ///
    /// let parameters = PnlParams {
    ///     k_length: 14,
    ///     k_smoothing: 3,
    ///     d_length: 3,
    /// };
    ///
    /// let result = stochastic.pnl(PnlParams { k_length:14, k_smoothing:3, d_length:3, });
    /// println!("PnL Result: {:?}", result);
    /// ```
    ///
    /// # Note
    /// - The method relies on a `stochastic` function to calculate the oscillator values and a `simulate` function to determine the PnL. Ensure that they are correctly implemented and are contextually appropriate.
    /// - Ensure proper error handling outside this method, especially if any of the called functions (`stochastic` or `simulate`) can raise exceptions or errors.
    #[allow(dead_code)]
    pub fn pnl(&self, pnl_params:PnlParams, ) -> PnL {
        // Calculate stochastic oscillator values.
        let stoch_values = stochastic(
            &self.lhc,
            pnl_params.k_length,
            pnl_params.k_smoothing,
            pnl_params.d_length
        );

        // Identify indices of the data points with both %K and %D lines available.
        let complete_indx: Vec<usize> = stoch_values
            .iter()
            .enumerate()
            .filter_map(|(index, value)| {
                if value.k_line.is_some() && value.d_line.is_some() {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

        // Map these indices to derive trigger signals for simulation.
        let data:Vec<TriggerSignal> = complete_indx
            .into_iter()
            .map(|indx| TriggerSignal {
                signal_in   : stoch_values[indx].k_line.unwrap(),
                signal_out  : stoch_values[indx].d_line.unwrap(),
                time_open   : self.klines[indx].time_open,
                price_open  : self.klines[indx].price_open,
                time_close  : self.klines[indx].time_close,
                price_close : self.klines[indx].price_close,
            }).collect();

        // Perform the simulation.
        let sim_params = SimulateParams::new(data)
            .capital(self.capital)
            .exchange_fee(self.exchange_fee)
            .min_qty(self.min_qty)
            .min_price(self.min_price)
            .asset_scale(self.asset_scale)
            .funds_scale(self.funds_scale);

        simulate(sim_params)

    }

    /// Identifies the top configurations (parameters) resulting in the highest net profits using the given range for the stochastic oscillator.
    ///
    /// This method systematically explores different configurations of the stochastic oscillator within the provided range.
    /// It then calculates the Profit and Loss (PnL) for each configuration and keeps track of the top 100 results by net profit.
    ///
    /// The method leverages parallel processing to speed up the computation of PnL across different configurations.
    /// The results are stored in a [`BTreeSet`] ensuring that they are sorted and the top configurations can be easily identified.
    ///
    /// # Parameters
    /// - `pnl_range`: An instance of [`PnlRange`] which defines the range (start and end) for each parameter (`k_length`, `k_smoothing`, and `d_length`) of the stochastic oscillator.
    ///
    /// # Algorithm
    /// 1. The method generates possible configurations based on [`pnl_range`].
    /// 2. For each configuration, the corresponding Profit and Loss (PnL) is computed.
    /// 3. The top 100 configurations by net profit are retained in a sorted [`BTreeSet`].
    /// 4. Results (top configurations and their net profits) are printed to standard output.
    ///
    /// # Side Effects
    /// - The method directly prints the top configurations along with their net profits to the standard output.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use oscillatorsetups::exchange::chart_data::klines::{Intervals, KlineParams};
    /// use oscillatorsetups::pnl_simulator::stochastic::{PnlRange, Stochastic};
    ///
    /// let kline_params = KlineParams {
    ///     base_asset  : "ETH",
    ///     quote_asset : "USD",
    ///     interval    : Intervals::H4,
    ///     limit       : 1000,
    ///     base_url    : None,
    ///     source      : Some("api"),
    /// };
    /// let stochastic = Stochastic::new("coinbase", kline_params).unwrap();
    ///
    /// let range = PnlRange {
    ///     k_length    : 5..=20,
    ///     k_smoothing : 3..=5,
    ///     d_length    : 3..=5,
    /// };
    ///
    /// stochastic.top_net_profit(range);
    /// // Expected output:
    /// // Net profit: XXX, Parameters: PnlParams { k_length: XX, k_smoothing: XX, d_length: XX }
    /// // ... (and so on for top configurations)
    /// ```
    ///
    /// # Note
    /// - The method uses a parallelized loop (`par_iter`) to compute the PnL for each configuration, ensuring efficient computation on multi-core systems.
    /// - Proper synchronization using `Arc` and `Mutex` ensures thread safety during concurrent modifications of the results.
    /// - This method can be computationally intensive, especially for larger ranges. Ensure optimal resource management when using it.
    #[allow(dead_code)]
    pub fn top_net_profit(&self, pnl_range:PnlRange){
        let top_profits = Arc::new(Mutex::new(BTreeSet::new()));

        // Generate possible parameter configurations.
        let k_length: Vec<_> = ((*pnl_range.k_length.start())..=(*pnl_range.k_length.end())).collect();
        let k_smoothing: Vec<_> = ((*pnl_range.k_smoothing.start())..=(*pnl_range.k_smoothing.end())).collect();
        let d_length: Vec<_> = ((*pnl_range.d_length.start())..=(*pnl_range.d_length.end())).collect();

        // For each parameter configuration, compute the PnL and track the top 100 results.
        k_length.par_iter().for_each(|&k_period| {
            println!("{}", k_period);
            for &k_smooth in &k_smoothing {
                for &d_smooth in &d_length {
                    let pnl_params = PnlParams { k_length: k_period, k_smoothing: k_smooth, d_length: d_smooth };
                    let pnl = self.pnl(pnl_params.clone());

                    let mut top_profits = top_profits.lock().unwrap();
                    top_profits.insert((Profit(pnl.net_profit), pnl_params));

                    if top_profits.len() > 100 {
                        let smallest = top_profits.iter().next().cloned().unwrap();
                        top_profits.remove(&smallest);
                    }
                }
            }
        });

        // Print the top 100 PnL configurations.
        for (profit, params) in &*top_profits.lock().unwrap() {
            println!("Net profit: {}, Parameters: {:?}", profit.0, params);
        }
    }
}

/// A simple structure representing profit, primarily designed for ordering and comparisons.
///
/// The `Profit` struct holds a single [`f64`] value, which represents the profit amount.
/// It provides implementations for equality and ordering to facilitate comparisons
/// and to be used in sorted collections like [`BTreeSet`].
///
/// # Derive
/// - `Debug`: Enables support for formatting using `{:?}`.
/// - `Clone`: Allows the creation of duplicate instances.
///
/// # Trait Implementations
/// - [`PartialEq::eq`]: Enables equality comparisons.
/// - [`Eq`]: Indicates that all values of this type are reflexive, symmetric, and transitive.
/// - [`PartialOrd::partial_cmp`]: Enables partial order comparisons.
/// - [`Ord::cmp`]: Provides a total ordering over `Profit`.
///
/// # Examples
///
/// ```rust
/// use oscillatorsetups::pnl_simulator::stochastic::Profit;
/// let profit1 = Profit(100.5);
/// let profit2 = Profit(150.0);
///
/// assert!(profit1 < profit2);
/// assert_ne!(profit1, profit2);
/// ```
///
/// # Caveats
/// - Although `Profit` contains a floating-point number, the implementations for ordering and
///   equality do not handle NaN values. Ensure that NaN is not used when working with `Profit`.
#[derive(Debug, Clone)]
pub struct Profit(pub f64);

impl PartialEq for Profit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Profit {}

impl PartialOrd for Profit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Profit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}