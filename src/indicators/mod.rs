pub mod atr;
pub mod dynamic_pivots;
pub mod rsi;
pub mod sma;
pub mod bbwp;
pub mod bollinger_bands;

use crate::models::{candle::Candle, generic_result::GenericResult};
use atr::ATR;
use dynamic_pivots::DynamicPivot;
use rsi::RSI;
use serde::Serialize;
use sma::SMA;
use bollinger_bands::BollingerBands;


pub trait PopulatesCandles {
    fn populate_candles(candles: &mut Vec<Candle>, length: usize) -> GenericResult<()>;
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize)]
pub enum IndicatorType {
    SMA(usize),
    RSI(usize),
    ATR(usize),
    BollingerBands(usize),
    DynamicPivot(usize),
}

#[derive(Debug, Clone)]
pub enum Indicator {
    SMA(Option<SMA>),
    RSI(Option<RSI>),
    ATR(Option<ATR>),
    BollingerBands(Option<BollingerBands>),
    DynamicPivot(Option<DynamicPivot>),
}

impl Indicator {
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

    pub fn as_atr(&self) -> Option<ATR> {
        if let Indicator::ATR(atr) = self {
            atr.clone()
        } else {
            None
        }
    }

    pub fn as_dynamic_pivots(&self) -> Option<DynamicPivot> {
        if let Indicator::DynamicPivot(pivots) = self {
            pivots.clone()
        } else {
            None
        }
    }

    #[allow(dead_code)] // TODO: Remove once used
    pub fn as_bollinger_bands(&self) -> Option<BollingerBands> {
        if let Indicator::BollingerBands(bb) = self {
            bb.clone()
        } else {
            None
        }
    }
}
