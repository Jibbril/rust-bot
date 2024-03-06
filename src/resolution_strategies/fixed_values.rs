use super::is_resolution_strategy::IsResolutionStrategy;
use crate::{
    indicators::indicator_type::IndicatorType,
    models::{
        candle::Candle, setups::setup::Setup, strategy_orientation::StrategyOrientation,
        traits::requires_indicators::RequiresIndicators,
    },
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedValuesResolution {
    pub high: f64,
    pub low: f64,
}

impl IsResolutionStrategy for FixedValuesResolution {
    fn n_candles_stop_loss(&self) -> usize {
        1
    }

    fn n_candles_take_profit(&self) -> usize {
        1
    }

    fn stop_loss_reached(
        &self,
        orientation: &StrategyOrientation,
        candles: &[Candle],
    ) -> Result<bool> {
        if candles.len() < 1 {
            return Err(anyhow!("No candle provided for fixed value resolution."));
        }

        let candle = &candles[0];

        Ok(match orientation {
            StrategyOrientation::Long => candle.close < self.low,
            StrategyOrientation::Short => candle.close > self.high,
        })
    }

    fn take_profit_reached(
        &self,
        orientation: &StrategyOrientation,
        candles: &[Candle],
    ) -> Result<bool> {
        if candles.len() < 1 {
            return Err(anyhow!("No candle provided for fixed value resolution."));
        }

        let candle = &candles[0];

        Ok(match orientation {
            StrategyOrientation::Long => candle.high > self.high,
            StrategyOrientation::Short => candle.low < self.low,
        })
    }

    fn set_initial_values(&mut self, _setup: &Setup) -> Result<()> {
        Err(anyhow!(
            "Fixed resolution does not support setting initial values from setup."
        ))
    }
}

impl RequiresIndicators for FixedValuesResolution {
    fn required_indicators(&self) -> Vec<IndicatorType> {
        vec![]
    }
}

impl FixedValuesResolution {
    pub fn new(high: f64, low: f64) -> Self {
        Self { high, low }
    }
}
