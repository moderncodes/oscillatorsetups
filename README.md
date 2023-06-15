# oscillatorsetups
## Cryptocurrency Technical Analysis Library

This Rust library provides functionalities to fetch and process cryptocurrency trading data. It is composed of two main modules:

- `binance_api`: This module interacts with the Binance cryptocurrency trading platform's API to fetch trading data.
- `oscillators`: This module performs various technical analysis computations on the trading data.

## Installation

To use this library in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
oscillatorsetups = "0.0.1"
```

## Usage
### Below is a basic example of how to use the library to fetch price data from Binance API:
```rust
use crate::binance_api::klines::{get_data_from_source, ApiParams};
#[tokio::main]
async fn main() {
    let params = ApiParams {
        limit: 100,
        trading_pair: "ETHUSD".to_string(),
        interval: "15m".to_string(),
        endpoint: 1,
    };
    let response = get_data_from_source("api", &params).await.unwrap();
    println!("{:#?}", response);
}
```
You'll need to add an async main and call this function within it since get_data_from_source is an async function.

### Or get stochastic values
```rust
fn main(){
    let response:Vec<Kline> = get_data_from_source("api", &params).await.unwrap();
    let mut lhc: Vec<Hlc> = response
        .into_iter()
        .map(|kline| Hlc {
            price_high: kline.high_price,
            price_low: kline.low_price,
            price_close: kline.close_price,
        })
        .collect();

    lhc.pop(); // Data received from binance API contains uncompleted tik data. Removing last index on your discretion

    let stoch_data = stochastic(&lhc, 14, 1, 3);
    
    for val in stoch_data {
        println!("{:?}", val);
    }
}

```

## Testing
To run the tests for the library, use the command cargo test.

## Contributing
Contributions to this project are welcome! Please create a fork of this repository, make your changes, and submit a pull request. For major changes, please open an issue first to discuss what you would like to change.

## License
This project is licensed under the Apache License 2.0 License. See the LICENSE file for more information.