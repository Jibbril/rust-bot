pub mod rsi_basic;
use crate::utils::{
    generic_result::GenericResult,
    timeseries::{Candle, Interval, TimeSeries},
};

use rsi_basic::RsiBasic;

#[derive(Debug, Clone)]
pub enum Strategy {
    #[allow(dead_code)] // TODO: Remove once used
    RsiBasic(RsiBasic),
}

#[derive(Debug, Clone)]
pub enum StrategyOrientation {
    Long,
    Short,
    Both,
}

pub trait FindsSetups {
    fn find_setups(&self, ts: &mut TimeSeries) -> GenericResult<Vec<Setup>>;
}

#[derive(Debug, Clone)]
pub struct Setup {
    pub candle: Candle,
    pub interval: Interval,
    pub orientation: StrategyOrientation,
}
