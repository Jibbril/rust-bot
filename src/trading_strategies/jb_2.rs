use std::{fmt::{Display, Formatter}, collections::HashSet};
use anyhow::Result;
use chrono::{Weekday, Datelike};
use crate::{models::{traits::{trading_strategy::TradingStrategy, requires_indicators::RequiresIndicators}, setups::{setup::Setup, setup_builder::SetupBuilder}, timeseries::TimeSeries, strategy_orientation::StrategyOrientation, candle::Candle, ma_type::MAType, interval::Interval}, indicators::indicator_type::IndicatorType, resolution_strategies::{resolution_strategy::ResolutionStrategy, pmarp_or_bbwp_vs_percentage::PmarpOrBbwpVsPercentageResolution}};

/// # JB 2
///
/// Trading strategy based on the strategy presented by Eric Crown in [this](https://www.youtube.com/watch?v=sB-8FTHA9hI&list=WL&index=8)
/// video. It is based around identifying periods of low PMARP and BBWP values
/// which initiate long-entries. Important to note DO NOT TRADE THURSDAYS AND
/// SUNDAYS. 
///
/// ## Directionality
/// - Long
///
/// ## Interval
/// - 15 Minute
///
/// ## Indicators
/// - PMARP, 100 lookback, 21 length, EMA-based 
/// - BBWP, 13 sample length, 5 sma length, 252 lookback
///
/// ## Entry conditions
/// - PMARP < 20
/// - BBWP < 50
/// - BBWP MA negative (current close must be lower than previous close)
///
/// ## Take Profit
/// - PMARP > 65 or BBWP > 80
///
/// ## Stop Loss
/// - 3% drawdown
///
/// ## Trading days
/// - Monday
/// - Tuesday
/// - Wednesday
/// - Friday
/// - Saturday
#[derive(Debug, Clone)]
pub struct JB2 {
    pmarp_len: usize,
    pmarp_lookback: usize,
    pmarp_ma_type: MAType,
    bbwp_len: usize,
    bbwp_lookback: usize,
    trading_days: HashSet<Weekday>
}

impl TradingStrategy for JB2 {
    fn new() -> Self where Self: Sized {
        Self {
            pmarp_len: 21,
            pmarp_lookback: 100,
            pmarp_ma_type: MAType::EMA,
            bbwp_len: 13,
            bbwp_lookback: 252,
            trading_days: Self::build_trading_days(),
        }
    }

    fn candles_needed_for_setup(&self) -> usize {
        2
    }

    fn find_setups(&self, ts: &TimeSeries) -> Result<Vec<Setup>> {
        let mut setups = vec![];

        for window in ts.candles.windows(self.candles_needed_for_setup()) {
            if let Some(sb) = self.check_last_for_setup(&window) {
                let setup = sb
                    .ticker(&ts.symbol)
                    .interval(&ts.interval)
                    .build()?;
                setups.push(setup);
            }
        }

        Ok(setups)
    }

    fn min_length(&self) -> usize {
        self.bbwp_lookback + 150
    }

    fn check_last_for_setup(&self, candles: &[Candle]) -> Option<SetupBuilder> {
        if candles.len() < 2 {
            return None;
        }

        let len = candles.len();
        let current = &candles[len-1];
        let previous = &candles[len-2];

        let is_active_day = self.trading_days.contains(&current.timestamp.weekday());
        if !is_active_day {
            return None;
        }

        let pmarp = current
            .indicators
            .get(&IndicatorType::PMARP(self.pmarp_len, self.pmarp_lookback, self.pmarp_ma_type))?
            .as_pmarp()?;

        let bbwp_type = IndicatorType::BBWP(self.bbwp_len,self.bbwp_lookback);
        let current_bbwp = current.indicators.get(&bbwp_type)?.as_bbwp()?;
        let prev_bbwp = previous.indicators.get(&bbwp_type)?.as_bbwp()?;

        let bbwp_ma_sloped_down = current_bbwp.sma < prev_bbwp.sma;
        let bbwp_low_enough = current_bbwp.value < 0.5;
        let pmarp_low_enough = pmarp.value < 0.2;


        if bbwp_ma_sloped_down && bbwp_low_enough && pmarp_low_enough {
            let resolution_strategy = self.default_resolution_strategy();
            let sb = SetupBuilder::new()
                .candle(&current)
                .orientation(&StrategyOrientation::Long)
                .resolution_strategy(&resolution_strategy);

            Some(sb)
        } else {
            None
        }
    }

    fn clone_box(&self) -> Box<dyn TradingStrategy> {
        Box::new(self.clone())
    }

    fn default_resolution_strategy(&self) -> ResolutionStrategy {
        let p = PmarpOrBbwpVsPercentageResolution {
            initial_value: None,
            drawdown_threshold: 3.0,
            pmarp_threshold: 0.65,
            pmarp_len: self.pmarp_len,
            pmarp_lookback: self.pmarp_lookback,
            pmarp_ma_type: self.pmarp_ma_type,
            bbwp_threshold: 0.80,
            bbwp_len: self.bbwp_len,
            bbwp_lookback: self.bbwp_lookback,
            bbwp_sma_len: 5,
        };

        ResolutionStrategy::PmarpOrBbwpVsPercentage(p)
    }

    fn orientation(&self) -> StrategyOrientation {
        StrategyOrientation::Long
    }

    fn interval(&self) -> Interval {
        Interval::Minute15
    }

    fn trading_days(&self) -> HashSet<Weekday> {
        self.trading_days.clone()
    }
}

impl JB2 {
    fn build_trading_days() -> HashSet<Weekday> {
        let mut set = HashSet::new();

        set.insert(Weekday::Mon);
        set.insert(Weekday::Tue);
        set.insert(Weekday::Wed);
        set.insert(Weekday::Fri);
        set.insert(Weekday::Sat);

        set
    }
}

impl RequiresIndicators for JB2 {
    fn required_indicators(&self) -> Vec<IndicatorType> {
        vec![
            // PMARP is EMA based, add here to ensure existence everywhere this 
            // strategy is used and reduce number of calculations.
            IndicatorType::EMA(self.pmarp_len),
            IndicatorType::PMARP(self.pmarp_len, self.pmarp_lookback, self.pmarp_ma_type),
            IndicatorType::BBWP(self.bbwp_len, self.bbwp_lookback)
        ]
    }
}

impl Display for JB2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "JB2")
    }
}
