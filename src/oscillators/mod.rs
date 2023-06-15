//! # Oscillators Module
//!
//! This module contains functionality related to technical analysis oscillators,
//! which are used in the field of financial analysis to predict future price movement.
//!
//! The `models` module defines the necessary data structures used in these calculations.
//! The `sma` module contains functions related to the Simple Moving Average (SMA), a type of oscillator.
//! The `stochastic` module contains functions for calculating the stochastic oscillator, another popular indicator.
//!
//! Each of the submodules (`sma` and `stochastic`) provides functions that operate on ticks and tick arrays.
//!
//! This module also provides some global constants used across all submodules in the `globals` module.
//!
//! For usage examples and more detailed information, please refer to the specific submodule documentation.
//!
//! ## Submodules
//! * `models`: Data structures used in oscillator calculations.
//! * `sma`: Simple Moving Average (SMA) calculation functionality.
//! * `stochastic`: Stochastic oscillator calculation functionality.
//! * `globals`: Global constants used across all submodules.
mod globals;
pub mod models;
pub mod sma;
pub mod stochastic;
