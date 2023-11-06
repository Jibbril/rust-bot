use std::fmt::Display;
use anyhow::Result;
use crate::{models::{timeseries::TimeSeries, setups::setup::Setup}, indicators::indicator_type::IndicatorType};

pub trait TradingStrategy: Display {
    fn max_length(&self) -> usize;
    fn find_setups(&self, ts: &TimeSeries) -> Result<Vec<Setup>>;
    fn find_reverse_setups(&self, ts: &TimeSeries) -> Result<Vec<Setup>>;
    fn required_indicators(&self) -> Vec<IndicatorType>;
}