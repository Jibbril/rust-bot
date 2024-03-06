use crate::models::{candle::Candle, interval::Interval, timeseries::TimeSeries};
use anyhow::Result;

pub trait ApiResponse {
    fn to_timeseries(&mut self, symbol: &str, interval: &Interval) -> Result<TimeSeries>;
    fn to_candles(&mut self, pop_last: bool) -> Result<Vec<Candle>>;
}
