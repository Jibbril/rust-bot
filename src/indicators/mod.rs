pub mod indicator;
pub mod indicator_type;
pub mod atr;
pub mod bbw;
pub mod bbwp;
pub mod bollinger_bands;
pub mod dynamic_pivots;
pub mod rsi;
pub mod sma;

use crate::models::{generic_result::GenericResult, timeseries::TimeSeries};

pub trait PopulatesCandles {
    fn populate_candles(ts: &mut TimeSeries, length: usize) -> GenericResult<()>;
    fn populate_candles_default(ts: &mut TimeSeries) -> GenericResult<()>;
}


