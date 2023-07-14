use crate::{
    calculation::resolution_strategies::ResolutionStrategy,
    utils::{
        generic_result::GenericResult,
        timeseries::{Candle, Interval, TimeSeries},
    },
};

use super::StrategyOrientation;

pub trait FindsSetups {
    fn find_setups(&self, ts: &mut TimeSeries) -> GenericResult<Vec<Setup>>;
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
