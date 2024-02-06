use crate::{models::{
        candle::Candle,
        setups::{setup::Setup, setup_builder::SetupBuilder},
        timeseries::TimeSeries, strategy_orientation::StrategyOrientation, interval::Interval,
    }, resolution_strategies::resolution_strategy::ResolutionStrategy};
use anyhow::Result;
use chrono::Weekday;
use std::{fmt::{Debug, Display}, collections::HashSet};
use super::requires_indicators::RequiresIndicators;

/// #TradingStrategy
///
/// This trait provides the interface for interacting with a Trading Strategy
/// used by the bot. 
pub trait TradingStrategy: Display + Debug + RequiresIndicators {
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

    /// Checks whether a new Setup has arisen upon the closure of the last
    /// candle provided.
    fn check_last_for_setup(&self, candles: &[Candle]) -> Option<SetupBuilder>;

    /// Returns a boxed clone of the current TradingStrategy
    fn clone_box(&self) -> Box<dyn TradingStrategy>;

    /// Returns the default resolution strategy associated with this Trading 
    /// strategy.
    fn default_resolution_strategy(&self) -> ResolutionStrategy;

    /// Returns the StrategyOrientation for this TradingStrategy
    fn orientation(&self) -> StrategyOrientation;

    /// Returns the Interval to be used with this TradingStrategy
    fn interval(&self) -> Interval;

    /// Returns the TradingDays to be used with this TradingStrategy
    fn trading_days(&self) -> HashSet<Weekday>;
}
