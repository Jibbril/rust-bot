mod cryptocompare_structs;
use std::env;

use reqwest::Client;

use crate::models::{generic_result::GenericResult, timeseries::TimeSeries, interval::Interval};

use self::cryptocompare_structs::CryptoCompareApiResponse;


pub async fn get() -> GenericResult<TimeSeries> {
    let symbol = "BTC";
    let interval = &Interval::Daily;
    let api_key = env::var("CRYPTOCOMPARE_KEY")?;
    let url = construct_url(symbol,interval);

    let client = Client::new();
    let  response= client
        .get(url)
        .header("Authorization", format!("Apikey {}", api_key))
        .send()
        .await?;

    let response: CryptoCompareApiResponse = response.json().await?;

    println!("Response:{:#?}", response);

    Ok(TimeSeries::dummy())
}

fn construct_url(symbol: &str, interval: &Interval) -> String {
    let market = "USD";
    let interval = match interval {
        Interval::Daily => "histoday"
    };

    // TODO: Enable multiples using the aggregate parameter in the api

    format!(
        "https://min-api.cryptocompare.com/data/v2/{}?fsym={}&tsym={}&?limit=10",
        interval,
        symbol,
        market
    )
}