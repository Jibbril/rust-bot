use crate::{
    data_sources::{
        alphavantage, bitfinex,
        bybit::{rest::bybit_rest_api::BybitRestApi, ws::bybit_ws_api::BybitWebsocketApi},
        coinmarketcap, cryptocompare, local,
    },
    models::{
        candle::Candle, interval::Interval, message_payloads::websocket_payload::WebsocketPayload,
        net_version::NetVersion, timeseries::TimeSeries, wallet::Wallet,
        websockets::wsclient::WebsocketClient,
    },
};
use actix::{spawn, Addr};
use anyhow::{anyhow, Result};
use std::fmt::{Display, Formatter};
use tokio::time::{sleep, Duration};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataSource {
    AlphaVantage,
    CoinMarketCap,
    Bitfinex,
    Bybit,
    CryptoCompare(Option<String>),
    Dummy(u64), // Milliseconds
}

impl DataSource {
    #[allow(dead_code)]
    pub async fn load_local_data(&self, symbol: &str, interval: &Interval) -> Result<TimeSeries> {
        local::read(self, symbol, interval).await
    }

    pub async fn enter_trade(&self, symbol: &str, quantity: f64) -> Result<()> {
        match self {
            DataSource::Bybit => BybitRestApi::market_buy(symbol, quantity).await,
            DataSource::Dummy(_) => BybitRestApi::market_buy(symbol, quantity).await,
            _ => Err(anyhow!(format!(
                "{} does not support entering positions yet",
                self
            ))),
        }
    }

    pub async fn exit_trade(&self, symbol: &str, quantity: f64) -> Result<()> {
        match self {
            DataSource::Bybit => BybitRestApi::market_sell(symbol, quantity).await,
            DataSource::Dummy(_) => BybitRestApi::market_sell(symbol, quantity).await,
            _ => Err(anyhow!(format!(
                "{} does not support exiting positions yet",
                self
            ))),
        }
    }

    pub async fn get_wallet(&self) -> Result<Wallet> {
        match self {
            DataSource::Dummy(_) => BybitRestApi::get_wallet_balance().await,
            DataSource::Bybit => BybitRestApi::get_wallet_balance().await,
            _ => Err(anyhow!(format!(
                "{} does not support fetching wallet balance yet",
                self
            ))),
        }
    }

    pub async fn get_symbol_price(&self, symbol: &str) -> Result<f64> {
        match self {
            DataSource::Dummy(_) => BybitRestApi::get_symbol_price(symbol).await,
            DataSource::Bybit => BybitRestApi::get_symbol_price(symbol).await,
            _ => Err(anyhow!(format!(
                "{} does not support fetching prices yet",
                self
            ))),
        }
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
            DataSource::Dummy(_duration) => {
                let mut ts = TimeSeries::dummy();
                let candles = Candle::dummy_data(len, "alternating", 1000.0);
                ts.set_candles(&candles);
                ts
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
            DataSource::Dummy(d) => spawn_dummy_generator(client, d),
            _ => {
                let err = format!("{} does not support websockets", self);
                return Err(anyhow!(err));
            }
        }

        Ok(())
    }
}

fn spawn_dummy_generator(client: Addr<WebsocketClient>, d: &u64) {
    let duration = *d;

    let fut = async move {
        let mut prev = Candle::dummy_from_val(1000.0);
        loop {
            sleep(Duration::from_millis(duration)).await;
            let candle = Candle::dyn_dummy_from_prev(&prev, Interval::Day1);
            let payload = WebsocketPayload {
                ok: true,
                message: None,
                candle: Some(candle.clone()),
            };
            client.do_send(payload);
            prev = candle;
        }
    };

    spawn(fut);
}

impl Display for DataSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            DataSource::AlphaVantage => write!(f, "AlphaVantage"),
            DataSource::Bitfinex => write!(f, "Bitfinex"),
            DataSource::Bybit => write!(f, "Bybit"),
            DataSource::CoinMarketCap => write!(f, "CoinMarketCap"),
            DataSource::CryptoCompare(_) => write!(f, "CryptoCompare"),
            DataSource::Dummy(_) => write!(f, "Dummy"),
        }
    }
}
