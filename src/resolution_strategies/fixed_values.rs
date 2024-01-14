use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use crate::{models::{strategy_orientation::StrategyOrientation, candle::Candle}, indicators::indicator_type::IndicatorType};
use super::is_resolution_strategy::IsResolutionStrategy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedValuesResolution {
    pub high: f64,
    pub low: f64
}

impl IsResolutionStrategy for FixedValuesResolution {
    fn n_candles_stop_loss(&self) -> usize {
        1
    }

    fn n_candles_take_profit(&self) -> usize {
        1
    }

    fn stop_loss_reached(&self, orientation: &StrategyOrientation, candles: &[Candle]) -> Result<bool> {
        if candles.len() < 1 {
            return Err(anyhow!("No candle provided for fixed value resolution."))
        }

        let candle = &candles[0];

        Ok(match orientation {
            StrategyOrientation::Long => candle.close < self.low,
            StrategyOrientation::Short => candle.close > self.high,
        })
    }

    fn take_profit_reached(&self, orientation: &StrategyOrientation, candles: &[Candle]) -> Result<bool> {
        if candles.len() < 1 {
            return Err(anyhow!("No candle provided for fixed value resolution."))
        }

        let candle = &candles[0];

        Ok(match orientation {
            StrategyOrientation::Long => candle.high > self.high,
            StrategyOrientation::Short => candle.low < self.low,
        })
    }

    fn required_indicators(&self) -> Vec<IndicatorType> {
        vec![]
    }
}

impl FixedValuesResolution {
    pub fn new(high: f64, low: f64) -> Self {
        Self { high, low }
    }
}


