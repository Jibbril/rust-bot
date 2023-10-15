use anyhow::Result;
use crate::models::{interval::Interval, timeseries::TimeSeries};

pub trait ApiResponse {
    fn to_timeseries(&mut self, symbol: &str, interval: &Interval) -> Result<TimeSeries>;
}
