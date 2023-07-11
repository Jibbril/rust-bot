pub mod rsi;
pub mod sma;

use rsi::RSI;
use serde::Serialize;
use sma::SMA;

use crate::utils::{generic_result::GenericResult, timeseries::Candle};

pub trait PopulatesCandles {
    fn populate_candles(candles: &mut Vec<Candle>, length: usize) -> GenericResult<()>;
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize)]
pub enum IndicatorType {
    SMA(usize),
    RSI(usize),
}

#[derive(Debug, Clone, Serialize)]
pub enum Indicator {
    SMA(Option<SMA>),
    RSI(Option<RSI>),
}
