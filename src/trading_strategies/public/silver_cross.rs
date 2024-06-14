use crate::{
    indicators::indicator_type::IndicatorType,
    models::{
        candle::Candle,
        interval::Interval,
        setups::setup_builder::SetupBuilder,
        strategy_orientation::StrategyOrientation,
        traits::{has_min_length::HasMinLength, requires_indicators::RequiresIndicators, trading_strategy::TradingStrategy},
    },
    resolution_strategies::resolution_strategy::ResolutionStrategy,
};
use chrono::Weekday;
use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

/// # Silver Cross Strategy
///
/// Strategy built on the silver cross event where the 21 SMA crosses the 55
/// SMA in either orientation.
#[derive(Debug, Clone)]
pub struct SilverCross {
    pub short_len: usize,
    pub long_len: usize,
    pub orientation: StrategyOrientation,
    pub trading_days: HashSet<Weekday>,
}

impl SilverCross {
    #[allow(dead_code)] // TODO: Remove once used
    pub fn new_args(orientation: StrategyOrientation, short_len: usize, long_len: usize) -> Self {
        SilverCross {
            orientation,
            short_len,
            long_len,
            trading_days: Self::build_trading_days(),
        }
    }

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

impl HasMinLength for SilverCross {
    fn min_length(&self) -> usize {
        self.long_len
    }
}

impl TradingStrategy for SilverCross {
    #[allow(dead_code)] // TODO: Remove once used
    fn new() -> Self {
        SilverCross {
            orientation: StrategyOrientation::Long,
            short_len: 21,
            long_len: 55,
            trading_days: Self::build_trading_days(),
        }
    }

    fn candles_needed_for_setup(&self) -> usize {
        // TODO: Add real value
        self.long_len
    }

    fn check_last_for_setup(&mut self, _candles: &[Candle]) -> Option<SetupBuilder> {
        todo!()
    }

    fn clone_box(&self) -> Box<dyn TradingStrategy> {
        Box::new(self.clone())
    }

    fn default_resolution_strategy(&self) -> ResolutionStrategy {
        todo!()
    }

    fn orientation(&self) -> StrategyOrientation {
        todo!()
    }

    fn interval(&self) -> crate::models::interval::Interval {
        Interval::Minute1
    }

    fn trading_days(&self) -> HashSet<Weekday> {
        Self::build_trading_days()
    }
}

impl RequiresIndicators for SilverCross {
    fn required_indicators(&self) -> Vec<IndicatorType> {
        vec![
            IndicatorType::SMA(self.short_len),
            IndicatorType::SMA(self.long_len),
        ]
    }
}

impl Display for SilverCross {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Silver Cross")
    }
}
