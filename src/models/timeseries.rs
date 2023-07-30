use super::{candle::Candle, interval::Interval};

#[derive(Debug, Clone)]
pub struct TimeSeries {
    pub ticker: String,
    pub interval: Interval,
    pub candles: Vec<Candle>,
}

impl TimeSeries {
    pub fn dummy() -> Self {
        TimeSeries {
            ticker: "DUMMY".to_string(),
            interval: Interval::Daily,
            candles: Vec::new(),
        }
    }
}
