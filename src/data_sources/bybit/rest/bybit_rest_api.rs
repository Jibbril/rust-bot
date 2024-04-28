use crate::{
    data_sources::bybit::rest::{tickers, kline, order_create, server_time, wallet_balance},
    models::{
        candle::Candle, interval::Interval, net_version::NetVersion, timeseries::TimeSeries,
        wallet::Wallet,
    },
};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct BybitRestApi;

impl BybitRestApi {
    pub async fn get_kline(
        symbol: &str,
        interval: &Interval,
        len: usize,
        net: &NetVersion,
    ) -> Result<TimeSeries> {
        Ok(kline::get(symbol, interval, len, net).await?)
    }

    pub async fn get_kline_between(
        symbol: &str,
        interval: &Interval,
        net: &NetVersion,
        from: i64,
        to: i64,
    ) -> Result<Vec<Candle>> {
        Ok(kline::get_candles_between(symbol, interval, net, from, to).await?)
    }

    pub async fn market_buy(quantity: f64) -> Result<()> {
        Ok(order_create::market_buy(quantity).await?)
    }

    pub async fn market_sell(symbol: &str, quantity: f64) -> Result<()> {
        Ok(order_create::market_sell(symbol, quantity).await?)
    }

    pub async fn market_sell_all(account_info: &Wallet) -> Result<()> {
        Ok(order_create::market_sell_all(account_info).await?)
    }

    pub async fn get_server_time() -> Result<u64> {
        Ok(server_time::get_server_time().await?)
    }

    pub async fn get_symbol_price(symbol: &str) -> Result<f64> {
        Ok(tickers::get_symbol_price(symbol).await?)
    }

    pub async fn get_wallet_balance() -> Result<Wallet> {
        Ok(wallet_balance::get().await?)
    }
}
