mod coinmarketcap_structs;
use std::env;

use reqwest::Client;

use crate::{
    data_sources::coinmarketcap::coinmarketcap_structs::CoinMarketCapApiResponse,
    models::{generic_result::GenericResult, interval::Interval, timeseries::TimeSeries},
};

pub async fn get() -> GenericResult<TimeSeries> {
    let symbol = "BTC";
    let interval = &Interval::Daily;

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

fn construct_url(_symbol: &str, _interval: &Interval) -> GenericResult<String> {
    let market = "cryptocurrency";
    Ok(format!(
        "https://pro-api.coinmarketcap.com/v1/{}/listings/latest?start=1&limit=1&convert=USD",
        market
    ))
}
