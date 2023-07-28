use crate::models::{generic_result::GenericResult, interval::Interval, timeseries::TimeSeries};

mod alphavantage;
mod coinmarketcap;
mod local;

// Available data sources
#[allow(dead_code)] 
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataSource {
    AlphaVantage,
    CoinMarketCap,
    Local(Box<DataSource>),
}

pub async fn request_data(
    source: &DataSource,
    symbol: &str,
    interval: Interval,
    save_local: bool,
) -> GenericResult<TimeSeries> {
    let ts: TimeSeries = match source {
        DataSource::AlphaVantage => alphavantage::get(symbol, &interval).await?,
        DataSource::CoinMarketCap => coinmarketcap::get().await?,
        DataSource::Local(source) => local::read(&source, &symbol, &interval).await?,
    };

    if save_local {
        match source {
            DataSource::Local(_) => (),
            _ => local::write(&ts, &source).await?,
        }
    }

    Ok(ts)
}
