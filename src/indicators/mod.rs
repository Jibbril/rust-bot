pub mod atr;
pub mod rsi;
pub mod sma;

use atr::ATR;
use rsi::RSI;
use serde::Serialize;
use sma::SMA;

use crate::models::{candle::Candle, generic_result::GenericResult};

pub trait PopulatesCandles {
    fn populate_candles(candles: &mut Vec<Candle>, length: usize) -> GenericResult<()>;
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize)]
pub enum IndicatorType {
    SMA(usize),
    RSI(usize),
    ATR(usize),
}

#[derive(Debug, Clone)]
pub enum Indicator {
    SMA(Option<SMA>),
    RSI(Option<RSI>),
    ATR(Option<ATR>),
}

impl Indicator {
    pub fn get_scalar_value(&self) -> Option<f64> {
        match self {
            Indicator::SMA(opt) => opt.map(|sma| sma.value),
            Indicator::RSI(opt) => opt.map(|rsi| rsi.value),
            Indicator::ATR(opt) => opt.map(|atr| atr.value),
        }
    }
}
