use anyhow::Result;
use crate::models::{timeseries::TimeSeries, net_version::NetVersion, interval::Interval, candle::Candle, wallet::Wallet};
use super::{kline, order_create, server_time, wallet_balance};

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

    pub async fn market_buy(quantity: f64, net: &NetVersion) -> Result<()> {
        Ok(order_create::market_buy(quantity, net).await?)
    }

    pub async fn market_sell_all(
        account_info: &Wallet, 
        net: &NetVersion
    ) -> Result<()> {
        Ok(order_create::market_sell_all(account_info, net).await?)
    }

    pub async fn get_server_time(net: &NetVersion) -> Result<u64> {
        Ok(server_time::get_server_time(net).await?)
    }

    pub async fn get_wallet_balance(net: &NetVersion) -> Result<Wallet> {
        Ok(wallet_balance::get(net).await?)
    }
}
