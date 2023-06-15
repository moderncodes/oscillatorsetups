//! # Binance API Module
//!
//! This module contains the functionality to interact with the Binance cryptocurrency trading platform's API.
//!
//! The `models` submodule contains data structures for deserializing the responses from the Binance API.
//! It also includes custom deserialization logic for handling numerical values that Binance may return as strings.
//!
//! The `klines` submodule contains functionality to perform API requests to retrieve kline/candlestick data,
//! which represents how the price of a particular cryptocurrency trading pair has changed over a given time interval.
//! This module can fetch data directly from the Binance API or from a local JSON file, and it also provides a function
//! for saving data fetched from the API to a local file.
//!
//! ## Submodules
//! * `models`: Contains data structures for deserializing the responses from the Binance API.
//! * `klines`: Contains functions to perform API requests and handle responses.
//!
//! For usage examples and more detailed information, please refer to the specific submodule documentation.
pub mod klines;
pub mod models;
