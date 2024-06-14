use crate::{
    indicators::{indicator_type::IndicatorType, rsi::RSI},
    models::{
        candle::Candle,
        interval::Interval,
        setups::setup_builder::SetupBuilder,
        strategy_orientation::StrategyOrientation,
        traits::{has_min_length::HasMinLength, requires_indicators::RequiresIndicators, trading_strategy::TradingStrategy},
    },
    resolution_strategies::resolution_strategy::ResolutionStrategy,
};
use chrono::{Datelike, Weekday};
use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

/// # RSIBasic
///
/// Simple RSI strategy that initiates entries when RSI is returning from
/// extremes.
///
/// ## Example:
///
/// If the upper band is set to 70.0 and the current RSI is above 70.0, a short
/// setup will be triggered when the RSI goes below 70.
#[derive(Debug, Clone)]
pub struct RsiBasic {
    pub len: usize,
    pub upper_band: f64,
    pub lower_band: f64,
    pub orientation: StrategyOrientation,
    pub trading_days: HashSet<Weekday>,
}

impl RsiBasic {
    #[allow(dead_code)] // TODO: Remove once used
    pub fn new_args(
        len: usize,
        upper_band: f64,
        lower_band: f64,
        orientation: StrategyOrientation,
    ) -> Self {
        RsiBasic {
            len,
            upper_band,
            lower_band,
            orientation,
            trading_days: Self::build_trading_days(),
        }
    }

    fn get_orientation(&self, prev: &RSI, current: &RSI) -> Option<StrategyOrientation> {
        let long_condition = prev.value < self.lower_band && current.value > self.lower_band;
        let _short_condition = prev.value > self.upper_band && current.value < self.upper_band;

        if long_condition {
            Some(StrategyOrientation::Long)
        // } else if short_condition {
        //     Some(StrategyOrientation::Short)
        } else {
            None
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

impl HasMinLength for RsiBasic {
    fn min_length(&self) -> usize {
        self.len
    }
}

impl TradingStrategy for RsiBasic {
    fn new() -> Self {
        RsiBasic {
            len: 14,
            upper_band: 70.0,
            lower_band: 30.0,
            orientation: StrategyOrientation::Long,
            trading_days: Self::build_trading_days(),
        }
    }

    fn candles_needed_for_setup(&self) -> usize {
        // TODO: Add real value
        self.len
    }

    fn check_last_for_setup(&mut self, candles: &[Candle]) -> Option<SetupBuilder> {
        if candles.len() < 2 {
            return None;
        }

        let current = candles.last()?;

        let is_active_day = self.trading_days.contains(&current.timestamp.weekday());
        if !is_active_day {
            return None;
        }

        let prev = candles.get(candles.len() - 2)?;

        let key = IndicatorType::RSI(self.len);
        let prev_rsi = prev.clone_indicator(&key).ok()?.as_rsi()?;
        let current_rsi = current.clone_indicator(&key).ok()?.as_rsi()?;

        let orientation = self.get_orientation(&prev_rsi, &current_rsi)?;

        let sb = SetupBuilder::new()
            .candle(&current)
            .orientation(&orientation);

        Some(sb)
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

    fn interval(&self) -> Interval {
        Interval::Minute1
    }

    fn trading_days(&self) -> HashSet<Weekday> {
        self.trading_days.clone()
    }
}

impl RequiresIndicators for RsiBasic {
    fn required_indicators(&self) -> Vec<IndicatorType> {
        vec![IndicatorType::RSI(self.len)]
    }
}

impl Display for RsiBasic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RSI Basic")
    }
}
