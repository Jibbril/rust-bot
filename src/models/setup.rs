use crate::{
    models::{
        candle::Candle, generic_result::GenericResult, interval::Interval,
        strategy_orientation::StrategyOrientation, timeseries::TimeSeries,
    },
    resolution_strategies::ResolutionStrategy,
};
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

pub trait FindsSetups {
    fn find_setups(&self, ts: &TimeSeries) -> GenericResult<Vec<Setup>>;
}

pub trait FindsReverseSetups {
    fn find_reverse_setups(&self, ts: &TimeSeries) -> GenericResult<Vec<Setup>>;
}
