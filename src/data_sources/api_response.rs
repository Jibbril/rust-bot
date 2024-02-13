use crate::models::{interval::Interval, timeseries::TimeSeries, candle::Candle};
use anyhow::Result;

pub trait ApiResponse {
    fn to_timeseries(&mut self, symbol: &str, interval: &Interval) -> Result<TimeSeries>;
    fn to_candles(&mut self, pop_last: bool) -> Result<Vec<Candle>>;
}
