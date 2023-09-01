use super::indicator_args::IndicatorArgs;
use crate::models::{generic_result::GenericResult, timeseries::TimeSeries};

pub trait PopulatesCandles {
    fn populate_candles(ts: &mut TimeSeries, args: IndicatorArgs) -> GenericResult<()>;
    fn populate_candles_default(ts: &mut TimeSeries) -> GenericResult<()>;
}