use super::{bbw::BBW, PopulatesCandles};
use crate::models::{generic_result::GenericResult, timeseries::TimeSeries};

/// Bollinger Band Width Percentile
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct BBWP {
    #[allow(dead_code)] // TODO: Remove once used
    pub length: usize,
    pub lookback: usize,
    bbw: BBW,
}

impl PopulatesCandles for BBWP {
    fn populate_candles(_ts: &mut TimeSeries, _length: usize) -> GenericResult<()> {
        // TODO: Add check for the BBW indicator needed.

        todo!()
    }

    fn populate_candles_default(ts: &mut TimeSeries) -> GenericResult<()> {
        Self::populate_candles(ts, 20)
    }
}

impl BBWP {}
