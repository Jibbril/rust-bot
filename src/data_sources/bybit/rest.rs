use std::vec;

use super::{bybit_structs::BybitApiResponse, util::interval_to_str};
use crate::{
    data_sources::api_response::ApiResponse,
    models::{interval::Interval, net_version::NetVersion, timeseries::TimeSeries, candle::Candle},
};
use anyhow::{anyhow, Result, Context};
use chrono::{Utc, DateTime};
use reqwest::Client;

const GET_REQUEST_LIMIT: usize = 1000;

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

        let url = generate_url(symbol, interval, remaining, net, end)?;
        let response = client.get(url).send().await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let mut response: BybitApiResponse = response.json().await?;
                let mut candles = response.to_candles(!first_iter)?;
                candles.reverse();
                acc.extend(candles.clone());

                end = candles.last()
                    .context("Expected at least one candle.")?
                    .timestamp;

                remaining -= remaining.min(candles.len());

                Ok(())
            }
            _ => Err(anyhow!("Bybit request failed.")),
        }?;


        if first_iter {
            first_iter = false;
        }
    }

    acc.reverse();

    Ok(TimeSeries::new(symbol.to_string(), interval.clone(), acc))
}

fn generate_url(
    symbol: &str, 
    interval: &Interval, 
    len: usize, 
    net: &NetVersion,
    end: DateTime<Utc>
) -> Result<String> {
    let interval = interval_to_str(interval)?;
    let base = match net {
        NetVersion::Testnet => "https://api-testnet.bybit.com",
        NetVersion::Mainnet => "https://api.bybit.com",
    };
    let end = end.timestamp_millis();

    Ok(format!(
        "{}/v5/market/kline?category=spot&symbol={}&interval={}&limit={}&end={}",
        base, symbol, interval, len, end.to_string()
    ))
}
