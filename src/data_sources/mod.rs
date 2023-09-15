mod alphavantage;
mod bitfinex;
mod coinmarketcap;
pub mod cryptocompare;
mod local;

use crate::models::{generic_result::GenericResult, interval::Interval, timeseries::TimeSeries};

// Available data sources
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataSource {
    AlphaVantage,
    CoinMarketCap,
    Bitfinex,
    CryptoCompare(Option<String>),
    Local(Box<DataSource>),
}

pub trait ApiResponse {
    fn to_timeseries(&mut self, symbol: &str, interval: &Interval) -> GenericResult<TimeSeries>;
}

pub async fn request_data(
    source: &DataSource,
    symbol: &str,
    interval: Interval,
    save_local: bool,
) -> GenericResult<TimeSeries> {
    let ts: TimeSeries;

    let mut source = source.clone();

    // Attempt local retrieval if possible
    if let DataSource::Local(s) = source {
        let result = local::read(&s, &symbol, &interval).await;

        match result {
            Ok(ts) => return Ok(ts),
            _ => source = *s,
        }
    }

    ts = match &source {
        DataSource::AlphaVantage => alphavantage::get(symbol, &interval).await?,
        DataSource::Bitfinex => bitfinex::rest::get(symbol, &interval).await?,
        DataSource::CoinMarketCap => coinmarketcap::get().await?,
        DataSource::CryptoCompare(exchange) => {
            cryptocompare::get(symbol, &interval, exchange.clone()).await?
        }
        _ => panic!("Error"),
    };

    if save_local {
        match source {
            DataSource::Local(_) => (),
            _ => local::write(&ts, &source).await?,
        }
    }

    Ok(ts)
}
