use std::fmt::{Display, Formatter};
use anyhow::Result;
use crate::{
    indicators::indicator_type::IndicatorType,
    models::{
        candle::Candle,
        setups::{setup::Setup, setup_builder::SetupBuilder},
        strategy_orientation::StrategyOrientation,
        timeseries::TimeSeries,
        traits::trading_strategy::TradingStrategy,
    },
    utils::math::sma, resolution_strategies::{dynamic_pivot::DynamicPivotResolution, resolution_strategy::ResolutionStrategy},
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
#[derive(Debug, Clone)]
pub struct JB1 {
    pmarp_len: usize,
    pmarp_lookback: usize,
    rsi_len: usize,
    rsi_ma_len: usize,
    short_ema_len: usize,
    long_ema_len: usize,
}

impl TradingStrategy for JB1 {
    fn new() -> Self {
        Self {
            pmarp_len: 20,
            pmarp_lookback: 350,
            rsi_len: 7,
            rsi_ma_len: 4,
            short_ema_len: 21,
            long_ema_len: 55,
        }
    }

    fn find_setups(&self, ts: &TimeSeries) -> Result<Vec<Setup>> {
        let mut setups = vec![];

        for window in ts.candles.windows(6) {
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

    fn required_indicators(&self) -> Vec<IndicatorType> {
        vec![
            IndicatorType::EMA(self.short_ema_len),
            IndicatorType::EMA(self.long_ema_len),
            IndicatorType::PMARP(self.pmarp_len, self.pmarp_lookback),
            IndicatorType::RSI(self.rsi_len),
        ]
    }

    fn check_last_for_setup(&self, candles: &[Candle]) -> Option<SetupBuilder> {
        if candles.len() < self.rsi_ma_len + 1 {
            return None;
        }

        let current = candles.last()?;
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
            .get(&IndicatorType::PMARP(self.pmarp_len, self.pmarp_lookback))?
            .as_pmarp()?;

        let (curr_rsi_sma, prev_rsi_sma) = self.calculate_rsi_smas(candles)?;

        let short_ema_is_higher = short_ema.value > long_ema.value;
        let pmarp_is_low = pmarp.value < 10.0;
        let rsi_is_positive = curr_rsi_sma > prev_rsi_sma;

        if short_ema_is_higher && pmarp_is_low && rsi_is_positive {
            // TODO: Set resolution to 4.5% drawdown once that resolution
            // strategy is implemented.
            let dp = DynamicPivotResolution::new();
            let resolution_strategy = ResolutionStrategy::DynamicPivot(dp);

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
}

impl Display for JB1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "JB1")
    }
}
