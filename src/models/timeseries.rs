use super::{candle::Candle, interval::Interval};

#[derive(Debug, Clone)]
pub struct TimeSeries {
    pub ticker: String,
    pub interval: Interval,
    pub candles: Vec<Candle>,
}
