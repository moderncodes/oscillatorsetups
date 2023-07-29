//! The `coinbase` module provides functionalities related to the Coinbase exchange.
//!
//! This module encompasses various utilities and models that support the retrieval,
//! storage, and transformation of data from the Coinbase API. Its design aims to
//! allow users to easily fetch kline (candlestick) data and store it for further use.
//!
//! # Modules:
//! - `fetch`: This submodule is responsible for the actual communication with the
//!   Coinbase API. It contains functions and utilities that prepare and send HTTP
//!   requests to retrieve product data. The submodule also handles pagination,
//!   rate limits, and creates the appropriate headers for the API calls.
//!
//! - `models`: The foundation of data structures used within the `coinbase` module.
//!   It defines the primary structures for API parameters ([`models::ApiParams`]) and kline data
//!   ([`models::Klines`]). These structures are used to represent, serialize, and deserialize
//!   the data retrieved from the Coinbase API or stored locally.
//!
//! - `candles`: This submodule bridges the functionalities of `fetch` and `models`.
//!   It provides a higher-level interface for users to retrieve kline data. Users
//!   can specify whether they want data from the API directly or from a local file.
//!   Additionally, it supports saving fetched data to a local file.
//!
//! # Examples:
//!
//! ## Fetching kline data from the API and saving to a file:
//! ```ignore
//! use crate::oscillatorsetups::exchange::coinbase::candles;
//!
//! let base_url = "https://api.exchange.coinbase.com";
//! let product_id = "ETH-USD";
//! let data = candles::remote_to_file(base_url, 3600, 300, product_id).unwrap();
//! ```
//!
//! For more specific examples and documentation, please refer to the respective
//! submodules.
//!
pub mod fetch;
pub mod models;
pub mod candles;
