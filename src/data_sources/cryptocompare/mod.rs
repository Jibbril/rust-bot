mod cryptocompare_structs;
use std::env;
use reqwest::Client;
use crate::models::{generic_result::GenericResult, timeseries::TimeSeries, interval::Interval};
use self::cryptocompare_structs::CryptoCompareApiResponse;
use super::ApiResponse;

pub async fn get(symbol: &str, interval: &Interval) -> GenericResult<TimeSeries> {
    let api_key = env::var("CRYPTOCOMPARE_KEY")?;
    let url = construct_url(symbol,interval, 1000);

    let client = Client::new();
    let  response= client
        .get(url)
        .header("Authorization", format!("Apikey {}", api_key))
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let mut response: CryptoCompareApiResponse = response.json().await?;
            let ts = response.to_timeseries(symbol, interval);
            ts.map(|ts| ts)
        },
        _ => Err("CryptoCompare request failed.".into())
    }
}

fn construct_url(symbol: &str, interval: &Interval, limit: u32) -> String {
    let market = "USD";
    let interval = match interval {
        Interval::Daily => "histoday"
    };

    // TODO: Enable multiples using the aggregate parameter in the api

    format!(
        "https://min-api.cryptocompare.com/data/v2/{}?fsym={}&tsym={}&limit={}",
        interval,
        symbol,
        market,
        limit
    )
}