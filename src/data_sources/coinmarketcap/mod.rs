mod coinmarketcap_structs;
use std::env;

use anyhow::Result;
use reqwest::Client;

use crate::{
    data_sources::coinmarketcap::coinmarketcap_structs::CoinMarketCapApiResponse,
    models::{interval::Interval, timeseries::TimeSeries},
};

pub async fn get() -> Result<TimeSeries> {
    let symbol = "BTC";
    let interval = &Interval::Day1;

    let api_key = env::var("COINMARKETCAP_KEY")?;
    let url = construct_url(symbol, interval)?;

    let client = Client::new();
    let response = client
        .get(url)
        .header("X-CMC_PRO_API_KEY", api_key)
        .send()
        .await?;
    let _: CoinMarketCapApiResponse = response.json().await?;

    Ok(TimeSeries::dummy())
}

fn construct_url(_symbol: &str, _interval: &Interval) -> Result<String> {
    let market = "cryptocurrency";
    Ok(format!(
        "https://pro-api.coinmarketcap.com/v1/{}/listings/latest?start=1&limit=1&convert=USD",
        market
    ))
}
