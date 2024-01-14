use super::is_resolution_strategy::IsResolutionStrategy;
use crate::{
    indicators::indicator_type::IndicatorType,
    models::{candle::Candle, strategy_orientation::StrategyOrientation},
};
use anyhow::{anyhow, Result, Context};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicPivotResolution {
    pub len: usize,
}

impl IsResolutionStrategy for DynamicPivotResolution {
    fn n_candles_stop_loss(&self) -> usize {
        self.len + 1
    }

    fn n_candles_take_profit(&self) -> usize {
        self.len + 1
    }

    fn stop_loss_reached(&self, orientation: &StrategyOrientation, candles: &[Candle]) -> Result<bool> {
        let len = candles.len();

        if len < self.len + 1 { 
            let msg = "Not enough enough candles to determine if stop loss reached.";
            return Err(anyhow!(msg));
        }

        let ind_type = IndicatorType::DynamicPivot(self.len);
        let pivots = candles[len - (self.len + 1)]
            .indicators
            .get(&ind_type)
            .context(format!("Unable to find DynamicPivots indicator of length {}", self.len))?
            .as_dynamic_pivots()
            .context("Unable to convert to Indicator::DynamicPivots")?;
       
        match orientation {
            StrategyOrientation::Long => {
                let bound = pivots.low.context("No pivot provided for comparison")?;
                Ok(candles[len-1].close < bound)
            }
            StrategyOrientation::Short => {
                let bound = pivots.high.context("No pivot provided for comparison")?;
                Ok(candles[len-1].close > bound)
            }
        }
    }

    fn take_profit_reached(&self, orientation: &StrategyOrientation, candles: &[Candle]) -> Result<bool> {
        let len = candles.len();

        if len < self.len + 1 { 
            let msg = "Not enough enough candles to determine if take-profit reached.";
            return Err(anyhow!(msg));
        }

        let ind_type = IndicatorType::DynamicPivot(self.len);
        let pivots = candles[len - (self.len + 1)]
            .indicators
            .get(&ind_type)
            .context(format!("Unable to find DynamicPivots indicator of length {}", self.len))?
            .as_dynamic_pivots()
            .context("Unable to convert to Indicator::DynamicPivots")?;
       
        match orientation {
            StrategyOrientation::Long => {
                let bound = pivots.high.context("No pivot provided for comparison")?;
                Ok(candles[len-1].high > bound)
            }
            StrategyOrientation::Short => {
                let bound = pivots.low.context("No pivot provided for comparison")?;
                Ok(candles[len-1].low < bound)
            }
        }
    }
}

impl DynamicPivotResolution {
    pub fn new() -> Self {
        Self { len: 15 }
    }
}
