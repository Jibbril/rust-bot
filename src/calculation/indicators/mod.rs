mod calculation_mode;
mod rsi;
mod sma;

use rsi::RSI;
use sma::SMA;

use crate::utils::{generic_result::GenericResult, timeseries::Candle};

pub trait PopulatesCandles {
    fn populate_candles(candles: &mut Vec<Candle>, length: usize) -> GenericResult<()>;
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum IndicatorType {
    SMA(usize),
    RSI(usize),
}

#[derive(Debug, Clone)]
pub enum Indicator {
    SMA(SMA),
    RSI(RSI),
}
