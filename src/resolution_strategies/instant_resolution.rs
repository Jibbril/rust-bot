use crate::models::traits::requires_indicators::RequiresIndicators;
use crate::{
    models::{candle::Candle, setups::setup::Setup, strategy_orientation::StrategyOrientation},
    resolution_strategies::is_resolution_strategy::IsResolutionStrategy,
};
use serde::{Deserialize, Serialize};

/// # InstantResolution
///
/// Dummy resolution strategy which always fires off positively for both
/// stop-loss and take-profit. Only intended for use in testing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstantResolution;

impl RequiresIndicators for InstantResolution {
    fn required_indicators(&self) -> Vec<crate::indicators::indicator_type::IndicatorType> {
        vec![]
    }
}

impl IsResolutionStrategy for InstantResolution {
    fn n_candles_stop_loss(&self) -> usize {
        1
    }

    fn n_candles_take_profit(&self) -> usize {
        1
    }

    fn stop_loss_reached(
        &self,
        _orientation: &StrategyOrientation,
        _candles: &[Candle],
    ) -> anyhow::Result<bool> {
        Ok(true)
    }

    fn take_profit_reached(
        &self,
        _orientation: &StrategyOrientation,
        _candles: &[Candle],
    ) -> anyhow::Result<bool> {
        Ok(true)
    }

    fn set_initial_values(&mut self, _setup: &Setup) -> anyhow::Result<()> {
        Ok(())
    }
}
