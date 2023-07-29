# oscillatorsetups
## Cryptocurrency Technical Analysis Library

`oscillatorsetups` is a Rust-based technical analysis library for cryptocurrencies. 
It's tailored to help determine the most profitable configurations for various technical oscillators. 
While the library currently emphasizes the Stochastic Oscillator, future versions will incorporate others such as MACD, RSI, and more.

## Features:
* **Oscillators**: Perform technical analysis computations on trading data.
* **Data Fetching**: Although not its primary focus, the library can fetch trading data from cryptocurrency exchanges as a supplementary tool to assist in oscillator analysis.
* **Profit and Loss Simulation**: Analyze potential profits and losses across various oscillator configurations.


## Installation

To include `oscillatorsetups` in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
oscillatorsetups = "0.1.0"
```

## Usage
### Analyzing Oscillator Configurations:
Here's how you can utilize the library to determine profitable Stochastic Oscillator configurations.
```rust
use oscillatorsetups::exchange::chart_data::klines::{Intervals, KlineParams};
use oscillatorsetups::pnl_simulator::stochastic::{PnlParams, Stochastic};

fn main() {
    let stochastic = match Stochastic::new(
        "binance",  // or "coinbase"
        KlineParams {
            base_asset  : "ETH",
            quote_asset : "USDT",
            interval    : Intervals::H4, // Refer to exchange api for correct intervals set values
            limit       : 1000,
            base_url    : None, // Defaults: binance is https://api.binance.us or coinbase is "https://api.exchange.coinbase.com"
            source      : Some("api"),
        }) {
        Ok(s) => s
            .exchange_fee(0.00075)  // Default None
            .min_qty(0.0001)        // Default None
            .min_price(0.01),       // Default None
        Err(e) => {
            eprintln!("Failed to create Stochastic: {}", e);
            return;
        }
    };
    stochastic.top_net_profit(PnlRange { k_length:5..=42, k_smoothing:3..=42, d_length:3..=42, });
    
    /* At the time of analyzing, above results were:
    Net profit: 416.82, Parameters: PnlParams { k_length:  7, k_smoothing: 41, d_length: 24 }
    Net profit: 418.47, Parameters: PnlParams { k_length: 11, k_smoothing: 40, d_length: 17 }
    Net profit: 424.65, Parameters: PnlParams { k_length:  8, k_smoothing: 41, d_length: 21 }
    Net profit: 426.67, Parameters: PnlParams { k_length:  8, k_smoothing: 41, d_length: 24 }
    Net profit: 427.69, Parameters: PnlParams { k_length: 23, k_smoothing: 8, d_length: 41 }
    Net profit: 435.26, Parameters: PnlParams { k_length:  8, k_smoothing: 40, d_length: 16 }
    Net profit: 437.07, Parameters: PnlParams { k_length:  6, k_smoothing: 42, d_length: 15 }
    Net profit: 440.80, Parameters: PnlParams { k_length:  8, k_smoothing: 41, d_length: 23 }
    Net profit: 444.60, Parameters: PnlParams { k_length:  8, k_smoothing: 40, d_length: 24 }
    Net profit: 456.35, Parameters: PnlParams { k_length: 42, k_smoothing: 3, d_length: 4 }
    */
    
    // Further to get full breakdown of most profitable configurations
    let pnl = stochastic.pnl(PnlParams { k_length: 42, k_smoothing: 3, d_length: 4, });
    println!("{:#?}",pnl);
    
    /* Prints 
    PnL { 
        net_profit: 456.35, 
        gross_profit: 1143.09, 
        gross_loss: -686.74, 
        buy_and_hold_return: 102.81, 
        profit_factor: 1.665, 
        commission_paid: Some(191.792,), 
        total_closed_trades: 94, 
        num_winning_trades: 36, 
        num_losing_trades: 58, 
        percent_profitable: 38.3, 
        avg_winning_trade: 31.75, 
        avg_losing_trade: -11.84, 
        ratio_avg_win_loss: 2.682, 
        largest_winning_trade: 130.9, 
        largest_losing_trade: -39.92, 
        avg_ticks_in_winning_trades: 7.36, 
        avg_ticks_in_losing_trades: 3.65, 
    } 
    */
}
```

### Exchange fees info
Each exchange has its fee structure, and the fee calculations can be different based on factors such as trading volume, order types, membership levels, and the use of native exchange tokens.

As an example coinbase fee tiers typically start at 0.50% for Taker and 0.50% for Maker and can be reduced to as low as 0.04% for Taker and 0.00% for Maker for high-volume traders.\
Hypothetically, if you were to purchase an asset worth $1000 on Coinbase Pro and the trading fee is 0.50%, you can calculate the fee as follows:\

#### Coinbase
The fee percentage will be in decimal form 0.50% = 0.005 (because 0.50 ÷ 100 = 0.005)\
Fee = $1000 (asset value) x 0.005 (fee percentage in decimal form) = $5.00
#### Binance
Standard Fee = $1000 (asset value) x 0.001 (standard fee percentage in decimal form)\
Standard Fee = $1\
BNB Discount Fee at 25% = $0.75\
**NOTE**: as of this post, Binance offers 0% on Tier 0 pairs [Fees - Binance US](https://www.binance.us/fees) Bitcoin and Ethereum trading in the U.S

## Testing
To run the tests for the library, use the command `cargo test`.

## Contributing
Your contributions are welcome and appreciated!
1. Fork the repository.
2. Create your feature branch (git checkout -b feature/YourFeatureName).
3. Commit your changes (git commit -am 'Add some feature').
4. Push to the branch (git push origin feature/YourFeatureName).
5. Open a pull request.

For significant changes, it's recommended to open an issue first to discuss the proposed modifications.

## Disclaimer:
This library offers tools for technical analysis based on historical market data. Please understand its limitations:
* **Past vs. Future**: Analysis results are based on historical data, which doesn't guarantee future outcomes.
* **No Predictive Guarantees**: While grounded in established financial theories, the library doesn't guarantee market predictions.
* **Diversify Strategies**: Do not rely solely on this tool. It's wise to merge its outputs with other strategies and insights.
* **Invest Responsibly**: Especially if new to trading or technical analysis, start with smaller investments to grasp market dynamics.

Your trading and investment decisions should always be backed by comprehensive research. When in doubt, seek advice from financial experts.

## License
This project is licensed under the Apache License 2.0 License. See the LICENSE file for more information.