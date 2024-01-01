use super::{bybit_structs::BybitApiResponse, util::interval_to_str};
use crate::{
    data_sources::api_response::ApiResponse,
    models::{interval::Interval, net_version::NetVersion, timeseries::TimeSeries},
};
use anyhow::{anyhow, Result};
use reqwest::Client;

pub async fn get(
    symbol: &str,
    interval: &Interval,
    len: usize,
    net: &NetVersion,
) -> Result<TimeSeries> {
    let url = generate_url(symbol, interval, len, net)?;

    let client = Client::new();
    let response = client.get(url).send().await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let mut response: BybitApiResponse = response.json().await?;

            let ts = response.to_timeseries(symbol, interval);
            ts.map(|ts| ts)
        }
        _ => Err(anyhow!("Bybit request failed.")),
    }
}

fn generate_url(symbol: &str, interval: &Interval, len: usize, net: &NetVersion) -> Result<String> {
    let interval = interval_to_str(interval)?;
    let base = match net {
        NetVersion::Testnet => "https://api-testnet.bybit.com",
        NetVersion::Mainnet => "https://api.bybit.com",
    };

    Ok(format!(
        "{}/v5/market/kline?category=spot&symbol={}&interval={}&limit={}",
        base, symbol, interval, len
    ))
}
