//! A module for simulating Profit and Loss (PnL) based on trading signals.
use super::models::{PnL, TriggerSignal};

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

/// Parameters required for simulating trading.
/// # Fields
/// - `signals`         : Vec<[TriggerSignal]>
/// - `initial_capital` : Default (1000.00). Starting capital, amount of funds initially available for the simulation.
/// - `exchange_fee`    : Default (None). Exchange fees, if any, paid for each entry and exit.
/// - `min_qty`         : Default (None). Minimum quantity or step size allowed when placing a trading order for a particular asset. It is also known as the "lot size" or "order step size" or "quantity increments"
/// - `min_price`       : Default (None). Same as min_qty, only this is the minimum price, or price increment allowed when placing order.
/// - `asset_scale`     : Default (8). The asset displayed precision in your wallet balance
/// - `funds_scale`     : Default (8). The quote or as price displayed precision in your wallet balance
///
/// ## Reference of methods
/// - [SimulateParams::new] - use constructor to apply `defaults`
/// - [SimulateParams::capital] - sets initial_capital [`initial_capital`]: SimulateParams::initial_capital
/// - [SimulateParams::exchange_fee] - sets `exchange_fee`
/// - [SimulateParams::min_qty] - sets `min_qty`
/// - [SimulateParams::min_price] - sets `min_price`
/// - [SimulateParams::asset_scale] - sets `asset_scale`
/// - [SimulateParams::get_asset_trade_scale] - sets `get_asset_trade_scale`
/// - [SimulateParams::get_funds_trade_scale] - sets `get_funds_trade_scale`
///
/// # Examples
/// ```
/// use oscillatorsetups::pnl_simulator::{ models::TriggerSignal, pnl::SimulateParams };
///
/// // TriggerSignal instance
/// let trigger_signal = TriggerSignal { signal_in:10.0, signal_out:9.0, time_open:1689294600000, time_close:1689295499999, price_open:1639.26, price_close : 1734.3, };
/// // vector containing the trigger_signals
/// let signals = vec![trigger_signal];
///
/// // Set SimulateParams instance
/// let initial_capital:f64         = 10000.0;
/// let exchange_fee:Option<f64>    = Some(0.00075); // Assuming a 0.075% exchange fee
/// let min_qty:Option<f64>         = Some(0.01);
/// let min_price:Option<f64>       = Some(10.0);
/// let asset_scale:u32             = 8;
/// let funds_scale:u32             = 8;
///
/// // Create SimulateParams instance
/// let params = SimulateParams { signals, initial_capital, exchange_fee, min_qty, min_price, asset_scale, funds_scale, };
///
/// assert_eq!(params.signals[0].signal_in, 10f64);
/// assert_eq!(params.signals[0].signal_out, 9f64);
/// assert_eq!(params.signals[0].time_open, 1689294600000u64);
/// assert_eq!(params.signals[0].time_close, 1689295499999u64);
/// assert_eq!(params.signals[0].price_open, 1639.26f64);
/// assert_eq!(params.signals[0].price_close, 1734.3f64);
/// assert_eq!(params.initial_capital, 10000f64);
/// assert_eq!(params.exchange_fee, Some(0.00075f64));
/// assert_eq!(params.min_qty, Some(0.01f64));
/// assert_eq!(params.min_price, Some(10f64));
/// assert_eq!(params.asset_scale, 8u32);
/// assert_eq!(params.funds_scale, 8u32);
/// ```
pub struct SimulateParams {
    pub signals         : Vec<TriggerSignal>,
    pub initial_capital : f64,
    pub exchange_fee    : Option<f64>,
    pub min_qty         : Option<f64>,
    pub min_price       : Option<f64>,
    pub asset_scale     : u32,
    pub funds_scale     : u32,
}

impl SimulateParams {
    /// Constructs a new`SimulateParams` with the provided signals, and default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use oscillatorsetups::pnl_simulator::{ models::TriggerSignal, pnl::SimulateParams };
    ///
    /// // TriggerSignal instance
    /// let trigger_signal = TriggerSignal { signal_in:10.0, signal_out:9.0, time_open:1689294600000, time_close:1689295499999, price_open:1639.26, price_close:1734.3, };
    ///
    /// // vector containing the trigger_signals
    /// let signals = vec![trigger_signal];
    /// let params = SimulateParams::new(signals);
    ///
    /// // Assert against default values
    /// assert_eq!(params.initial_capital, 1000.0);
    /// assert_eq!(params.exchange_fee, None);
    /// assert_eq!(params.min_qty, None);
    /// assert_eq!(params.min_price, None);
    /// assert_eq!(params.asset_scale, 8);
    /// assert_eq!(params.funds_scale, 8);
    /// ```
    pub fn new(signals:Vec<TriggerSignal>) -> Self {
        SimulateParams { signals,
            initial_capital     : 1000.0,
            exchange_fee        : None,

            min_qty     : None,
            min_price   : None,

            asset_scale : 8,
            funds_scale : 8
        }
    }

    /// set optional `initial_capital`
    /// # Example
    /// ```
    /// use oscillatorsetups::pnl_simulator::{ models::TriggerSignal, pnl::SimulateParams };
    ///
    /// let trigger_signal = TriggerSignal { signal_in:10.0, signal_out:9.0, time_open:1689294600000, time_close:1689295499999, price_open:1639.26, price_close:1734.3, };
    /// let signals = vec![trigger_signal];
    /// let params = SimulateParams::new(signals).capital( 100000.0 );
    /// // Assert against new values
    /// assert_eq!(params.initial_capital, 100000f64);
    ///```
    pub fn capital(mut self, capital: f64) -> Self { self.initial_capital = capital;self }

    /// set optional `exchange_fee`
    /// # Example
    /// ```
    /// use oscillatorsetups::pnl_simulator::{ models::TriggerSignal, pnl::SimulateParams };
    ///
    /// let trigger_signal = TriggerSignal { signal_in:10.0, signal_out:9.0, time_open:1689294600000, time_close:1689295499999, price_open:1639.26, price_close:1734.3, };
    /// let signals = vec![trigger_signal];
    /// let params = SimulateParams::new(signals).exchange_fee( Some(0.00075) );
    /// // Assert against new values
    /// assert_eq!(params.exchange_fee, Some(0.00075f64));
    ///```
    pub fn exchange_fee(mut self, exchange_fee: Option<f64>) -> Self { self.exchange_fee = exchange_fee;self }

    /// set optional `min_qty`
    /// # Example
    /// ```
    /// use oscillatorsetups::pnl_simulator::{ models::TriggerSignal, pnl::SimulateParams };
    ///
    /// let trigger_signal = TriggerSignal { signal_in:10.0, signal_out:9.0, time_open:1689294600000, time_close:1689295499999, price_open:1639.26, price_close:1734.3, };
    /// let signals = vec![trigger_signal];
    /// let params = SimulateParams::new(signals).min_qty( Some(10.0) );
    /// // Assert against new values
    /// assert_eq!(params.min_qty, Some(10f64));
    ///```
    pub fn min_qty(mut self, min_qty: Option<f64>) -> Self {self.min_qty = min_qty; self }

    /// set optional `min_price`
    /// # Example
    /// ```
    /// use oscillatorsetups::pnl_simulator::{ models::TriggerSignal, pnl::SimulateParams };
    ///
    /// let trigger_signal = TriggerSignal { signal_in:10.0, signal_out:9.0, time_open:1689294600000, time_close:1689295499999, price_open:1639.26, price_close:1734.3, };
    /// let signals = vec![trigger_signal];
    /// let params = SimulateParams::new(signals).min_price( Some(0.01) );
    /// // Assert against new values
    /// assert_eq!(params.min_price, Some(0.01f64));
    ///```
    pub fn min_price(mut self, min_price: Option<f64>) -> Self {self.min_price = min_price; self }

    /// set optional `asset_scale`
    pub fn asset_scale(mut self, asset_scale: u32) -> Self {self.asset_scale = asset_scale; self }

    /// set optional `funds_scale`
    pub fn funds_scale(mut self, funds_scale: u32) -> Self {self.funds_scale = funds_scale; self }

    /// Returns the scale (number of decimal places) of the minimum quantity (`min_qty`) property
    /// for the asset trade. This is helpful to determine the precision at which the asset
    /// trades occur. If `min_qty` is not set, the function returns `None`.
    ///
    /// # Examples
    /// ```
    /// use oscillatorsetups::pnl_simulator::{ models::TriggerSignal, pnl::SimulateParams };
    ///
    /// let trigger_signal = TriggerSignal { signal_in:10.0, signal_out:9.0, time_open:1689294600000, time_close:1689295499999, price_open:1639.26, price_close:1734.3, };
    /// let signals = vec![trigger_signal];
    /// let params = SimulateParams::new(signals).min_qty(Some(0.001));
    /// assert_eq!(params.get_asset_trade_scale(), Some(3));  // 3 decimal places in 0.001
    /// ```
    pub fn get_asset_trade_scale(&self) -> Option<u32> {
        let mut scale_result:Option<u32> = None;
        if let Some(min_qty) = self.min_qty {
            let decimal_val = Decimal::from_f64(min_qty).unwrap();
            scale_result = Some(decimal_val.scale());
        }
        scale_result
    }

    /// Returns the scale (number of decimal places) of the minimum price (`min_price`)
    /// for the funds trade. This helps in determining the precision at which the funds
    /// trades are conducted. If `min_price` is not set, the function returns `None`.
    ///
    /// # Examples
    /// ```
    /// use oscillatorsetups::pnl_simulator::{ models::TriggerSignal, pnl::SimulateParams };
    ///
    /// let trigger_signal = TriggerSignal { signal_in:10.0, signal_out:9.0, time_open:1689294600000, time_close:1689295499999, price_open:1639.26, price_close:1734.3, };
    /// let signals = vec![trigger_signal];
    /// let params = SimulateParams::new(signals).min_price(Some(0.01));
    /// assert_eq!(params.get_funds_trade_scale(), Some(2));  // 2 decimal places in 0.01
    /// ```
    pub fn get_funds_trade_scale(&self) -> Option<u32> {
        let mut scale_result:Option<u32> = None;
        if let Some(min_price) = self.min_price {
            let decimal_val = Decimal::from_f64(min_price).unwrap();
            scale_result = Some(decimal_val.scale());
        }
        scale_result
    }

}

/// Simulates a sequence of trades based on the given parameters and computes key trading performance metrics.
///
/// This function uses the provided simulation parameters to drive a series of buy and sell decisions.
/// It tracks the performance of these decisions in terms of net profit, gross profit, gross loss,
/// buy-and-hold return, and other related metrics. The purpose of this function is to help evaluate
/// the potential success of a trading strategy using historical data.
///
/// # Parameters
/// - `sim_params`: [SimulateParams] The parameters that drive the simulation. This includes data such as price signals,
///                 fees, initial capital, and trade scaling.
///
/// # Returns
/// A [`PnL`] object which encapsulates various trading performance metrics, such as net profit,
/// number of winning trades, average winning trade value, etc.
///
/// # Notes
/// The simulation iterates through each "tick" (price point) in the provided signals. Depending on
/// the relation between the `signal_in` and `signal_out` values of the tick and the current position status,
/// a buy or sell decision is simulated. The performance metrics are updated based on the outcome
/// of these simulated trades.
pub fn simulate(sim_params: SimulateParams) -> PnL {
    let mut pnl = PnL {
        net_profit      : 0.0,
        gross_profit    : 0.0,
        gross_loss      : 0.0,
        buy_and_hold_return : 0.0,
        profit_factor   : 0.0,
        commission_paid : None,
        total_closed_trades : 0,
        num_winning_trades  : 0,
        num_losing_trades   : 0,
        percent_profitable  : 0.0,
        avg_winning_trade   : 0.0,
        avg_losing_trade    : 0.0,
        ratio_avg_win_loss  : 0.0,
        largest_winning_trade:   0.0,
        largest_losing_trade    : 0.0,
        avg_ticks_in_winning_trades : 0.0,
        avg_ticks_in_losing_trades  : 0.0,
    };

    let asset_trade_scale = sim_params.get_asset_trade_scale();
    let funds_trade_scale = sim_params.get_funds_trade_scale();

    let exchange_fee : Option<Decimal> = sim_params.exchange_fee.map(|v| Decimal::from_f64(v).unwrap());
    let mut funds = Decimal::from_f64(sim_params.initial_capital).unwrap();

    pnl.buy_and_hold_return = buy_and_hold_return(
        &funds,
        &exchange_fee,
        &Decimal::from_f64(sim_params.signals.get(0).unwrap().price_open).unwrap(),
        &Decimal::from_f64(sim_params.signals.last().unwrap().price_close).unwrap(),
        &sim_params.asset_scale,
        &sim_params.funds_scale,
        &funds_trade_scale,
        &asset_trade_scale,
    );

    let mut position_open   : bool  = false;
    let mut simulate_buy    : bool  = false;
    let mut simulate_sell   : bool  = false;

    let mut asset_init_cost = dec!(0.0);
    let mut assets:Decimal = dec!(0.0);

    let mut commission_paid = dec!(0.0);

    let mut tik_at_purchase:u16 = 0;
    let mut gross_profit = dec!(0.0);

    let mut winning_trades:Vec<Decimal> = vec![];

    let mut winning_ticks:Vec<u16> = vec![];
    let mut loosing_ticks:Vec<u16> = vec![];

    let mut gross_loss = dec!(0.0);
    let mut losing_trades:Vec<Decimal> = vec![];

    let zero_val = dec!(0.0);
    let min_funds = dec!(10.0);

    let sim_stop_at = sim_params.signals.len() -1;

    for (indx,tick) in sim_params.signals.iter().enumerate() {
        if simulate_buy {
            let purchase = stage_purchase(
                &funds,
                &Decimal::from_f64(tick.price_open).unwrap(),
                &exchange_fee,
                &sim_params.asset_scale,
                &sim_params.funds_scale,
                &funds_trade_scale,
            );

            asset_init_cost     = purchase.total_fee.map_or(purchase.cost_before_fee, |fee| purchase.cost_before_fee + fee);
            funds   -= asset_init_cost;
            assets  += purchase.asset_qty;

            if let Some(fee) = purchase.total_fee { commission_paid += fee; }

            position_open   = true;
            simulate_buy    = false;

            tik_at_purchase = indx as u16;
        }

        else if simulate_sell || (indx == sim_stop_at && position_open)  {
            let tik_price_open = Decimal::from_f64(tick.price_open).unwrap();
            let sell = stage_sale(
                &assets,
                &tik_price_open,
                &exchange_fee,
                &sim_params.asset_scale,
                &sim_params.funds_scale,
                &asset_trade_scale,
            );
            funds   += sell.sale_before_fee;
            assets  -= sell.assets_sold;

            let mut trade_profit = sell.sale_before_fee - asset_init_cost;

            if let Some(fee) = sell.fee_asset_total {
                let commission_cost = fee * tik_price_open;

                commission_paid += commission_cost;
                assets  -= fee;
                trade_profit -= commission_cost;
            }

            pnl.total_closed_trades += 1;

            #[allow(clippy::comparison_chain)]
            if trade_profit > zero_val {
                gross_profit += trade_profit;
                pnl.num_winning_trades +=1;
                winning_trades.push(trade_profit);
                winning_ticks.push(indx as u16 - tik_at_purchase);
            }
            else if trade_profit < zero_val {
                gross_loss += trade_profit;
                pnl.num_losing_trades +=1;
                losing_trades.push(trade_profit);
                loosing_ticks.push(indx as u16 - tik_at_purchase);
            }

            if trade_profit != zero_val {
                position_open = false;
                simulate_sell = false;
            }

            if funds < min_funds { break; };
        }

        if tick.signal_in > tick.signal_out && !position_open {
            simulate_buy = true;
        } else if tick.signal_in < tick.signal_out && position_open {
            simulate_sell = true;
        }
    }

    pnl.net_profit = (gross_profit + gross_loss).to_f64().unwrap();
    pnl.commission_paid = commission_paid.to_f64();
    pnl.gross_profit = gross_profit.to_f64().unwrap();
    pnl.gross_loss = gross_loss.to_f64().unwrap();

    let percentage = Decimal::from_i32(pnl.num_winning_trades).unwrap() / Decimal::from_i32(pnl.total_closed_trades).unwrap() * dec!(100.0);
    pnl.percent_profitable = percentage.round_dp(2).to_f64().unwrap();

    pnl.avg_winning_trade = array_of_decimal_avg(&winning_trades);
    pnl.avg_losing_trade = array_of_decimal_avg(&losing_trades);

    if pnl.avg_losing_trade != 0.0 {
        let avg_winning_trade = Decimal::from_f64(pnl.avg_winning_trade).unwrap();
        let avg_losing_trade = Decimal::from_f64(pnl.avg_losing_trade).unwrap().abs();
        pnl.ratio_avg_win_loss = (avg_winning_trade / avg_losing_trade).round_dp(3).to_f64().unwrap();
    }

    if let Some(&max) = winning_trades.iter().max_by(|a, b| a.partial_cmp(b).unwrap()) {
        pnl.largest_winning_trade = max.round_dp(2).to_f64().unwrap();
    }

    if let Some(&min) = losing_trades.iter().min_by(|a, b| a.partial_cmp(b).unwrap()) {
        pnl.largest_losing_trade = min.round_dp(2).to_f64().unwrap();
    }

    let sum_tik_wins:u16 = winning_ticks.iter().sum();
    pnl.avg_ticks_in_winning_trades = sum_tik_wins as f64 / (winning_ticks.len() as f64);

    let sum_tik_losses:u16 = loosing_ticks.iter().sum();
    pnl.avg_ticks_in_losing_trades = sum_tik_losses as f64 / (loosing_ticks.len() as f64);

    pnl.profit_factor = profit_factor(&winning_trades, &losing_trades)
        .unwrap_or(0.0);

    pnl
}

/// Returns the average of a list of `Decimal` values.
/// Note: The following example is illustrative and won't run as a doc-test.
/// # Examples
///
/// ```ignore
/// use rust_decimal::prelude::*;
///
/// let values = vec![Decimal::new(10, 2), Decimal::new(20, 2)];
/// let avg = array_of_decimal_avg(&values);
/// assert_eq!(avg, 15.0);
/// ```
fn array_of_decimal_avg(arr:&Vec<Decimal>) -> f64 {
    if arr.is_empty() { 0.0 }
    else {
        let sum_values = arr.iter().fold(Decimal::from_f64(0.0).unwrap(), |a, b| a + b);
        (sum_values / Decimal::from_usize(arr.len()).unwrap()).round_dp(2).to_f64().unwrap()
    }
}

/// Represents information related to an asset purchase.
///
/// This struct encapsulates details about the quantity of assets purchased, the
/// cost before applying any fees, and the total fees (if any) associated with the simulated purchase.
#[derive(Debug)]
struct PurchaseInfo {
    /// The quantity of assets purchased.
    asset_qty       : Decimal,
    /// The total cost of the purchase before accounting for fees (if any).
    cost_before_fee : Decimal,
    /// The total fees associated with the purchase, if any.
    total_fee       : Option<Decimal>
}

/// Computes and stages the details for purchasing assets.
///
/// This function calculates the amount of assets that can be purchased for a given
/// amount of funds at a given price, while also considering exchange fees and scale factors.
///
/// # Parameters
///
/// * `funds`: The amount of funds available for purchasing assets.
/// * `price`: The price of a single unit of the asset.
/// * `exchange_fee`: The optional exchange fee that is applied to the purchase.
/// * `asset_scale`: The scale (number of decimal places) to use when truncating the asset quantity.
/// * `funds_scale`: The scale to use when truncating the funds value.
/// * `funds_trade_scale`: Optional scale factor for truncating the funds value when trading.
fn stage_purchase(
    funds           : &Decimal,
    price           : &Decimal,
    exchange_fee    : &Option<Decimal>,

    asset_scale     : &u32,
    funds_scale     : &u32,

    funds_trade_scale   : &Option<u32>,
) -> PurchaseInfo {
    // Determine the available funds after accounting for potential exchange fees
    let funds_available = exchange_fee
        .map_or(*funds, |fee| (funds - (funds * fee)).trunc_with_scale(*funds_scale));

    // Calculate the quantity of assets that can be purchased with the available funds
    let mut asset_qty = (funds_available / price).trunc_with_scale(*asset_scale);

    let mut cost_before_fee = asset_qty * price;

    // Adjust the cost and asset quantity based on the trade scale, if provided
    if let Some(scale) = funds_trade_scale {
        cost_before_fee = cost_before_fee.trunc_with_scale(*scale);
        asset_qty = (cost_before_fee / price).trunc_with_scale(*asset_scale);
    }

    // Calculate the total fees, if any
    let total_fee = exchange_fee.map_or(None, |fee| Some(cost_before_fee * fee));

    PurchaseInfo { asset_qty, cost_before_fee, total_fee, }
}

/// Represents information related to an asset sale.
///
/// This struct encapsulates details about the quantity of assets sold, the
/// proceeds from the sale before applying any fees, and the total fees (if any)
/// deducted from the asset quantity before the sale.
#[derive(Debug)]
struct SaleInfo {
    /// The quantity of assets sold.
    assets_sold     : Decimal,
    /// The proceeds from the sale before accounting for fees.
    sale_before_fee : Decimal,
    /// The total quantity of assets deducted as fees, if applicable.
    fee_asset_total : Option<Decimal>,
}
/// Computes and stages the details for selling assets.
///
/// This function calculates the net proceeds from selling a certain quantity of
/// assets at a given price, while also considering exchange fees and scale factors.
///
/// # Parameters
///
/// * `asset_qty`: The quantity of assets to be sold.
/// * `price`: The price at which each asset unit will be sold.
/// * `exchange_fee`: The optional exchange fee deducted from the asset quantity before the sale.
/// * `asset_scale`: The scale (number of decimal places) to use when truncating the asset quantity.
/// * `funds_scale`: The scale to use when truncating the proceeds from the sale.
/// * `asset_trade_scale`: Optional scale factor for truncating the asset quantity when trading.
fn stage_sale(
    asset_qty   : &Decimal,
    price       : &Decimal,
    exchange_fee: &Option<Decimal>,

    asset_scale     : &u32,
    funds_scale     : &u32,

    asset_trade_scale   : &Option<u32>,
) -> SaleInfo {
    // Calculate the net quantity of assets to be sold after accounting for potential exchange fees
    let mut assets_sold = exchange_fee
        .map_or(*asset_qty, |fee| (asset_qty - (asset_qty * fee)).trunc_with_scale(*asset_scale));

    // Adjust the assets quantity based on the trade scale, if provided
    if let Some(trade_scale) = asset_trade_scale {
        assets_sold = assets_sold.trunc_with_scale(*trade_scale);
    }

    // Calculate the proceeds from the sale before fees
    let sale_before_fee = (assets_sold * price).trunc_with_scale(*funds_scale);

    // Determine the total asset quantity deducted as fees, if any
    let fee_asset_total = exchange_fee.map_or(None, |fee| Some(assets_sold * fee));

    SaleInfo {assets_sold, sale_before_fee, fee_asset_total}
}

/// Calculates the return from a buy-and-hold trading strategy.
///
/// This function computes the net return of buying an asset at an entry price
/// and selling it at an exit price, taking into account potential fees and scales.
///
/// # Parameters
/// - `funds`: The initial funds available for purchasing.
/// - `exchange_fee`: The optional fee incurred during the transaction.
/// - `price_entry`: The price at which the asset is purchased.
/// - `price_exit`: The price at which the asset is sold.
/// - `asset_scale`: The scale (precision) for the asset quantity.
/// - `funds_scale`: The scale (precision) for the funds.
/// - `funds_trade_scale`: The optional trade scale for funds.
/// - `asset_trade_scale`: The optional trade scale for the asset.
///
/// # Returns
/// The net return from the buy-and-hold strategy, rounded to two decimal places.
#[allow(clippy::too_many_arguments)]
fn buy_and_hold_return(
    funds       : &Decimal,
    exchange_fee: &Option<Decimal>,
    price_entry : &Decimal,
    price_exit  : &Decimal,

    asset_scale     : &u32,
    funds_scale     : &u32,

    funds_trade_scale   : &Option<u32>,
    asset_trade_scale   : &Option<u32>,
) -> f64 {

    let purchase = stage_purchase(
        funds,
        price_entry,
        exchange_fee,
        asset_scale,
        funds_scale,
        funds_trade_scale,
    );
    let mut position = funds - purchase.cost_before_fee;
    if let Some(fee) = purchase.total_fee { position -= fee; }


    let sale = stage_sale(
        &purchase.asset_qty,
        price_exit,
        exchange_fee,
        asset_scale,
        funds_scale,
        asset_trade_scale,
    );

    position += sale.sale_before_fee;
    if let Some(fee) = sale.fee_asset_total {
        position -= (fee * price_exit).trunc_with_scale(*funds_scale);
    }

    position += ((purchase.asset_qty - sale.assets_sold) * price_exit).trunc_with_scale(*funds_scale);

    (position - funds).round_dp(2).to_f64().unwrap()
}

/// Calculates the profit factor of a set of trades.
///
/// The profit factor is the ratio of the total profit to the total loss. It is a measure of
/// a trading strategy's effectiveness. A profit factor greater than 1 indicates a profitable
/// strategy, whereas a value less than 1 suggests a losing strategy.
///
/// # Parameters
/// - `profitable_trades`: A list of profit values from profitable trades.
/// - `losing_trades`: A list of loss values from losing trades.
///
/// # Returns
/// An `Option` containing the profit factor rounded to three decimal places.
/// If the total loss is effectively zero (close to machine epsilon), it returns `None`.
fn profit_factor(profitable_trades: &[Decimal], losing_trades: &[Decimal]) -> Option<f64> {
    let total_profit = profitable_trades.iter().fold(Decimal::from_f64(0.0).unwrap(), |a, b| a + b);
    let total_loss = losing_trades.iter().fold(Decimal::from_f64(0.0).unwrap(), |a, b| a + b);

    if total_loss.abs() > Decimal::try_from(f64::EPSILON).unwrap() { // Check if total_loss is not effectively zero
        Some((total_profit / total_loss.abs()).round_dp(3).to_f64().unwrap())
    } else {
        None
    }
}