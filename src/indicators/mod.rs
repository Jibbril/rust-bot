pub mod atr;
pub mod bbw;
pub mod bbwp;
pub mod bollinger_bands;
pub mod dynamic_pivots;
pub mod rsi;
pub mod sma;

use crate::models::{generic_result::GenericResult, timeseries::TimeSeries};
use atr::ATR;
use bbw::BBW;
use bbwp::BBWP;
use bollinger_bands::BollingerBands;
use dynamic_pivots::DynamicPivot;
use rsi::RSI;
use serde::Serialize;
use sma::SMA;

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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Indicator {
    SMA(Option<SMA>),
    RSI(Option<RSI>),
    ATR(Option<ATR>),
    BollingerBands(Option<BollingerBands>),
    BBW(Option<BBW>),
    BBWP(Option<BBWP>),
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

    #[allow(dead_code)] // TODO: Remove once used
    pub fn as_bbw(&self) -> Option<BBW> {
        if let Indicator::BBW(bbw) = self {
            bbw.clone()
        } else {
            None
        }
    }

    #[allow(dead_code)] // TODO: Remove once used
    pub fn as_bbwp(&self) -> Option<BBWP> {
        if let Indicator::BBWP(bbwp) = self {
            bbwp.clone()
        } else {
            None
        }
    }
}
