use crate::models::{
    interval::Interval, timeseries::TimeSeries, websockets::wsclient::WebsocketClient,
};
use anyhow::{anyhow, Result};
use std::fmt::{Display, Formatter};

use super::{alphavantage, bitfinex, bybit, coinmarketcap, cryptocompare, local};

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
    pub async fn get_historical_data(
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

    pub async fn connect_ws(&self, client: &WebsocketClient) -> Result<()> {
        match self {
            DataSource::Bitfinex => bitfinex::ws::connect_ws(&client).await?,
            DataSource::Bybit => bybit::ws::connect_ws(&client).await?,
            _ => {
                let err = format!("{} does not support websockets", self);
                return Err(anyhow!(err));
            }
        }

        Ok(())
    }
}

impl Display for DataSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            DataSource::AlphaVantage => write!(f, "AlphaVantage"),
            DataSource::Bitfinex => write!(f, "Bitfinex"),
            DataSource::Bybit => write!(f, "Bybit"),
            DataSource::CoinMarketCap => write!(f, "CoinMarketCap"),
            DataSource::CryptoCompare(_) => write!(f, "CryptoCompare"),
            DataSource::Local(_) => write!(f, "Local"),
        }
    }
}
