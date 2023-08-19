use crate::models::{timeseries::TimeSeries, generic_result::GenericResult};

pub trait PopulatesCandles {
    fn populate_candles(ts: &mut TimeSeries, length: usize) -> GenericResult<()>;
    fn populate_candles_default(ts: &mut TimeSeries) -> GenericResult<()>;
}