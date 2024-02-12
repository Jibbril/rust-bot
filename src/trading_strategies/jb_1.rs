use std::{fmt::{Display, Formatter}, collections::HashSet};
use anyhow::Result;
use chrono::{Weekday, Datelike};
use crate::{
    indicators::indicator_type::IndicatorType,
    models::{
        candle::Candle,
        setups::{setup::Setup, setup_builder::SetupBuilder},
        strategy_orientation::StrategyOrientation,
        timeseries::TimeSeries,
        traits::{trading_strategy::TradingStrategy, requires_indicators::RequiresIndicators}, ma_type::MAType, interval::Interval,
    },
    utils::math::sma, resolution_strategies::{resolution_strategy::ResolutionStrategy, pmarp_vs_percentage::PmarpVsPercentageResolution},
};

/// # JB 1
///
/// Trading strategy based on the strategy presented by Eric Crown in [this](https://youtu.be/ijeujHi4ZRM?si=JHVlQQJ1fFdebahz)
/// video. It is based around identifying ema crosses when the PMARP indicator
/// is low and a moving average for a momentum oscillator (here RSI) is
/// positive.
///
/// ## Directionality
/// - Long
///
/// ## Interval
/// - 1 Hour
///
/// ## Entry conditions
/// - 21 EMA > 55 EMA
/// - PMARP < 10
/// - RSI moving average of length 4 is sloped positively.
///
/// ## Take Profit
/// - PMARP > 67
///
/// ## Stop Loss
/// - 4.5% drawdown
///
/// ## Trading days
/// - All 
///
#[derive(Debug, Clone)]
pub struct JB1 {
    pmarp_len: usize,
    pmarp_lookback: usize,
    pmarp_ma_type: MAType,
    rsi_len: usize,
    rsi_ma_len: usize,
    short_ema_len: usize,
    long_ema_len: usize,
    trading_days: HashSet<Weekday>
}

impl TradingStrategy for JB1 {
    fn new() -> Self {
        Self {
            pmarp_len: 20,
            pmarp_lookback: 350,
            pmarp_ma_type: MAType::VWMA,
            rsi_len: 7,
            rsi_ma_len: 4,
            short_ema_len: 21,
            long_ema_len: 55,
            trading_days: Self::build_trading_days(),
        }
    }

    fn find_setups(&self, ts: &TimeSeries) -> Result<Vec<Setup>> {
        let mut setups = vec![];

        for window in ts.candles.windows(self.candles_needed_for_setup()) {
            if let Some(sb) = self.check_last_for_setup(&window) {
                let setup = sb
                    .ticker(&ts.ticker)
                    .interval(&ts.interval)
                    .build()?;
                setups.push(setup);
            }
        }

        Ok(setups)
    }

    fn min_length(&self) -> usize {
        // Additional candles needed to ensure PMARP has had time to properly
        // start populating (initial values are on very low lookback so not
        // fully reliable).
        self.pmarp_lookback + 150
    }

    fn check_last_for_setup(&self, candles: &[Candle]) -> Option<SetupBuilder> {
        if candles.len() < self.rsi_ma_len + 1 {
            return None;
        }

        let current = candles.last()?;

        let is_active_day = self.trading_days.contains(&current.timestamp.weekday());
        if !is_active_day {
            return None;
        }

        let short_ema = current
            .indicators
            .get(&IndicatorType::EMA(self.short_ema_len))?
            .as_ema()?;
        let long_ema = current
            .indicators
            .get(&IndicatorType::EMA(self.long_ema_len))?
            .as_ema()?;
        let pmarp = current
            .indicators
            .get(&IndicatorType::PMARP(self.pmarp_len, self.pmarp_lookback, self.pmarp_ma_type))?
            .as_pmarp()?;

        let (curr_rsi_sma, prev_rsi_sma) = self.calculate_rsi_smas(candles)?;

        let short_ema_is_higher = short_ema.value > long_ema.value;
        let pmarp_is_low = pmarp.value < 0.1;
        let rsi_is_positive = curr_rsi_sma > prev_rsi_sma;

        if short_ema_is_higher && pmarp_is_low && rsi_is_positive {
            let resolution_strategy = self.default_resolution_strategy();

            Some(
                SetupBuilder::new()
                    .candle(&current)
                    .orientation(&StrategyOrientation::Long)
                    .resolution_strategy(&resolution_strategy),
            )
        } else {
            None
        }
    }

    fn clone_box(&self) -> Box<dyn TradingStrategy> {
        Box::new(self.clone())
    }

    fn candles_needed_for_setup(&self) -> usize {
        // We need to create the two last rsi moving averages
        self.rsi_ma_len + 1
    }

    fn default_resolution_strategy(&self) -> ResolutionStrategy {
        let p = PmarpVsPercentageResolution {
            pmarp_ma_type: MAType::VWMA,
            pmarp_threshhold: 0.68,
            initial_value: None,
            drawdown_threshold: 4.5,
            pmarp_len: 20,
            pmarp_lookback: 350,
        };

        ResolutionStrategy::PmarpVsPercentage(p)
    }

    fn orientation(&self) -> StrategyOrientation {
        StrategyOrientation::Long
    }

    fn interval(&self) -> Interval {
        Interval::Hour1
    }

    fn trading_days(&self) -> HashSet<Weekday> {
        self.trading_days.clone()
    }
}

impl JB1 {
    #[allow(dead_code)]

    fn calculate_rsi_smas(&self, candles: &[Candle]) -> Option<(f64, f64)> {
        let len = candles.len();
        if len < self.rsi_ma_len + 1 {
            return None;
        };

        let start = len - self.rsi_ma_len;

        // Get current values
        let segment = &candles[start..len];
        let curr_vals = self.get_rsi_values(segment);

        // Get previous values
        let segment = &candles[start - 1..len - 1];
        let prev_vals = self.get_rsi_values(segment);

        if curr_vals.len() != self.rsi_ma_len || prev_vals.len() != self.rsi_ma_len {
            return None;
        };

        let current_rsi_sma = sma(&curr_vals);
        let prev_rsi_sma = sma(&prev_vals);

        Some((current_rsi_sma, prev_rsi_sma))
    }

    fn get_rsi_values(&self, segment: &[Candle]) -> Vec<f64> {
        let key = IndicatorType::RSI(self.rsi_len);

        segment.iter()
            .filter_map(|c| {
                c.indicators
                    .get(&key)
                    .and_then(|i| i.as_rsi().map(|rsi| rsi.value))
            })
            .collect()
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

impl RequiresIndicators for JB1 {
    fn required_indicators(&self) -> Vec<IndicatorType> {
        let mut v = vec![
            IndicatorType::EMA(self.short_ema_len),
            IndicatorType::EMA(self.long_ema_len),
            IndicatorType::PMARP(self.pmarp_len, self.pmarp_lookback, self.pmarp_ma_type),
            IndicatorType::RSI(self.rsi_len),
        ];

        // PMARP may need EMA of same length.
        if self.pmarp_len != self.short_ema_len && self.pmarp_len != self.long_ema_len {
            v.insert(0, IndicatorType::EMA(self.pmarp_len));
        };

        v
    }
}

impl Display for JB1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "JB1")
    }
}
