pub mod atr_resolution;
use crate::models::{candle::Candle, generic_result::GenericResult, strategy_orientation::StrategyOrientation};
use atr_resolution::AtrResolution;

#[derive(Debug, Clone)]
pub enum ResolutionStrategy {
    ATR(AtrResolution),
}

impl CalculatesTradeBounds for ResolutionStrategy {
    fn get_trade_bounds(
        &self,
        candles: &Vec<Candle>,
        i: usize,
        orientation: &StrategyOrientation,
    ) -> GenericResult<(f64, f64)> {
        match self {
            ResolutionStrategy::ATR(atr) => atr.get_trade_bounds(candles, i, orientation),
        }
    }
}

pub trait CalculatesTradeBounds {
    fn get_trade_bounds(
        &self,
        candles: &Vec<Candle>,
        i: usize,
        orientation: &StrategyOrientation,
    ) -> GenericResult<(f64, f64)>;
}
