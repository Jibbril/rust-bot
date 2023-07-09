mod calculation;
mod data_sources;
mod utils;

use calculation::indicators::{rsi::RSI, sma::SMA, PopulatesCandles};
use data_sources::{request_data, DataSource};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Get TimeSeries data
    let mut ts = request_data(DataSource::AlphaVantage, "BTC").await?;

    // Calculate indicator data for TimeSeries
    let _ = SMA::populate_candles(&mut ts.candles, 7);
    let _ = RSI::populate_candles(&mut ts.candles, 14);

    let segment = &ts.candles[0..20];

    println!("Populated candles: {:#?}", segment);

    // TODO: Implement Strategy to analyze TimeSeries

    Ok(())
}
