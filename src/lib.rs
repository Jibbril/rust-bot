mod data_sources;
mod indicators;
mod models;
mod notifications;
mod resolution_strategies;
mod trading_strategies;
mod utils;

use crate::{models::interval::Interval, trading_strategies::strategy::Strategy};
use data_sources::{request_data, DataSource};
use dotenv::dotenv;
use notifications::notify;

use crate::{
    indicators::{atr::ATR, rsi::RSI, sma::SMA, PopulatesCandles},
    trading_strategies::{rsi_basic::RsiBasic, setup::FindsSetups},
};

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

    println!("Found {} setups!", setups.len());
    for setup in setups.iter() {
        println!("{:#?}", setup.clone());
    }

    notify(&setups[0], &strategy).await?;

    Ok(())
}
