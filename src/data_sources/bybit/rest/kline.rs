use crate::{
    data_sources::{
        api_response::ApiResponse,
        bybit::{rest::api_responses::kline::KlineResponse, util::interval_to_str},
    },
    models::{
        candle::Candle, interval::Interval, net_version::NetVersion, timeseries::TimeSeries,
        timeseries_builder::TimeSeriesBuilder,
    },
};
use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use std::vec;

const GET_REQUEST_LIMIT: usize = 1000;
const BYBIT_ERROR: &str = "Bybit request failed.";

#[allow(dead_code)]
pub async fn get_candles_between(
    symbol: &str,
    interval: &Interval,
    net: &NetVersion,
    from: i64,
    to: i64,
) -> Result<Vec<Candle>> {
    let client = Client::new();
    let url = generate_url(symbol, interval, 1000, net, Some(to), Some(from))?;
    let response = client.get(url).send().await?;

    let candles = match response.status() {
        reqwest::StatusCode::OK => {
            let mut response: KlineResponse = response.json().await?;
            response.to_candles(true)
        }
        _ => Err(anyhow!(BYBIT_ERROR)),
    }?;

    Ok(candles)
}

pub async fn get(
    symbol: &str,
    interval: &Interval,
    len: usize,
    net: &NetVersion,
) -> Result<TimeSeries> {
    let client = Client::new();

    let mut acc: Vec<Candle> = vec![];
    let mut remaining = len;
    let mut pop_last = len > GET_REQUEST_LIMIT;
    let mut end = None;

    while remaining > 1 {
        // Add one for final request due to issues with request ordering
        if remaining < GET_REQUEST_LIMIT {
            remaining += 1;
        }

        let url = generate_url(symbol, interval, remaining, net, end, None)?;
        let response = client.get(url).send().await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let mut response: KlineResponse = response.json().await?;
                let mut candles = response.to_candles(!pop_last)?;
                candles.reverse();
                acc.extend(candles.clone());

                let new_end = candles
                    .last()
                    .context("Expected at least one candle.")?
                    .timestamp;
                end = Some(new_end.timestamp_millis());

                remaining -= remaining.min(candles.len());

                Ok(())
            }
            _ => Err(anyhow!(BYBIT_ERROR)),
        }?;

        if pop_last {
            pop_last = false;
        }
    }

    acc.reverse();

    let ts = TimeSeriesBuilder::new()
        .symbol(symbol.to_string())
        .interval(interval.clone())
        .candles(acc)
        .build();

    Ok(ts)
}

fn generate_url(
    symbol: &str,
    interval: &Interval,
    len: usize,
    net: &NetVersion,
    end: Option<i64>,
    start: Option<i64>,
) -> Result<String> {
    let interval = interval_to_str(interval)?;
    let base = match net {
        NetVersion::Testnet => "https://api-testnet.bybit.com",
        NetVersion::Mainnet => "https://api.bybit.com",
    };

    let mut url = format!(
        "{}/v5/market/kline?category=spot&symbol={}&interval={}&limit={}",
        base, symbol, interval, len,
    );

    match start {
        Some(s) => url = format!("{}&start={}", url, s.to_string()),
        _ => {}
    }

    match end {
        Some(e) => url = format!("{}&end={}", url, e.to_string()),
        _ => {}
    }

    Ok(url)
}
