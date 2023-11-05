use anyhow::{anyhow, Result};
use reqwest::Client;

use crate::{
    data_sources::api_response::ApiResponse,
    models::{interval::Interval, timeseries::TimeSeries},
};

use super::bybit_structs::BybitApiResponse;

pub async fn get(symbol: &str, interval: &Interval, len: usize) -> Result<TimeSeries> {
    let url = generate_url(symbol, interval, len)?;

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

fn generate_url(symbol: &str, interval: &Interval, len: usize) -> Result<String> {
    let interval = match interval {
        Interval::Minute1 => "1",
        Interval::Minute5 => "5",
        Interval::Minute15 => "15",
        Interval::Minute30 => "30",
        Interval::Hour1 => "60",
        Interval::Hour4 => "240",
        Interval::Day1 => "D",
        Interval::Week1 => "W",
        // Interval::Month1 => "1M",
        _ => return Err(anyhow!("Bybit does not support this interval.")),
    };

    Ok(format!(
        "https://api.bybit.com/v5/market/kline?category=spot&symbol={}&interval={}&limit={}",
        symbol, interval, len
    ))
}
