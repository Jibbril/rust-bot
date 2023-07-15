use crate::{
    models::{
        generic_result::GenericResult,
        timeseries::TimeSeries, candle::Candle, interval::Interval,
    },
    resolution_strategies::ResolutionStrategy,
};

use super::StrategyOrientation;

pub trait FindsSetups {
    fn find_setups(&self, ts: &TimeSeries) -> GenericResult<Vec<Setup>>;
}

#[derive(Debug, Clone)]
pub struct Setup {
    pub candle: Candle,
    pub interval: Interval,
    pub orientation: StrategyOrientation,
    pub resolution_strategy: ResolutionStrategy,
    pub stop_loss: f64,
    pub take_profit: f64,
}
