mod calculation;
mod data_sources;
mod utils;

use calculation::{
    indicators::{rsi::RSI, sma::SMA, PopulatesCandles},
    trading_strategies::{rsi_basic::RsiBasic, setup::FindsSetups},
};
use data_sources::{request_data, DataSource};
use dotenv::dotenv;
use utils::timeseries::Interval;

use crate::calculation::indicators::atr::ATR;

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
    let rsi_strategy = RsiBasic::new_default();

    let setups = rsi_strategy.find_setups(&mut ts)?;

    println!("Found {} setups!", setups.len());
    for setup in setups.iter() {
        println!("{:#?}", setup.clone());
    }

    Ok(())
}
