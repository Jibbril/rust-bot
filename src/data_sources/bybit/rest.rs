use std::vec;

use super::{bybit_structs::BybitApiResponse, util::interval_to_str};
use crate::{
    data_sources::api_response::ApiResponse,
    models::{
        candle::Candle, interval::Interval, net_version::NetVersion, timeseries::TimeSeries,
        timeseries_builder::TimeSeriesBuilder,
    },
};
use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use reqwest::Client;

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
    let url = generate_url(symbol, interval, 1000, net, to, Some(from))?;
    let response = client.get(url).send().await?;

    let candles = match response.status() {
        reqwest::StatusCode::OK => {
            let mut response: BybitApiResponse = response.json().await?;
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
    let mut end = Utc::now();
    let mut first_iter = true;

    while remaining > 1 {
        // Add one for final request due to issues with request ordering
        if remaining < GET_REQUEST_LIMIT {
            remaining += 1;
        }

        let url = generate_url(
            symbol,
            interval,
            remaining,
            net,
            end.timestamp_millis(),
            None,
        )?;
        let response = client.get(url).send().await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let mut response: BybitApiResponse = response.json().await?;
                let mut candles = response.to_candles(!first_iter)?;
                candles.reverse();
                acc.extend(candles.clone());

                end = candles
                    .last()
                    .context("Expected at least one candle.")?
                    .timestamp;

                remaining -= remaining.min(candles.len());

                Ok(())
            }
            _ => Err(anyhow!(BYBIT_ERROR)),
        }?;

        if first_iter {
            first_iter = false;
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
    end: i64,
    start: Option<i64>,
) -> Result<String> {
    let interval = interval_to_str(interval)?;
    let base = match net {
        NetVersion::Testnet => "https://api-testnet.bybit.com",
        NetVersion::Mainnet => "https://api.bybit.com",
    };

    let mut url = format!(
        "{}/v5/market/kline?category=spot&symbol={}&interval={}&limit={}&end={}",
        base,
        symbol,
        interval,
        len,
        end.to_string()
    );

    match start {
        Some(s) => url = format!("{}&start={}", url, s.to_string()),
        _ => {}
    }

    Ok(url)
}
