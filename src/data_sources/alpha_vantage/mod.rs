mod structs;

use crate::utils::generic_result::GenericResult;
use crate::utils::timeseries::{Interval, TimeSeries};
use reqwest;
use std::env;
use structs::AlphaVantageApiResponse;

pub async fn get(symbol: &str, interval: Interval) -> GenericResult<TimeSeries> {
    let function = "DIGITAL_CURRENCY_DAILY";
    let url = construct_url(function, symbol, interval);

    let response = reqwest::get(url).await?;

    match response.status() {
        reqwest::StatusCode::OK => convert_data(response).await,
        _ => Err("Request failed.".into()),
    }
}

async fn convert_data(res: reqwest::Response) -> GenericResult<TimeSeries> {
    let mut alpha_vantage_data: AlphaVantageApiResponse = res.json().await?;

    let timeseries = alpha_vantage_data.to_timeseries(Interval::Daily);

    timeseries.map(|ts| ts)
}

fn construct_url(function: &str, symbol: &str, interval: Interval) -> String {
    //TODO: Implement different intervals
    
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
