use crate::{
    models::{candle::Candle, interval::Interval, timeseries::TimeSeries},
    utils::millis_to_datetime,
};
use anyhow::{anyhow, Result};
use reqwest::Client;
use std::collections::HashMap;

pub async fn get(symbol: &str, interval: &Interval) -> Result<TimeSeries> {
    let url = generate_url(symbol, interval)?;

    let client = Client::new();
    let response = client.get(url).send().await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let mut response: Vec<Vec<f64>> = response.json().await?;
            response.reverse();

            generate_timeseries(symbol, interval, response)
        }
        _ => Err(anyhow!("Bitfinex request failed.")),
    }
}

fn generate_url(symbol: &str, interval: &Interval) -> Result<String> {
    let interval = match interval {
        // Interval::Minute1 => "1m",
        Interval::Minute5 => "5m",
        Interval::Minute15 => "15m",
        Interval::Minute30 => "30m",
        Interval::Hour1 => "1h",
        Interval::Hour4 => "4h",
        Interval::Day1 => "1D",
        Interval::Week1 => "7D",
        _ => return Err(anyhow!("Bitfinex does not support this interval.")),
        // Interval::Month1 => "1M",
    };

    Ok(format!(
        "https://api-pub.bitfinex.com/v2/candles/trade:{}:t{}/hist?limit=2000",
        interval, symbol
    ))
}

fn generate_timeseries(
    symbol: &str,
    interval: &Interval,
    response: Vec<Vec<f64>>,
) -> Result<TimeSeries> {
    let candles = response
        .iter()
        .map(|entry| {
            let timestamp = millis_to_datetime(entry[0] as u64)?;

            Ok(Candle {
                timestamp,
                open: entry[1],
                close: entry[2],
                high: entry[3],
                low: entry[4],
                volume: entry[5],
                indicators: HashMap::new(),
            })
        })
        .collect::<Result<Vec<Candle>>>()?;

    if candles.is_empty() {
        return Err(anyhow!("Bitfinex request failed."));
    }

    Ok(TimeSeries::new(
        symbol.to_string(),
        interval.clone(),
        candles,
    ))
}
