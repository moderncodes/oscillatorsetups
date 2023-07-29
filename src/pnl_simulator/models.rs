/// `PnL` struct holds various statistical measures about trading strategy performance.
///
/// # Fields
///
/// - `net_profit`: Total returns of the strategy subtracting losses and commissions.
/// - `gross_profit`: The sum of all profitable trades.
/// - `gross_loss`: The sum of all losing trades.
/// - `buy_and_hold_return`: The return if we just bought and held the asset without trading.
/// - `profit_factor`: The ratio of gross profit to gross loss. A value greater than 1 indicates a profitable system.
/// - `commission_paid`: Total commission paid for all trades.
/// - `total_closed_trades`: Total number of closed trades.
/// - `num_winning_trades`: Number of trades that resulted in profit.
/// - `num_losing_trades`: Number of trades that resulted in loss.
/// - `percent_profitable`: Percentage of trades that were profitable.
/// - `avg_winning_trade`: Average profit for winning trades.
/// - `avg_losing_trade`: Average loss for losing trades.
/// - `ratio_avg_win_loss`: Ratio of the average win to the average loss.
/// - `largest_winning_trade`: The largest profit from a single trade.
/// - `largest_losing_trade`: The largest loss from a single trade.
/// - `avg_ticks_in_winning_trades`: Average number of ticks (time periods) that winning trades were held.
/// - `avg_ticks_in_losing_trades`: Average number of ticks (time periods) that losing trades were held.
///
#[derive(Debug)]
pub struct PnL {
    pub net_profit: f64,
    pub gross_profit: f64,
    pub gross_loss: f64,
    pub buy_and_hold_return: f64,
    pub profit_factor: f64,
    pub commission_paid: Option<f64>,
    pub total_closed_trades: i32,
    pub num_winning_trades: i32,
    pub num_losing_trades: i32,
    pub percent_profitable: f64,
    pub avg_winning_trade: f64,
    pub avg_losing_trade: f64,
    pub ratio_avg_win_loss: f64,
    pub largest_winning_trade: f64,
    pub largest_losing_trade: f64,
    pub avg_ticks_in_winning_trades: f64,
    pub avg_ticks_in_losing_trades: f64,
}
/// `TriggerSignal` struct holds data used for calculating PnL analysis
///
/// # Fields
/// - `signal_in`   : Higher value over `signal_out` triggers entry
/// - `signal_out`  : Higher value over `signal_in` triggers exit
/// - `time_open`   : The time that the kline/candlestick open, represented as a Unix timestamp.
/// - `time_close`  : The time that the kline/candlestick closed, represented as a Unix timestamp.
/// - `price_open`  : The price at the opening of the kline/candlestick.
/// - `price_close` : The price at the closing of the kline/candlestick.
///
#[derive(Debug)]
pub struct TriggerSignal {
    pub signal_in   : f64,
    pub signal_out  : f64,

    pub time_open   : u64,
    pub time_close  : u64,

    pub price_open  : f64,
    pub price_close : f64,
}