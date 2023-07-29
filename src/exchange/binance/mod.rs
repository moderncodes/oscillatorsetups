//! # Binance API Module
//!
//! This module contains the functionality to interact with the Binance cryptocurrency trading platform's API.
//!
//! The module is organized into several submodules, each handling specific functionalities related to the Binance API:
//!
//! - `models`: Provides data structures and models required to represent and deserialize the data received from the Binance API. It also contains custom deserialization logic for handling numerical values that might be returned as strings from the Binance API.
//! - `klines`: Focuses on fetching kline/candlestick data. Kline data represents how the price of a specific cryptocurrency trading pair has evolved over a set time interval. This submodule can retrieve data either directly from the Binance API or from a local JSON file. Additionally, it provides functionality to save the fetched data into local files.
//! - `exchange`: This submodule provides functionalities related to the exchange specifics of Binance, like trading symbols, filters applied to symbols, and exchange information. It also has functions to fetch data from specified sources (like "api" or "file") and supports saving some of this data to local files.
//! - `fetch`: Contains the primary function to make API calls to Binance, fetch data and handle potential errors or discrepancies in the API responses. It utilizes the [`models::ApiParams`] struct from the `models` submodule to guide its requests.
//!
//! ## Examples & Utilities
//!
//! The module and its submodules also contain several examples, primarily showcasing how to use the provided functions and structures. These examples can be a valuable guide for developers who are unfamiliar with the workings of the Binance API.
//!
//! Furthermore, utility functions like [`crate::utils::data_from_json`], [`crate::utils::data_to_json`], and [`crate::utils::string_to_f64`] are used across the module to handle common tasks like converting data to and from JSON or parsing strings to floating-point numbers.
//!
//! This comprehensive structure ensures that developers have a well-organized set of tools and documentation at their disposal when working with the Binance API.

pub mod exchange;
pub mod fetch;
pub mod klines;
pub mod models;
