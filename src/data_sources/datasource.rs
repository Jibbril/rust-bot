use super::{
    alphavantage, bitfinex,
    bybit::{ws::bybit_ws_api::BybitWebsocketApi, rest::bybit_rest_api::BybitRestApi},
    coinmarketcap, cryptocompare, local,
};
use crate::models::{
    interval::Interval, net_version::NetVersion, timeseries::TimeSeries,
    websockets::wsclient::WebsocketClient,
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
        net: &NetVersion,
    ) -> Result<TimeSeries> {
        let ts = match self {
            DataSource::AlphaVantage => alphavantage::get(symbol, &interval).await?,
            DataSource::Bitfinex => bitfinex::rest::get(symbol, &interval).await?,
            DataSource::Bybit => BybitRestApi::get_kline(symbol, &interval, len, net).await?, 
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
        net: &NetVersion,
    ) -> Result<()> {
        match self {
            DataSource::Bitfinex => bitfinex::ws::connect_ws(&client, &interval).await?,
            DataSource::Bybit => {
                let mut api = BybitWebsocketApi::new(&client, interval);
                api.connect(net).await?
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
