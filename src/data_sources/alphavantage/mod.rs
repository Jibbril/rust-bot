mod alphavantage_structs;

use super::api_response::ApiResponse;
use crate::models::interval::Interval;
use crate::models::timeseries::TimeSeries;
use alphavantage_structs::AlphaVantageApiResponse;
use anyhow::{anyhow, Result};
use reqwest;
use std::env;

pub async fn get(symbol: &str, interval: &Interval) -> Result<TimeSeries> {
    let function = "DIGITAL_CURRENCY_DAILY";
    let url = construct_url(function, symbol, interval);

    let response = reqwest::get(url).await?;

    match response.status() {
        reqwest::StatusCode::OK => convert_data(symbol, response, interval).await,
        _ => Err(anyhow!("Request failed.")),
    }
}

async fn convert_data(
    symbol: &str,
    res: reqwest::Response,
    interval: &Interval,
) -> Result<TimeSeries> {
    let mut alpha_vantage_data: AlphaVantageApiResponse = res.json().await?;

    let timeseries = alpha_vantage_data.to_timeseries(symbol, interval);

    timeseries.map(|ts| ts)
}

fn construct_url(function: &str, symbol: &str, interval: &Interval) -> String {
    //TODO: Implement different intervals
    match interval {
        Interval::Day1 => (),
        _ => panic!("{} interval not supported by Alpha Vantage.", interval),
    }

    let market = "USD";
    let key = env::var("ALPHA_VANTAGE_KEY");

    if let Ok(key) = key {
        format!(
            "https://www.alphavantage.co/query?function={}&symbol={}&market={}&apikey={}",
            function, symbol, market, key
        )
    } else {
        panic!("Unable to read Alpha Vantage API key.");
    }
}
