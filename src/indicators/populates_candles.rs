use crate::models::{timeseries::TimeSeries, generic_result::GenericResult};
use super::indicator_args::IndicatorArgs;

pub trait PopulatesCandles {
    fn populate_candles(ts: &mut TimeSeries, args: IndicatorArgs) -> GenericResult<()>;
    fn populate_candles_default(ts: &mut TimeSeries) -> GenericResult<()>;
}
