mod alphavantage_structs;

use super::ApiResponse;
use crate::models::generic_result::GenericResult;
use crate::models::interval::Interval;
use crate::models::timeseries::TimeSeries;
use alphavantage_structs::AlphaVantageApiResponse;
use reqwest;
use std::env;

pub async fn get(symbol: &str, interval: &Interval) -> GenericResult<TimeSeries> {
    let function = "DIGITAL_CURRENCY_DAILY";
    let url = construct_url(function, symbol, interval);

    let response = reqwest::get(url).await?;

    match response.status() {
        reqwest::StatusCode::OK => convert_data(symbol, response).await,
        _ => Err("Request failed.".into()),
    }
}

async fn convert_data(symbol: &str, res: reqwest::Response) -> GenericResult<TimeSeries> {
    let mut alpha_vantage_data: AlphaVantageApiResponse = res.json().await?;

    let timeseries = alpha_vantage_data.to_timeseries(symbol, &Interval::Daily);

    timeseries.map(|ts| ts)
}

fn construct_url(function: &str, symbol: &str, _interval: &Interval) -> String {
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
