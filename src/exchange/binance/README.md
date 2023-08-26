# Binance API Module

This sub module contains the functionality to interact with the Binance cryptocurrency trading platform's API.\
While the library has the capability to fetch trading data from exchange, this feature serves primarily as a supplementary tool to obtain necessary data for oscillator and their analysis, rather than a primary functionality.

## Usage

### Exchange klines
```rust
use oscillatorsetups::exchange::binance::{models::ApiParams,klines::remote_to_file};
use std::collections::HashMap;

fn main() {
    let api_params = ApiParams {
        base_url: "https://api.binance.us",
        endpoint: "/api/v3/klines",
        params  : &HashMap::from([("interval", "15m"), ("limit", "2"), ("symbol", "ETHUSDT")]),
    };

    let klines = remote_to_file(api_params);
    println!("{:#?}", klines);

}
```

Output will be similar as:
```
x-mbx-used-weight: Some("2")
x-mbx-used-weight-1m: Some("2")                        
Ok(                                                    
    [                                                  
        Klines {                                       
            open_time: 1689301800000,
            open_price: 1619.68,
            high_price: 1635.0,
            low_price: 1600.0,
            close_price: 1600.0,
            volume: 66.8571,
            close_time: 1689302699999,
            quote_asset_volume: 107658.785106,
            number_of_trades: 539,
            taker_buy_base_asset_volume: 34.5637,
            taker_buy_quote_asset_volume: 55842.597553,
            unused_field: "0",
        },
        Klines {
            open_time: 1689302700000,
            open_price: 1605.06,
            high_price: 1615.0,
            low_price: 1579.76,
            close_price: 1595.49,
            volume: 79.0479,
            close_time: 1689303599999,
            quote_asset_volume: 126802.649083,
            number_of_trades: 694,
            taker_buy_base_asset_volume: 23.5694,
            taker_buy_quote_asset_volume: 37825.68282,
            unused_field: "0",
        },
    ],
)
```