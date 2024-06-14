use crate::{
    models::{
        candle::Candle, interval::Interval, setups::setup_builder::SetupBuilder,
        strategy_orientation::StrategyOrientation, traits::has_min_length::HasMinLength,
        traits::requires_indicators::RequiresIndicators,
    },
    resolution_strategies::resolution_strategy::ResolutionStrategy,
};
use chrono::Weekday;
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
};

/// #TradingStrategy
///
/// This trait provides the interface for interacting with a Trading Strategy
/// used by the bot.
pub trait TradingStrategy: Display + Debug + RequiresIndicators + HasMinLength {
    fn new() -> Self
    where
        Self: Sized;

    /// Returns the number of candles needed from a TimeSeries to calculate
    /// whether the strategy has yielded a  setup for the last candle provided.
    fn candles_needed_for_setup(&self) -> usize;

    /// Checks whether a new Setup has arisen upon the closure of the last
    /// candle provided.
    fn check_last_for_setup(&mut self, candles: &[Candle]) -> Option<SetupBuilder>;

    /// Returns a boxed clone of the current TradingStrategy
    fn clone_box(&self) -> Box<dyn TradingStrategy>;

    /// Returns the default resolution strategy associated with this Trading
    /// strategy.
    fn default_resolution_strategy(&self) -> ResolutionStrategy;

    /// Returns the resolution strategy of this Trading Strategy. If none has
    /// been set then it returns the default strategy.
    // fn resolution_strategy(&self) -> ResolutionStrategy;

    /// Sets the provided ResolutionStrategy for this strategy.
    // fn set_resolution_strategy(&mut self, strat: ResolutionStrategy);

    /// Returns the StrategyOrientation for this TradingStrategy
    fn orientation(&self) -> StrategyOrientation;

    /// Returns the Interval to be used with this TradingStrategy
    fn interval(&self) -> Interval;

    /// Returns the TradingDays to be used with this TradingStrategy
    fn trading_days(&self) -> HashSet<Weekday>;
}
