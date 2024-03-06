use crate::models::{
    candle::Candle, setups::setup::Setup, strategy_orientation::StrategyOrientation,
    traits::requires_indicators::RequiresIndicators,
};
use anyhow::Result;

pub trait IsResolutionStrategy: RequiresIndicators {
    /// Number of candles needed to check whether stop-loss has been reached
    fn n_candles_stop_loss(&self) -> usize;

    /// Number of candles needed to check whether take-profit has been reached
    fn n_candles_take_profit(&self) -> usize;

    /// Check whether stop-loss has been reached for the last candle
    fn stop_loss_reached(
        &self,
        orientation: &StrategyOrientation,
        candles: &[Candle],
    ) -> Result<bool>;

    /// Check whether take-profit has been reached for the last candle
    fn take_profit_reached(
        &self,
        orientation: &StrategyOrientation,
        candles: &[Candle],
    ) -> Result<bool>;

    /// Set initial values from setup if applicable
    fn set_initial_values(&mut self, setup: &Setup) -> Result<()>;
}
