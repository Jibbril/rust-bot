use crate::{
    data_sources::api_response::ApiResponse,
    models::{candle::Candle, interval::Interval, timeseries::TimeSeries},
    utils::millis_to_datetime,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BybitApiResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: ResultData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

impl ApiResponse for BybitApiResponse {
    fn to_timeseries(&mut self, symbol: &str, interval: &Interval) -> Result<TimeSeries> {
        let klines = match &self.result.list {
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

            // Bybit returns the current interval as well, we only want
            // historical data here.
            candles.pop();

            TimeSeries::new(symbol.to_string(), interval.clone(), candles)
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultData {
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
