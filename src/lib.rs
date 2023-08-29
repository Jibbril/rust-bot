mod data_sources;
mod indicators;
mod models;
mod notifications;
mod resolution_strategies;
mod strategy_testing;
mod trading_strategies;
mod utils;

use crate::{
    indicators::{atr::ATR, bbwp::BBWP, populates_candles::PopulatesCandles, rsi::RSI},
    models::{interval::Interval, setup::FindsReverseSetups, strategy::Strategy},
    strategy_testing::test_setups,
    trading_strategies::rsi_basic::RsiBasic,
    utils::save_setups,
};
use data_sources::{request_data, DataSource};
use dotenv::dotenv;
use notifications::notify;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Get TimeSeries data
    let source = DataSource::CryptoCompare(Some("Coinbase".to_string()));
    let source = DataSource::Local(Box::new(source));
    let interval = Interval::Day1;
    let mut ts = request_data(&source, "BTC", interval, true).await?;

    // Calculate indicators for TimeSeries
    // SMA::populate_candles_default(&mut ts.candles)?;
    // SMA::populate_candles_default(&mut ts.candles)?;
    // SMA::populate_candles_default(&mut ts.candles)?;
    // BollingerBands::populate_candles_default(&mut ts.candles)?;
    // DynamicPivot::populate_candles_default(&mut ts.candles)?;
    // BBW::populate_candles_default(&mut ts)?;
    BBWP::populate_candles_default(&mut ts)?;
    RSI::populate_candles_default(&mut ts)?;
    ATR::populate_candles_default(&mut ts)?;

    println!("Candles:{:#?}", ts.candles);

    // Implement Strategy to analyze TimeSeries
    let rsi_strategy = Strategy::RsiBasic(RsiBasic::new_default());

    let rsi_setups = rsi_strategy.find_reverse_setups(&ts)?;

    save_setups(&rsi_setups, "rsi-setups.csv")?;

    // Send email notifications
    if false {
        notify(&rsi_setups[0], &rsi_strategy).await?;
    }

    // Test result of taking setups
    let _ = test_setups(&rsi_setups, &ts.candles);

    // println!("RSI results:{:#?}", rsi_results);

    Ok(())
}
