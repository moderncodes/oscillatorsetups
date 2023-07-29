//! # Oscillators Module
//!
//! The `oscillators` module provides various financial technical analysis oscillators.
//!
//! Technical oscillators are tools that can provide buy and sell signals in a range of technical analysis and trading.
//! Oscillators are typically used to gauge the momentum and direction of a security or index over a specific time frame.
//! They move within a range (e.g., 0 to 100) and can provide insights on overbought or oversold conditions.
//!
//! This module contains models, calculations, and utilities for working with various well-known oscillators.
//!
//! # Sub-modules:
//! - `models`: Contains data structures and models required for oscillator calculations, such as [`models::Hlc`].
//! - `sma`: Provides functions for calculating the Simple Moving Average (SMA) of price data.
//! - `stochastic`: Offers functionalities related to the Stochastic Oscillator, including the raw stochastic value
//!   calculation (%K), and the smoothed stochastic value (%D).
//!
//! Depending on the specific oscillator you're interested in, you might then dive deeper into one of the sub-modules
//! to use its functions or structures.

pub mod models;
pub mod sma;
pub mod stochastic;
