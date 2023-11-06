use anyhow::Result;

use super::indicator_args::IndicatorArgs;
use crate::models::timeseries::TimeSeries;

pub trait PopulatesCandles {
    fn populate_candles_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()>;
    fn populate_candles(ts: &mut TimeSeries) -> Result<()>;
}

pub trait PopulatesCandlesWithSelf {
    fn populate_candles(&self, ts: &mut TimeSeries) -> Result<()>;
}
