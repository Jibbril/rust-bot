mod calculation;
mod data_sources;
mod utils;

use calculation::{indicators::{rsi::RSI, sma::SMA, PopulatesCandles}, strategies::{rsi_basic::RsiBasic, FindsSetups}};
use data_sources::{request_data, DataSource};
use dotenv::dotenv;


pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Get TimeSeries data
    let mut ts = request_data(DataSource::AlphaVantage, "BTC").await?;

    // Calculate indicator data for TimeSeries
    let _ = SMA::populate_candles(&mut ts.candles, 7);
    let _ = RSI::populate_candles(&mut ts.candles, 14);

    // Implement Strategy to analyze TimeSeries
    let rsi_strategy = RsiBasic::new_default();
    
    let res = rsi_strategy.find_setups(&mut ts);

    res.map(|setups| {
        println!("Found {} setups!", setups.len());

        for setup in setups.iter() {
            println!("{:#?}",setup.clone());
        }
    })?;

    Ok(())
}
