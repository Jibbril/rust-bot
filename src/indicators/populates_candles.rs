use anyhow::Result;

use super::indicator_args::IndicatorArgs;
use crate::models::timeseries::TimeSeries;

pub trait PopulatesCandles {
    fn populate_candles(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()>;
    fn populate_candles_default(ts: &mut TimeSeries) -> Result<()>;
}
