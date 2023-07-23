mod data_sources;
mod indicators;
mod models;
mod notifications;
mod resolution_strategies;
mod strategy_testing;
mod trading_strategies;
mod utils;

use crate::{
    indicators::{atr::ATR, rsi::RSI, sma::SMA, PopulatesCandles},
    models::interval::Interval,
    strategy_testing::test_setups,
    trading_strategies::strategy::Strategy,
    trading_strategies::{
        rsi_basic::RsiBasic,
        setup::{FindsReverseSetups, FindsSetups},
    },
};
use data_sources::{request_data, DataSource};
use dotenv::dotenv;
use notifications::notify;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Get TimeSeries data
    let source = DataSource::AlphaVantage;
    let source = DataSource::Local(Box::new(source));
    let interval = Interval::Daily;
    let mut ts = request_data(&source, "BTC", interval, true).await?;

    // Calculate indicator data for TimeSeries
    SMA::populate_candles(&mut ts.candles, 7)?;
    RSI::populate_candles(&mut ts.candles, 14)?;
    ATR::populate_candles(&mut ts.candles, 14)?;

    // Implement Strategy to analyze TimeSeries
    let strategy = Strategy::RsiBasic(RsiBasic::new_default());
    let setups = strategy.find_setups(&ts)?;
    let reverse_setups = strategy.find_reverse_setups(&ts)?;

    // Send email notifications
    if false {
        notify(&setups[0], &strategy).await?;
    }

    // Test result of taking setups
    let results = test_setups(&setups, &ts.candles);
    let reverse_results = test_setups(&reverse_setups, &ts.candles);

    println!("Results:{:#?}", results);
    println!("Reverse results:{:#?}", reverse_results);

    Ok(())
}
