//! The `chart_data` module provides structures and functionalities related to exchange chart data.
//! It is general structures for K-lines manipulation and representation
//! This encompasses functionality for fetching and manipulating data from different exchanges.
//!
//! The module consolidates functionalities from individual sub-modules:
//! - `binance`: Contains functions and structures specific to Binance's chart data.
//! - `coinbase`: Contains functions and structures specific to Coinbase's chart data.
//! - `klines`: Provides an abstracted representation of K-line (or candlestick) data and related functionalities.
//!
//! Re-exported for convenience are the main entities of each submodule.

pub mod klines;
