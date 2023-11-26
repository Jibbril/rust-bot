use crate::models::{interval::Interval, timeseries::TimeSeries};
use anyhow::Result;

pub trait ApiResponse {
    fn to_timeseries(&mut self, symbol: &str, interval: &Interval) -> Result<TimeSeries>;
}
