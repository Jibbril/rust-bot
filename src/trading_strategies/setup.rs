use crate::{
    models::{
        candle::Candle, generic_result::GenericResult, interval::Interval, timeseries::TimeSeries,
    },
    resolution_strategies::ResolutionStrategy,
};

use super::StrategyOrientation;

pub trait FindsSetups {
    fn find_setups(&self, ts: &TimeSeries) -> GenericResult<Vec<Setup>>;
}

#[derive(Debug, Clone)]
pub struct Setup {
    pub ticker: String,
    pub candle: Candle,
    pub interval: Interval,
    pub orientation: StrategyOrientation,
    pub resolution_strategy: ResolutionStrategy,
    pub stop_loss: f64,
    pub take_profit: f64,
}
