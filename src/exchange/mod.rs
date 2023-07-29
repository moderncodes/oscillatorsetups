//! # Exchange Module
//!
//! The `exchange` module serves as the central hub for interfacing with different cryptocurrency exchanges.
//! It provides functionalities to fetch, represent, store, and manipulate data related to cryptocurrency trading.
//!
//! This module is comprised of the following submodules:
//!
//! - [`binance`]: Offers comprehensive functionality to interact with the Binance trading platform's API.
//!   It provides structures, utilities, and examples to help developers work with the Binance API efficiently.
//!   Notable features include fetching kline/candlestick data, exchange specifics, and direct API calls.
//!
//! - [`chart_data`]: A general-purpose module for K-lines (candlestick) data manipulation and representation.
//!   It abstracts chart data-related functionalities, making it easier to work with various exchanges like Binance and Coinbase.
//!   The submodule consolidates various functionalities and re-exports main entities for convenience.
//!
//! - [`coinbase`]: Centers around functionalities specific to the Coinbase exchange.
//!   It provides utilities, models, and submodules to support data retrieval, storage, and transformation from the Coinbase API.
//!   The module focuses on fetching kline data and offers examples for users to understand its capabilities better.
//!
//! Developers can dive into each submodule to understand specific functionalities and use the provided examples to guide their implementations.
//!
//! [`binance`]: ./binance/index.html
//! [`chart_data`]: ./chart_data/index.html
//! [`coinbase`]: ./coinbase/index.html

pub mod  binance;
pub mod chart_data;
pub mod coinbase;