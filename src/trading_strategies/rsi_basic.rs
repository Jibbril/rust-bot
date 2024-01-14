use crate::{
    indicators::{indicator_type::IndicatorType, rsi::RSI},
    models::{
        candle::Candle,
        setups::{setup::Setup, setup_builder::SetupBuilder},
        strategy_orientation::StrategyOrientation,
        timeseries::TimeSeries,
        traits::{trading_strategy::TradingStrategy, requires_indicators::RequiresIndicators},
    }, resolution_strategies::{fixed_values::FixedValuesResolution, resolution_strategy::ResolutionStrategy},
};
use anyhow::Result;
use std::fmt::{Display, Formatter};

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
}

impl TradingStrategy for RsiBasic {
    fn new() -> Self {
        RsiBasic {
            len: 14,
            upper_band: 70.0,
            lower_band: 30.0,
            orientation: StrategyOrientation::Long,
        }
    }

    fn find_setups(&self, ts: &TimeSeries) -> Result<Vec<Setup>> {
        let len = self.len;
        let key = IndicatorType::RSI(len);
        let mut setups: Vec<Setup> = Vec::new();

        for (i, candle) in ts.candles.iter().enumerate().skip(1) {
            let prev_candle = &ts.candles[i - 1];

            let prev_rsi = prev_candle.clone_indicator(&key)?.as_rsi();
            let current_rsi = candle.clone_indicator(&key)?.as_rsi();

            if let (Some(prev), Some(current)) = (prev_rsi, current_rsi) {
                let orientation = self.get_orientation(&prev, &current);

                if let Some(orientation) = orientation {
                    let take_profit = candle.close * 1.05;
                    let stop_loss = candle.close * 0.95;
                    let fv = FixedValuesResolution::new(take_profit, stop_loss);
                    let resolution_strategy = ResolutionStrategy::FixedValues(fv);

                    setups.push(Setup {
                        ticker: ts.ticker.clone(),
                        candle: candle.clone(),
                        interval: ts.interval.clone(),
                        orientation,
                        resolution_strategy: Some(resolution_strategy),
                        stop_loss: Some(stop_loss),
                        take_profit: Some(take_profit),
                    });
                }
            }
        }

        Ok(setups)
    }

    fn min_length(&self) -> usize {
        self.len
    }

    fn candles_needed_for_setup(&self) -> usize {
        // TODO: Add real value
        self.len
    }

    fn check_last_for_setup(&self, candles: &[Candle]) -> Option<SetupBuilder> {
        if candles.len() < 2 {
            return None;
        }

        let current = candles.last()?;
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
