use crate::{
    indicators::indicator_type::IndicatorType,
    models::{
        candle::Candle,
        interval::Interval,
        setups::setup_builder::SetupBuilder,
        strategy_orientation::StrategyOrientation,
        traits::{requires_indicators::RequiresIndicators, trading_strategy::TradingStrategy},
    },
    resolution_strategies::resolution_strategy::ResolutionStrategy,
};
use chrono::Weekday;
use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

/// # One Activation Strategy
///
/// Dummy strategy used for testing purposes. Always returns that entry
/// conditions have been reached on first ask, after that always negative.
/// Same for take-profit and stop-loss.
///
/// ## Directionality
/// - Long
///
/// ## Interval
/// - Any
///
/// ## Entry Conditions
/// - Always positive on first ask.
///
/// ## Take-profit
/// - Always positive one first ask.
///
/// ## Stop-loss
/// - Always positive on first ask.
///
/// ## Trading days
/// - All
///
#[derive(Debug, Clone)]
pub struct AlwaysTrueStrategy {
    trading_days: HashSet<Weekday>,
    triggered: bool
}

impl TradingStrategy for AlwaysTrueStrategy {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            trading_days: Self::build_trading_days(),
            triggered: false
        }
    }

    fn candles_needed_for_setup(&self) -> usize {
        1
    }

    fn min_length(&self) -> usize {
        1
    }

    fn check_last_for_setup(&mut self, candles: &[Candle]) -> Option<SetupBuilder> {
        if self.triggered {
            return None;
        }

        let sb = SetupBuilder::new()
            .candle(&candles[0])
            .orientation(&StrategyOrientation::Long);

        self.triggered = true;

        Some(sb)
    }

    fn clone_box(&self) -> Box<dyn TradingStrategy> {
        Box::new(self.clone())
    }

    fn default_resolution_strategy(&self) -> ResolutionStrategy {
        todo!()
    }

    fn orientation(&self) -> StrategyOrientation {
        StrategyOrientation::Long
    }

    fn interval(&self) -> Interval {
        Interval::Minute1
    }

    fn trading_days(&self) -> HashSet<Weekday> {
        self.trading_days.clone()
    }
}

impl AlwaysTrueStrategy {
    fn build_trading_days() -> HashSet<Weekday> {
        let mut set = HashSet::new();

        set.insert(Weekday::Mon);
        set.insert(Weekday::Tue);
        set.insert(Weekday::Wed);
        set.insert(Weekday::Thu);
        set.insert(Weekday::Fri);
        set.insert(Weekday::Sat);
        set.insert(Weekday::Sun);

        set
    }
}

impl RequiresIndicators for AlwaysTrueStrategy {
    fn required_indicators(&self) -> Vec<IndicatorType> {
        vec![]
    }
}

impl Display for AlwaysTrueStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NewStrategy")
    }
}
