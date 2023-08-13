use crate::models::{generic_result::GenericResult, candle::Candle};
use super::{bbw::BBW, PopulatesCandles};

/// Bollinger Band Width Percentile
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct BBWP {
    #[allow(dead_code)] // TODO: Remove once used
    pub length: usize,
    pub lookback: usize,
    bbw: BBW,
}

impl PopulatesCandles for BBWP {
    fn populate_candles(candles: &mut Vec<Candle>, length: usize) -> GenericResult<()> {
        todo!()
    }

    fn populate_candles_default(candles: &mut Vec<Candle>) -> GenericResult<()> {
        Self::populate_candles(candles, 20)
    }
}

impl BBWP {
    pub fn calculate(length: usize, i: usize, candles: &[Candle]) -> Option<BBWP> {
        todo!()
    } 
}