pub mod atr_resolution;
pub mod dynamic_pivot;

use crate::models::{
    candle::Candle, generic_result::GenericResult, strategy_orientation::StrategyOrientation,
};
use atr_resolution::AtrResolution;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};
use self::dynamic_pivot::DynamicPivotResolution;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolutionStrategy {
    ATR(AtrResolution),
    DynamicPivot(DynamicPivotResolution)
}

impl CalculatesStopLosses for ResolutionStrategy {
    fn calculate_stop_loss(
        &self,
        candles: &Vec<Candle>,
        i: usize,
        orientation: &StrategyOrientation,
        length: usize
    ) -> GenericResult<f64> {
        match self {
            ResolutionStrategy::ATR(atr) => atr.calculate_stop_loss(candles, i, orientation, length),
            ResolutionStrategy::DynamicPivot(pivot) => pivot.calculate_stop_loss(candles,i,orientation,length),
        }
    }
}

impl CalculatesTakeProfits for ResolutionStrategy {
    fn calculate_take_profit(
        &self,
        candles: &Vec<Candle>,
        i: usize,
        orientation: &StrategyOrientation,
        length: usize
    ) -> GenericResult<f64> {
        match self {
            ResolutionStrategy::ATR(atr) => atr.calculate_take_profit(candles, i, orientation, length),
            ResolutionStrategy::DynamicPivot(_) => Err("DynamicPivotResolution cannot be used to calculate take-profits.".into()),
        }
    }
}

impl Display for ResolutionStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::ATR(atr) => write!(
                f,
                "ATR resolution({},{},{})",
                atr.length, atr.take_profit_multiple, atr.stop_loss_multiple
            ),
            Self::DynamicPivot(pivot) => write!(f,"DynamicPivot({})", pivot.length),
        }
    }
}

pub trait CalculatesStopLosses {
    fn calculate_stop_loss(
        &self,
        candles: &Vec<Candle>,
        i: usize,
        orientation: &StrategyOrientation,
        length: usize,
    ) -> GenericResult<f64>;
}

pub trait CalculatesTakeProfits {
    fn calculate_take_profit(
        &self,
        candles: &Vec<Candle>,
        i: usize,
        orientation: &StrategyOrientation,
        length: usize,
    ) -> GenericResult<f64>;
}