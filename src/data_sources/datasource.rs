use anyhow::Result;

use crate::models::{interval::Interval, timeseries::TimeSeries};

use super::{local, alphavantage, bitfinex, bybit, coinmarketcap, cryptocompare};


#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataSource {
    AlphaVantage,
    CoinMarketCap,
    Bitfinex,
    Bybit,
    CryptoCompare(Option<String>),
    Local(Box<DataSource>),
}

impl DataSource {
    pub async fn request_data(
        &self,
        symbol: &str,
        interval: Interval,
        save_local: bool,
    ) -> Result<TimeSeries> {
        let ts: TimeSeries;

        // Attempt local retrieval if possible
        if let DataSource::Local(s) = self {
            let result = local::read(&s, &symbol, &interval).await;

            match result {
                Ok(ts) => return Ok(ts),
                _ => {}
            }
        }

        ts = match self {
            DataSource::AlphaVantage => alphavantage::get(symbol, &interval).await?,
            DataSource::Bitfinex => bitfinex::rest::get(symbol, &interval).await?,
            DataSource::Bybit => bybit::rest::get(symbol, &interval).await?,
            DataSource::CoinMarketCap => coinmarketcap::get().await?,
            DataSource::CryptoCompare(exchange) => {
                cryptocompare::get(symbol, &interval, exchange.clone()).await?
            }
            _ => panic!("Error"),
        };

        if save_local {
            match self {
                DataSource::Local(_) => (),
                _ => local::write(&ts, &self).await?,
            }
        }

        Ok(ts)
    }
}