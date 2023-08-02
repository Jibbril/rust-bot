use super::CalculatesTradeBounds;
use crate::models::{
    candle::Candle, generic_result::GenericResult, strategy_orientation::StrategyOrientation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicPivots {
    pub left_candles: usize,
    pub right_candles: usize,
}

impl CalculatesTradeBounds for DynamicPivots {
    fn get_trade_bounds(
        &self,
        candles: &Vec<Candle>,
        i: usize,
        orientation: &StrategyOrientation,
    ) -> GenericResult<(f64, f64)> {
        todo!()
    }
}
