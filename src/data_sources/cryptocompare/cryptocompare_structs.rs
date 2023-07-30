use crate::{
    data_sources::ApiResponse,
    models::{
        candle::Candle, generic_result::GenericResult, interval::Interval, timeseries::TimeSeries,
    },
    utils::secs_to_datetime,
};
use serde::Deserialize;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct CryptoCompareApiResponse {
    #[serde(rename = "Response")]
    response: String,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "HasWarning")]
    has_warning: bool,
    #[serde(rename = "Type")]
    type_: i32,
    #[serde(rename = "RateLimit")]
    rate_limit: serde_json::Value,
    #[serde(rename = "Data")]
    data: Data,
}

impl ApiResponse for CryptoCompareApiResponse {
    fn to_timeseries(&mut self, symbol: &str, interval: &Interval) -> GenericResult<TimeSeries> {
        let candles: GenericResult<Vec<Candle>> = self
            .data
            .data
            .iter()
            .map(|entry| {
                let timestamp = secs_to_datetime(entry.time)?;

                Ok(Candle {
                    timestamp,
                    open: entry.open,
                    close: entry.close,
                    high: entry.high,
                    low: entry.low,
                    volume: entry.volume_from,
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
            }
        })
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "Aggregated")]
    aggregated: bool,
    #[serde(rename = "TimeFrom")]
    time_from: u64,
    #[serde(rename = "TimeTo")]
    time_to: u64,
    #[serde(rename = "Data")]
    data: Vec<DataEntry>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct DataEntry {
    #[serde(rename = "time")]
    time: u64,
    #[serde(rename = "high")]
    high: f64,
    #[serde(rename = "low")]
    low: f64,
    #[serde(rename = "open")]
    open: f64,
    #[serde(rename = "volumefrom")]
    volume_from: f64,
    #[serde(rename = "volumeto")]
    volume_to: f64,
    #[serde(rename = "close")]
    close: f64,
    #[serde(rename = "conversionType")]
    conversion_type: String,
    #[serde(rename = "conversionSymbol")]
    conversion_symbol: String,
}
