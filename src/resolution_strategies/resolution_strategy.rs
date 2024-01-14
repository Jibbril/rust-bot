use crate::models::{candle::Candle, strategy_orientation::StrategyOrientation};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use super::{dynamic_pivot::DynamicPivotResolution, is_resolution_strategy::IsResolutionStrategy, fixed_values::FixedValuesResolution};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolutionStrategy {
    DynamicPivot(DynamicPivotResolution),
    FixedValues(FixedValuesResolution),
}

impl IsResolutionStrategy for ResolutionStrategy {
    fn n_candles_stop_loss(&self) -> usize {
        match self {
            ResolutionStrategy::DynamicPivot(dp) => dp.n_candles_stop_loss(),
            ResolutionStrategy::FixedValues(fv) => fv.n_candles_stop_loss(),
        }       
    }

    fn n_candles_take_profit(&self) -> usize {
        match self {
            ResolutionStrategy::DynamicPivot(dp) => dp.n_candles_take_profit(),
            ResolutionStrategy::FixedValues(fv) => fv.n_candles_take_profit(),
        }       
    }

    fn stop_loss_reached(&self, orientation: &StrategyOrientation, candles: &[Candle]) -> Result<bool>{
        match self {
            ResolutionStrategy::DynamicPivot(dp) => dp.stop_loss_reached(orientation, candles),
            ResolutionStrategy::FixedValues(fv) => fv.stop_loss_reached(orientation, candles),
        }       
    }

    fn take_profit_reached(&self, orientation: &StrategyOrientation, candles: &[Candle]) -> Result<bool> {
        match self {
            ResolutionStrategy::DynamicPivot(dp) => dp.take_profit_reached(orientation, candles),
            ResolutionStrategy::FixedValues(fv) => fv.take_profit_reached(orientation, candles),
        }       
    }
}

impl Display for ResolutionStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DynamicPivot(dp) => write!(f, "DynamicPivot({})", dp.len),
            Self::FixedValues(fv) => write!(f, "FixedValues({},{})", fv.high, fv.low),
        }
    }
}
