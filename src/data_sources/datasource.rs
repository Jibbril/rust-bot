use super::{
    alphavantage, bitfinex,
    bybit::{self, ws::bybit_ws_api::BybitWebsocketApi},
    coinmarketcap, cryptocompare, local,
};
use crate::models::{
    interval::Interval, timeseries::TimeSeries, websockets::wsclient::WebsocketClient,
};
use actix::Addr;
use anyhow::{anyhow, Result};
use std::fmt::{Display, Formatter};

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
            _ => unreachable!(),
        };

        if save_local {
            match self {
                DataSource::Local(_) => (),
                _ => local::write(&ts, &self).await?,
            }
        }

        Ok(ts)
    }

    pub async fn connect_ws(&self, client: Addr<WebsocketClient>) -> Result<()> {
        match self {
            DataSource::Bitfinex => bitfinex::ws::connect_ws(&client).await?,
            DataSource::Bybit => {
                let api = BybitWebsocketApi::new(&client);
                api.connect().await?
            }
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
