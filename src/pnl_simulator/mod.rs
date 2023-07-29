//! The `pnl_simulator` module provides functionality to simulate and analyze profit and loss based
//! on various parameters and algorithms, specifically focusing on stochastic operations.

//! The `models` module contains the necessary data structures to support
//! the PnL simulations and stochastic operations.
//!
//! This might include representations for candles, signals, PnL results,
//! and any other related entities.

//! The `pnl` module provides the core logic for simulating profit and loss.
//!
//! This includes the main `simulate` function which calculates the potential
//! profit or loss based on provided trigger signals and simulation parameters.

//! The `stochastic` module contains the logic related to stochastic calculations
//! and their utilization in the PnL simulations.
//!
//! This encompasses the generation of stochastic values, the calculation of profit
//! and loss based on these values, and any related utility functions and structures.

pub mod models;
pub mod pnl;
pub mod stochastic;