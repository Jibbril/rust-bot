use crate::{
    indicators::indicator_type::IndicatorType,
    models::{setups::setup::Setup, timeseries::TimeSeries},
};
use anyhow::Result;
use std::fmt::Display;

pub trait TradingStrategy: Display {
    fn max_length(&self) -> usize;
    fn find_setups(&self, ts: &TimeSeries) -> Result<Vec<Setup>>;
    fn find_reverse_setups(&self, ts: &TimeSeries) -> Result<Vec<Setup>>;
    fn required_indicators(&self) -> Vec<IndicatorType>;
}