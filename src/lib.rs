//! # Financial Technical Analysis Library
//!
//! `oscillatorsetups` is a financial technical analysis library, focused on determining the most
//! profitable configurations for various technical oscillators, including but not limited to the Stochastic Oscillator, MACD,
//! RSI, and others in development.
//!
//! ## Modules:
//! * `oscillators`: Dedicated to various financial technical analysis oscillators.
//! * `pnl_simulator`: Simulates and analyzes profit and loss across a range of oscillator strategies.
//! * `utils`: Utility functions for operations across the library.
//! * `exchange`: Provides optional functionalities to interact with cryptocurrency exchange platforms.
//!
//! Cryptocurrency `OHLCV` candlestick data was chosen due to its accessibility and cost-free API options, facilitating efficient testing and development.
//! However, any other stock index or crypto pair candlestick data can also be used for analysis.
//! While the library has the capability to fetch trading data from exchanges, this feature serves primarily
//! as a supplementary tool to obtain necessary data for oscillator analysis, rather than a primary functionality.
//!
//! For detailed descriptions, usage examples, and more, refer to the specific module documentation.
//!
//! ## Example Usage:
//! Analyzing the most profitable Stochastic Oscillator configurations:
//! ```rust,no_run
//! use oscillatorsetups::exchange::chart_data::klines::{Intervals, KlineParams};
//! use oscillatorsetups::pnl_simulator::stochastic::{PnlParams,PnlRange, Stochastic};
//!
//! # fn main() {
//! let stochastic = match Stochastic::new(
//!     "coinbase",  // or "coinbase"
//!     KlineParams {
//!         base_asset  : "ETH",
//!         quote_asset : "USD",
//!         interval    : Intervals::M15, // Refer to exchange api for correct intervals set values
//!         limit       : 1000,
//!         base_url    : None, // Defaults: binance is https://api.binance.us or coinbase is "https://api.exchange.coinbase.com"
//!         source      : Some("api"),
//!     }) {
//!     Ok(s) => s
//!         //.exchange_fee(0.00075)  // Default None
//!         .min_qty(0.0001)        // Default None
//!         .min_price(0.01),       // Default None
//!     Err(e) => {
//!         eprintln!("Failed to create Stochastic: {}", e);
//!         return;
//!     }
//! };
//!
//! let top_profits = stochastic.top_net_profit(PnlRange {
//!     k_length: 5..=42,
//!     k_smoothing: 3..=42,
//!     d_length: 3..=42,
//! });
//! for (profit, params) in &*top_profits.lock().unwrap() {
//!     println!("Net profit: {}, Parameters: {:?}", profit.0, params);
//! }
//!
//! /* Result
//! Net profit: 416.82, PnlParams { k_length:  7, k_smoothing: 41, d_length: 24 }
//! Net profit: 418.47, PnlParams { k_length: 11, k_smoothing: 40, d_length: 17 }
//! Net profit: 424.65, PnlParams { k_length:  8, k_smoothing: 41, d_length: 21 }
//! Net profit: 426.67, PnlParams { k_length:  8, k_smoothing: 41, d_length: 24 }
//! Net profit: 427.69, PnlParams { k_length: 23, k_smoothing:  8, d_length: 41 }
//! Net profit: 435.26, PnlParams { k_length:  8, k_smoothing: 40, d_length: 16 }
//! Net profit: 437.07, PnlParams { k_length:  6, k_smoothing: 42, d_length: 15 }
//! Net profit: 440.80, PnlParams { k_length:  8, k_smoothing: 41, d_length: 23 }
//! Net profit: 444.60, PnlParams { k_length:  8, k_smoothing: 40, d_length: 24 }
//! Net profit: 456.35, PnlParams { k_length: 42, k_smoothing:  3, d_length: 4 }
//! */
//!
//! let pnl = stochastic.pnl(PnlParams { k_length: 42, k_smoothing: 3, d_length: 4, });
//! println!("{:#?}",pnl);
//! /* Prints
//! PnL {
//!     net_profit: 456.35,
//!     gross_profit: 1143.09,
//!     gross_loss: -686.74,
//!     buy_and_hold_return: 102.81,
//!     profit_factor: 1.665,
//!     commission_paid: Some(191.792,),
//!     total_closed_trades: 94,
//!     num_winning_trades: 36,
//!     num_losing_trades: 58,
//!     percent_profitable: 38.3,
//!     avg_winning_trade: 31.75,
//!     avg_losing_trade: -11.84,
//!     ratio_avg_win_loss: 2.682,
//!     largest_winning_trade: 130.9,
//!     largest_losing_trade: -39.92,
//!     avg_ticks_in_winning_trades: 7.36,
//!     avg_ticks_in_losing_trades: 3.65,
//! }
//! */
//!
//! # }
//! ```
//! ## Common Issues and Solutions
//! **Issue**:
//! Failed to create Stochastic: error decoding response body: invalid type: map, expected a sequence at line 1 column 0
//! Fix Suggestion: Check if the exchange supports the specified interval in:
//! **Fix Suggestion**:
//! Ensure that the exchange you are querying supports the specified interval. Check and modify the interval in the following configuration:
//! ```ignore
//! KlineParams {
//!     base_asset  : "ETH",
//!     quote_asset : "USDT",
//!     interval    : Intervals::M15, // Refer to the exchange API for the correct set of interval values.
//!     limit       : 1000,
//!     base_url    : None, // Defaults: Binance is "https://api.binance.us" and Coinbase is "https://api.exchange.coinbase.com"
//!     source      : Some("api"),
//! }
//! ```
//!
//! ## Disclaimer:
//! This library is intended to be a tool for technical analysis based on historical data. It does not offer any guarantee
//! of future performance. Like all tools based on historical analysis, its results should be used responsibly and with caution:
//!
//! * **Historical Analysis**: The results and recommendations generated by this library are based on past market data. Past performance does not guarantee future results.
//!
//! * **No Crystal-Gazer Predictions**: While the algorithms used are based on tested financial theories and methods, there is no foolproof tool for predicting market movements. Financial markets can be influenced by a myriad of unpredictable factors.
//!
//! * **Diversified Strategies**: Relying on one strategy or tool alone is often not the ideal approach. It's important to integrate the outputs of this library with other strategies, tools, and market insights.
//!
//! * **Learning by Doing**: Especially for those new to cryptocurrency trading or technical analysis, it's advisable to start with small investments to understand the risks and dynamics of the markets.
//!
//! Trading and investment decisions should be made based on thorough research and, if possible, with the advice of financial professionals.

pub mod oscillators;
pub mod pnl_simulator;
pub mod utils;
pub mod exchange;
