pub mod atr_resolution;
pub mod dynamic_pivot;

use self::dynamic_pivot::DynamicPivotResolution;
use crate::models::{candle::Candle, strategy_orientation::StrategyOrientation};
use anyhow::{anyhow, Result};
use atr_resolution::AtrResolution;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolutionStrategy {
    ATR(AtrResolution),
    DynamicPivot(DynamicPivotResolution),
}

impl CalculatesStopLosses for ResolutionStrategy {
    fn calculate_stop_loss(
        &self,
        candles: &Vec<Candle>,
        i: usize,
        orientation: &StrategyOrientation,
        len: usize,
    ) -> Result<f64> {
        match self {
            ResolutionStrategy::ATR(atr) => atr.calculate_stop_loss(candles, i, orientation, len),
            ResolutionStrategy::DynamicPivot(pivot) => {
                pivot.calculate_stop_loss(candles, i, orientation, len)
            }
        }
    }
}

impl CalculatesTakeProfits for ResolutionStrategy {
    fn calculate_take_profit(
        &self,
        candles: &Vec<Candle>,
        i: usize,
        orientation: &StrategyOrientation,
        len: usize,
    ) -> Result<f64> {
        match self {
            ResolutionStrategy::ATR(atr) => atr.calculate_take_profit(candles, i, orientation, len),
            ResolutionStrategy::DynamicPivot(_) => Err(anyhow!(
                "DynamicPivotResolution cannot be used to calculate take-profits."
            )),
        }
    }
}

impl Display for ResolutionStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ATR(atr) => write!(
                f,
                "ATR resolution({},{},{})",
                atr.len, atr.take_profit_multiple, atr.stop_loss_multiple
            ),
            Self::DynamicPivot(pivot) => write!(f, "DynamicPivot({})", pivot.len),
        }
    }
}

pub trait CalculatesStopLosses {
    fn calculate_stop_loss(
        &self,
        candles: &Vec<Candle>,
        i: usize,
        orientation: &StrategyOrientation,
        len: usize,
    ) -> Result<f64>;
}

pub trait CalculatesTakeProfits {
    fn calculate_take_profit(
        &self,
        candles: &Vec<Candle>,
        i: usize,
        orientation: &StrategyOrientation,
        len: usize,
    ) -> Result<f64>;
}
