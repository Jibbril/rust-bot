use super::{candle::Candle, interval::Interval, websockets::{observer::Observer, websocketpayload::WebsocketPayload}};
use crate::indicators::indicator_type::IndicatorType;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct TimeSeries {
    pub ticker: String,
    pub interval: Interval,
    pub candles: Vec<Candle>,
    pub indicators: HashSet<IndicatorType>,
}

impl Observer<WebsocketPayload> for TimeSeries {
    fn update(&mut self, payload: WebsocketPayload) {
        println!("Payload: {:#?}",payload);
        
        if let Some(candle) = payload.candle {
            self.add_candle(candle);
        } else if let Some(error) = payload.error {
            println!("Error: {}", error);
        }
    }
}

impl TimeSeries {
    pub fn new(ticker: String, interval: Interval, candles: Vec<Candle>) -> Self {
        TimeSeries {
            ticker,
            interval,
            candles,
            indicators: HashSet::new(),
        }
    }

    fn add_candle(&mut self, candle: Candle) {
        // TODO: Perform checks of timestamp to ensure that no
        // duplicates are added, or that there has not been any
        // missed candles in between.
        self.candles.push(candle);
    }

    pub fn dummy() -> Self {
        TimeSeries {
            ticker: "DUMMY".to_string(),
            interval: Interval::Day1,
            candles: Vec::new(),
            indicators: HashSet::new(),
        }
    }
}
