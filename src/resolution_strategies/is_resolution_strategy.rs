use anyhow::Result;
use crate::models::{strategy_orientation::StrategyOrientation, candle::Candle, traits::requires_indicators::RequiresIndicators};

pub trait IsResolutionStrategy: RequiresIndicators {
    /// Number of candles needed to check whether stop-loss has been reached
    fn n_candles_stop_loss(&self) -> usize;

    /// Number of candles needed to check whether take-profit has been reached
    fn n_candles_take_profit(&self) -> usize;

    /// Check whether stop-loss has been reached for the last candle
    fn stop_loss_reached(&self, orientation: &StrategyOrientation, candles: &[Candle]) -> Result<bool>;

    /// Check whether take-profit has been reached for the last candle
    fn take_profit_reached(&self, orientation: &StrategyOrientation, candles: &[Candle]) -> Result<bool>;
}
