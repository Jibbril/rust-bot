use serde::{Serialize, Deserialize};

use crate::{
    models::{
        candle::Candle, generic_result::GenericResult, interval::Interval,
        strategy_orientation::StrategyOrientation, timeseries::TimeSeries,
    },
    resolution_strategies::{atr_resolution::AtrResolution, ResolutionStrategy},
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setup {
    pub ticker: String,
    pub candle: Candle,
    pub interval: Interval,
    pub orientation: StrategyOrientation,
    pub resolution_strategy: ResolutionStrategy,
    pub stop_loss: f64,
    pub take_profit: f64,
}

impl Setup {
    #[allow(dead_code)] // TODO: Remove once used
    pub fn dummy() -> Setup {
        let candle = Candle::dummy_data(1, "", 100.0).pop().unwrap();
        Setup {
            ticker: "DUMMY".to_string(),
            candle,
            interval: Interval::Day1,
            orientation: StrategyOrientation::Long,
            resolution_strategy: ResolutionStrategy::ATR(AtrResolution::new(14, 1.0, 1.0)),
            stop_loss: 0.0,
            take_profit: 0.0,
        }
    }
}

pub trait FindsSetups {
    fn find_setups(&self, ts: &TimeSeries) -> GenericResult<Vec<Setup>>;
}

pub trait FindsReverseSetups {
    fn find_reverse_setups(&self, ts: &TimeSeries) -> GenericResult<Vec<Setup>>;
}
