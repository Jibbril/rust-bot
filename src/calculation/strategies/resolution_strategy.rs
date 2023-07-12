use crate::utils::timeseries::Candle;

use super::strategy::StrategyOrientation;

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
    ) -> (f64, f64) {
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
    ) -> (f64, f64);
}

#[derive(Debug, Clone)]
pub struct AtrResolution {
    length: usize,
    stop_loss_multiple: f64,
    take_profit_multiple: f64,
}

impl CalculatesTradeBounds for AtrResolution {
    fn get_trade_bounds(
        &self,
        candles: &Vec<Candle>,
        i: usize,
        orientation: &StrategyOrientation,
    ) -> (f64, f64) {
        // Check if atr indicator is available on candle, if so, use it

        // If atr indicator not available on candle, calculate it from previous candles

        // Use multiples from self to find limits and return them

        (1.3, 3.4)
    }
}

impl AtrResolution {
    pub fn new(length: usize, stop_loss_multiple: f64, take_profit_multiple: f64) -> Self {
        AtrResolution {
            length,
            stop_loss_multiple,
            take_profit_multiple,
        }
    }
}
