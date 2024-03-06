use super::is_resolution_strategy::IsResolutionStrategy;
use crate::{
    indicators::indicator_type::IndicatorType,
    models::{
        candle::Candle, setups::setup::Setup, strategy_orientation::StrategyOrientation,
        traits::requires_indicators::RequiresIndicators,
    },
};
use anyhow::{anyhow, Result};

struct PercentageResolution {
    initial_value: f64,
    take_profit: f64,
    drawdown: f64,
}

impl IsResolutionStrategy for PercentageResolution {
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
        if candles.is_empty() {
            return Err(anyhow!("No candle provided for percentage resolution."));
        }

        let candle = &candles[0];

        Ok(match orientation {
            StrategyOrientation::Long => {
                candle.close < self.initial_value * (1.0 - self.drawdown / 100.0)
            }
            StrategyOrientation::Short => {
                candle.close > self.initial_value * (1.0 + self.drawdown / 100.0)
            }
        })
    }

    fn take_profit_reached(
        &self,
        orientation: &StrategyOrientation,
        candles: &[Candle],
    ) -> Result<bool> {
        if candles.is_empty() {
            return Err(anyhow!("No candle provided for percentage resolution."));
        }

        let candle = &candles[0];

        Ok(match orientation {
            StrategyOrientation::Long => {
                candle.high > self.initial_value * (1.0 + self.take_profit / 100.0)
            }
            StrategyOrientation::Short => {
                candle.low < self.initial_value * (1.0 - self.take_profit / 100.0)
            }
        })
    }

    fn set_initial_values(&mut self, setup: &Setup) -> Result<()> {
        self.initial_value = setup.candle.close;

        Ok(())
    }
}

impl RequiresIndicators for PercentageResolution {
    fn required_indicators(&self) -> Vec<IndicatorType> {
        vec![]
    }
}
