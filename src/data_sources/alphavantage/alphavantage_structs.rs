use anyhow::Result;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

use crate::{
    data_sources::ApiResponse,
    models::{candle::Candle, interval::Interval, timeseries::TimeSeries},
    utils::str_date_to_datetime,
};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct AlphaVantageMetaData {
    #[serde(rename = "1. Information")]
    _information: String,

    #[serde(rename = "2. Digital Currency Code")]
    digital_currency_code: String,

    #[serde(rename = "3. Digital Currency Name")]
    _digital_currency_name: String,

    #[serde(rename = "4. Market Code")]
    _market_code: String,

    #[serde(rename = "5. Market Name")]
    _market_name: String,

    #[serde(rename = "6. Last Refreshed")]
    _last_refreshed: String,

    #[serde(rename = "7. Time Zone")]
    _time_zone: String,
}

#[derive(Debug, Deserialize)]
struct AlphaVantageTimeSeries {
    #[serde(rename = "1a. open (USD)")]
    open: String,

    #[serde(rename = "1b. open (USD)")]
    _open_duplicate: String,

    #[serde(rename = "2a. high (USD)")]
    high: String,

    #[serde(rename = "2b. high (USD)")]
    _high_duplicate: String,

    #[serde(rename = "3a. low (USD)")]
    low: String,

    #[serde(rename = "3b. low (USD)")]
    _low_duplicate: String,

    #[serde(rename = "4a. close (USD)")]
    close: String,

    #[serde(rename = "4b. close (USD)")]
    _close_duplicate: String,

    #[serde(rename = "5. volume")]
    volume: String,

    #[serde(rename = "6. market cap (USD)")]
    _market_cap: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct AlphaVantageApiResponse {
    #[serde(rename = "Meta Data")]
    metadata: AlphaVantageMetaData,

    #[serde(rename = "Time Series (Digital Currency Daily)")]
    timeseries: HashMap<String, AlphaVantageTimeSeries>,
}

impl ApiResponse for AlphaVantageApiResponse {
    fn to_timeseries(&mut self, symbol: &str, interval: &Interval) -> Result<TimeSeries> {
        let candles: Result<Vec<Candle>> = self
            .timeseries
            .iter()
            .map(|(date, ts)| {
                let datetime = str_date_to_datetime(date)?;

                Ok(Candle {
                    timestamp: datetime,
                    open: ts.open.parse::<f64>()?,
                    close: ts.close.parse::<f64>()?,
                    high: ts.high.parse::<f64>()?,
                    low: ts.low.parse::<f64>()?,
                    volume: ts.volume.parse::<f64>()?,
                    indicators: HashMap::new(),
                })
            })
            .collect();

        candles.map(|mut candles| {
            candles.sort_by_key(|candle| candle.timestamp);

            TimeSeries {
                ticker: symbol.to_string(),
                interval: interval.clone(),
                candles,
                indicators: HashSet::new(),
            }
        })
    }
}
