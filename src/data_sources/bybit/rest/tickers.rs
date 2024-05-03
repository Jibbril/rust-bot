use crate::{
    data_sources::bybit::rest::{api_responses::tickers::TickersApiResponse, utils::bybit_url},
    models::net_version::NetVersion,
};
use anyhow::{anyhow, Context, Result};
use reqwest::get;

pub async fn get_symbol_price(symbol: &str) -> Result<f64> {
    let url = bybit_url("/v5/market/tickers", &NetVersion::Mainnet);
    let url = format!("{}?category=spot&symbol={}", url, symbol);

    let res = get(url).await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let resdata: TickersApiResponse = res.json().await?;
            let res = resdata.result.context("Unable to parse TickersResult")?;

            if res.list.len() > 0 {
                Ok(res.list[0].last_price.parse()?)
            } else {
                Err(anyhow!("Unable to find ticker data."))
            }
        }
        _ => panic!("Unable to fetch server time."),
    }
}
