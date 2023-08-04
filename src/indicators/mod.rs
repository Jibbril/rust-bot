pub mod atr;
pub mod rsi;
pub mod sma;
pub mod dynamic_pivots;

use atr::ATR;
use rsi::RSI;
use serde::Serialize;
use sma::SMA;
use dynamic_pivots::DynamicPivot;

use crate::models::{candle::Candle, generic_result::GenericResult};

pub trait PopulatesCandles {
    fn populate_candles(candles: &mut Vec<Candle>, length: usize) -> GenericResult<()>;
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize)]
pub enum IndicatorType {
    SMA(usize),
    RSI(usize),
    ATR(usize),
    DynamicPivot(usize),
}

#[derive(Debug, Clone)]
pub enum Indicator {
    SMA(Option<SMA>),
    RSI(Option<RSI>),
    ATR(Option<ATR>),
    DynamicPivot(Option<DynamicPivot>),
}

impl Indicator {
    pub fn get_scalar_value(&self) -> Option<f64> {
        match self {
            Indicator::SMA(opt) => opt.map(|sma| sma.value),
            Indicator::RSI(opt) => opt.map(|rsi| rsi.value),
            Indicator::ATR(opt) => opt.map(|atr| atr.value),
            _ => None
        }
    }

    pub fn as_sma(&self) -> Option<SMA> {
        if let Indicator::SMA(sma) = self {
            sma.clone()
        } else {
            None
        }
    }

    pub fn as_rsi(&self) -> Option<RSI> {
        if let Indicator::RSI(rsi) = self {
            rsi.clone()
        } else {
            None
        }
    }

    #[allow(dead_code)] // TODO: Remove once used
    pub fn as_atr(&self) -> Option<ATR> {
        if let Indicator::ATR(atr) = self {
            atr.clone()
        } else {
            None
        }
    }

    #[allow(dead_code)] // TODO: Remove once used
    pub fn as_dynamic_pivots(&self) -> Option<DynamicPivot> {
        if let Indicator::DynamicPivot(pivots) = self {
            pivots.clone()
        } else {
            None
        }
    }
}
