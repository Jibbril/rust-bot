use crate::{
    indicators::indicator_type::IndicatorType,
    models::{
        candle::Candle,
        setups::{setup::Setup, setup_builder::SetupBuilder},
        timeseries::TimeSeries,
    },
};
use anyhow::Result;
use std::fmt::{Debug, Display};

/// #TradingStrategy
///
/// This trait provides the interface for interacting with a Trading Strategy
/// used by the bot. 
pub trait TradingStrategy: Display + Debug {
    fn new() -> Self where Self: Sized;

    /// Returns the number of candles needed from a TimeSeries to calculate 
    /// whether the strategy has yielded a  setup for the last candle provided.
    fn candles_needed_for_setup(&self) -> usize;

    /// Analyzes the given TimeSeries for all historical trade setups triggered 
    /// by the current TradingStrategy.
    fn find_setups(&self, ts: &TimeSeries) -> Result<Vec<Setup>>;

    /// Returns the minimum number of candles needed in a TimeSeries for the
    /// current TradingStrategy to work.
    fn min_length(&self) -> usize;

    /// Returns a Vec<IndicatorType> specifying the indicators needed for the
    /// current TradingStrategy to work .
    fn required_indicators(&self) -> Vec<IndicatorType>;

    /// Checks whether a new Setup has arisen upon the closure of the last
    /// candle provided.
    fn check_last_for_setup(&self, candles: &Vec<Candle>) -> Option<SetupBuilder>;

    /// Returns a boxed clone of the current TradingStrategy
    fn clone_box(&self) -> Box<dyn TradingStrategy>;
}
