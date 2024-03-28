use crate::{
    data_sources::api_response::ApiResponse,
    models::{
        candle::Candle, interval::Interval, timeseries::TimeSeries,
        timeseries_builder::TimeSeriesBuilder,
    },
    utils::millis_to_datetime,
};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KlineResponse {
    #[serde(rename = "retCode")]
    pub ret_code: u32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: Option<KlineResult>,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl ApiResponse for KlineResponse {
    fn to_timeseries(&mut self, symbol: &str, interval: &Interval) -> Result<TimeSeries> {
        let candles = Self::to_candles(self, true)?;

        let ts = TimeSeriesBuilder::new()
            .symbol(symbol.to_string())
            .interval(interval.clone())
            .candles(candles)
            .build();

        Ok(ts)
    }

    fn to_candles(&mut self, pop_last: bool) -> Result<Vec<Candle>> {
        let result = self
            .result
            .as_ref()
            .context("Unable to parse KlineResult.")?;
        let klines = match &result.list {
            Some(result) => result,
            None => return Err(anyhow!(self.ret_msg.clone())),
        };

        // .ok_or_else(|| self.ret_msg.clone())?;

        let candles: Result<Vec<Candle>> = klines
            .iter()
            .map(|entry| {
                let timestamp = millis_to_datetime(entry.timestamp.parse::<u64>()?)?;
                Ok(Candle::new(
                    timestamp,
                    entry.open.parse::<f64>()?,
                    entry.close.parse::<f64>()?,
                    entry.high.parse::<f64>()?,
                    entry.low.parse::<f64>()?,
                    entry.volume.parse::<f64>()?,
                ))
            })
            .collect();

        candles.map(|mut candles| {
            candles.sort_by_key(|candle| candle.timestamp);

            if pop_last && candles.len() > 1 {
                // Bybit returns incomplete last candle, remove it.
                candles.pop();
            }

            candles
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KlineResult {
    // pub category: String,
    pub symbol: Option<String>,
    pub list: Option<Vec<BybitKline>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BybitKline {
    #[serde(rename = "0")]
    pub timestamp: String,
    #[serde(rename = "1")]
    pub open: String,
    #[serde(rename = "2")]
    pub high: String,
    #[serde(rename = "3")]
    pub low: String,
    #[serde(rename = "4")]
    pub close: String,
    #[serde(rename = "5")]
    pub volume: String,
    #[serde(rename = "6")]
    pub turnover: String,
}
