pub mod atr_resolution;

use std::fmt::{Formatter, Display, Result};
use crate::models::{
    candle::Candle, generic_result::GenericResult, strategy_orientation::StrategyOrientation,
};
use atr_resolution::AtrResolution;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolutionStrategy {
    ATR(AtrResolution),
}

impl CalculatesTradeBounds for ResolutionStrategy {
    fn get_trade_bounds(
        &self,
        candles: &Vec<Candle>,
        i: usize,
        orientation: &StrategyOrientation,
    ) -> GenericResult<(f64, f64)> {
        match self {
            ResolutionStrategy::ATR(atr) => atr.get_trade_bounds(candles, i, orientation),
        }
    }
}

impl Display for ResolutionStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::ATR(atr) => write!(f, "ATR resolution({},{},{})",atr.length, atr.take_profit_multiple, atr.stop_loss_multiple)
        }
    }
}

pub trait CalculatesTradeBounds {
    fn get_trade_bounds(
        &self,
        candles: &Vec<Candle>,
        i: usize,
        orientation: &StrategyOrientation,
    ) -> GenericResult<(f64, f64)>;
}
