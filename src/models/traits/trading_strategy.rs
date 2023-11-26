use crate::{
    indicators::indicator_type::IndicatorType,
    models::{setups::{setup::Setup, setup_builder::SetupBuilder}, timeseries::TimeSeries, candle::Candle},
};
use anyhow::Result;
use std::fmt::{Debug, Display};

pub trait TradingStrategy: Display + Debug {
    fn max_length(&self) -> usize;
    fn candles_needed_for_setup(&self) -> usize;
    fn find_setups(&self, ts: &TimeSeries) -> Result<Vec<Setup>>;
    fn required_indicators(&self) -> Vec<IndicatorType>;
    fn check_last_for_setup(&self, candles: &Vec<Candle>) -> Option<SetupBuilder>;
    fn clone_box(&self) -> Box<dyn TradingStrategy>;
}