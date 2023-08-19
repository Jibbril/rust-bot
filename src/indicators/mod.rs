pub mod indicator;
pub mod atr;
pub mod bbw;
pub mod bbwp;
pub mod bollinger_bands;
pub mod dynamic_pivots;
pub mod rsi;
pub mod sma;

use crate::models::{generic_result::GenericResult, timeseries::TimeSeries};
use serde::Serialize;

pub trait PopulatesCandles {
    fn populate_candles(ts: &mut TimeSeries, length: usize) -> GenericResult<()>;
    fn populate_candles_default(ts: &mut TimeSeries) -> GenericResult<()>;
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize)]
pub enum IndicatorType {
    SMA(usize),
    RSI(usize),
    ATR(usize),
    BollingerBands(usize),
    BBW(usize),
    BBWP(usize),
    DynamicPivot(usize),
}

