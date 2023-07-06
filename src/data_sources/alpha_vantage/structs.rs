use serde::Deserialize;
use std::collections::HashMap;
use std::cmp::Reverse;
use crate::utils::{
    timeseries::{
        str_date_to_datetime, 
        Candle, 
        TimeSeries, 
        Interval
    }, 
    generic_result::GenericResult
};

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

#[derive(Debug, Deserialize)]
pub struct  AlphaVantageApiResponse {
    #[serde(rename = "Meta Data")]
    metadata: AlphaVantageMetaData,

    #[serde(rename = "Time Series (Digital Currency Daily)")]
    timeseries: HashMap<String, AlphaVantageTimeSeries>
}

impl AlphaVantageApiResponse {
    pub fn to_timeseries(&mut self, interval: Interval) -> GenericResult<TimeSeries> {
        let candles: GenericResult<Vec<Candle>> = self.timeseries.iter()
            .map(|(date,ts)| {
                let datetime = str_date_to_datetime(date)?;

                Ok(Candle {
                    timestamp: datetime,
                    open: ts.open.parse::<f32>()?,
                    close: ts.close.parse::<f32>()?,
                    high: ts.high.parse::<f32>()?,
                    low: ts.low.parse::<f32>()?,
                    volume: ts.volume.parse::<f32>()?,
                })
            })
            .collect();

        candles.map(|mut candles| {
            candles.sort_by_key(|candle| Reverse(candle.timestamp));

            TimeSeries {
                ticker: self.metadata.digital_currency_code.to_string(),
                interval,
                candles, 
            }
        })
    }
}
