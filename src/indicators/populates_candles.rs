use crate::{indicators::indicator_args::IndicatorArgs, models::timeseries::TimeSeries};
use anyhow::Result;

pub trait PopulatesCandles {
    fn populate_candles(ts: &mut TimeSeries) -> Result<()>;
    fn populate_candles_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()>;
    fn populate_last_candle(ts: &mut TimeSeries) -> Result<()>;
    fn populate_last_candle_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()>;
}

pub trait PopulatesCandlesWithSelf {
    fn populate_candles(&self, ts: &mut TimeSeries) -> Result<()>;
    fn populate_last_candle(&self, ts: &mut TimeSeries) -> Result<()>;
}
