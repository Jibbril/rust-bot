use super::{candle::Candle, interval::Interval};
use crate::indicators::indicator_type::IndicatorType;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct TimeSeries {
    pub ticker: String,
    pub interval: Interval,
    pub candles: Vec<Candle>,
    pub indicators: HashSet<IndicatorType>,
}

impl TimeSeries {
    pub fn dummy() -> Self {
        TimeSeries {
            ticker: "DUMMY".to_string(),
            interval: Interval::Day1,
            candles: Vec::new(),
            indicators: HashSet::new(),
        }
    }
}
