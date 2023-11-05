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
}

impl DataSource {
    #[allow(dead_code)]
    pub async fn load_local_data(&self, symbol: &str, interval: &Interval) -> Result<TimeSeries> {
        local::read(self, symbol, interval).await
    }

    pub async fn get_historical_data(
        &self,
        symbol: &str,
        interval: &Interval,
        len: usize,
    ) -> Result<TimeSeries> {
        let ts = match self {
            DataSource::AlphaVantage => alphavantage::get(symbol, &interval).await?,
            DataSource::Bitfinex => bitfinex::rest::get(symbol, &interval).await?,
            DataSource::Bybit => bybit::rest::get(symbol, &interval, len).await?,
            DataSource::CoinMarketCap => coinmarketcap::get().await?,
            DataSource::CryptoCompare(exchange) => {
                cryptocompare::get(symbol, &interval, exchange.clone()).await?
            }
        };

        Ok(ts)
    }

    pub async fn connect_ws(
        &self,
        client: Addr<WebsocketClient>,
        interval: Interval,
    ) -> Result<()> {
        match self {
            DataSource::Bitfinex => bitfinex::ws::connect_ws(&client, &interval).await?,
            DataSource::Bybit => {
                let mut api = BybitWebsocketApi::new(&client, interval);
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
        }
    }
}
