mod cryptocompare_structs;
pub mod websockets;

use self::cryptocompare_structs::CryptoCompareApiResponse;
use super::api_response::ApiResponse;
use crate::models::{interval::Interval, timeseries::TimeSeries};
use anyhow::{anyhow, Result};
use reqwest::Client;
use std::env;

pub async fn get(
    symbol: &str,
    interval: &Interval,
    exchange: Option<String>,
) -> Result<TimeSeries> {
    let api_key = env::var("CRYPTOCOMPARE_KEY")?;
    let url = construct_url(symbol, interval, 2000, exchange);

    let client = Client::new();
    let response = client
        .get(url)
        .header("Authorization", format!("Apikey {}", api_key))
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let mut response: CryptoCompareApiResponse = response.json().await?;
            let ts = response.to_timeseries(symbol, interval);
            ts.map(|ts| ts)
        }
        _ => Err(anyhow!("CryptoCompare request failed.")),
    }
}

fn construct_url(
    symbol: &str,
    interval: &Interval,
    limit: u32,
    exchange: Option<String>,
) -> String {
    let market = "USD";
    let minute = "histominute";
    let hour = "histohour";
    let day = "histoday";

    let exchange = exchange.unwrap_or("CCCAGG".to_string());

    let (interval, aggregate) = match interval {
        Interval::Minute1 => (minute, 1),
        Interval::Minute5 => (minute, 5),
        Interval::Minute15 => (minute, 15),
        Interval::Minute30 => (minute, 30),
        Interval::Hour1 => (hour, 1),
        Interval::Hour4 => (hour, 4),
        Interval::Hour12 => (hour, 12),
        Interval::Day1 => (day, 1),
        Interval::Day5 => (day, 5),
        Interval::Week1 => (day, 5),
    };

    // TODO: Enable multiples using the aggregate parameter in the api

    format!(
        "https://min-api.cryptocompare.com/data/v2/{}?fsym={}&tsym={}&limit={}&aggregate={}&e={}",
        interval, symbol, market, limit, aggregate, exchange
    )
}
