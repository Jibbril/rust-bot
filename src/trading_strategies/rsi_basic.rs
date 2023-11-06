use crate::{
    indicators::{indicator_type::IndicatorType, rsi::RSI},
    models::{
        setups::setup::Setup,
        strategy_orientation::StrategyOrientation,
        timeseries::TimeSeries,
        traits::trading_strategy::TradingStrategy,
    },
    resolution_strategies::{
        atr_resolution::AtrResolution, CalculatesStopLosses, CalculatesTakeProfits,
        ResolutionStrategy,
    },
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
    pub fn new(
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

    pub fn new_default() -> Self {
        RsiBasic {
            len: 14,
            upper_band: 70.0,
            lower_band: 30.0,
            orientation: StrategyOrientation::Long,
        }
    }

    fn get_orientation(&self, prev: &RSI, current: &RSI) -> Option<StrategyOrientation> {
        let long_condition = prev.value < self.lower_band && current.value > self.lower_band;
        let short_condition = prev.value > self.upper_band && current.value < self.upper_band;

        if long_condition {
            Some(StrategyOrientation::Long)
        } else if short_condition {
            Some(StrategyOrientation::Short)
        } else {
            None
        }
    }

    fn get_reverse_orientation(&self, prev: &RSI, current: &RSI) -> Option<StrategyOrientation> {
        if let Some(orientation) = self.get_orientation(prev, current) {
            match orientation {
                StrategyOrientation::Long => Some(StrategyOrientation::Short),
                StrategyOrientation::Short => Some(StrategyOrientation::Long),
            }
        } else {
            None
        }
    }

    fn find_setups_by_direction(&self, ts: &TimeSeries, reversed: bool) -> Result<Vec<Setup>> {
        let len = self.len;
        let key = IndicatorType::RSI(len);
        let mut setups: Vec<Setup> = Vec::new();

        for (i, candle) in ts.candles.iter().enumerate().skip(1) {
            let prev_candle = &ts.candles[i - 1];

            let prev_rsi = prev_candle.get_indicator(&key)?.as_rsi();
            let current_rsi = candle.get_indicator(&key)?.as_rsi();

            if let (Some(prev), Some(current)) = (prev_rsi, current_rsi) {
                let orientation = if reversed {
                    self.get_reverse_orientation(&prev, &current)
                } else {
                    self.get_orientation(&prev, &current)
                };

                if let Some(orientation) = orientation {
                    let atr = AtrResolution::new(14, 2.0, 1.0);
                    let resolution_strategy = ResolutionStrategy::ATR(atr);
                    let take_profit = resolution_strategy.calculate_take_profit(
                        &ts.candles,
                        i,
                        &orientation,
                        len,
                    )?;
                    let stop_loss = resolution_strategy.calculate_stop_loss(
                        &ts.candles,
                        i,
                        &orientation,
                        len,
                    )?;

                    setups.push(Setup {
                        ticker: ts.ticker.clone(),
                        candle: candle.clone(),
                        interval: ts.interval.clone(),
                        orientation,
                        take_profit_resolution: resolution_strategy.clone(),
                        stop_loss_resolution: resolution_strategy,
                        stop_loss,
                        take_profit,
                    });
                }
            }
        }

        Ok(setups)
    }
}

impl TradingStrategy for RsiBasic {
    fn find_reverse_setups(&self, ts: &TimeSeries) -> Result<Vec<Setup>> {
        self.find_setups_by_direction(ts, true)   
    }

    fn find_setups(&self, ts: &TimeSeries) -> Result<Vec<Setup>> {
        self.find_setups_by_direction(ts, false)
    }

    fn required_indicators(&self) -> Vec<IndicatorType> {
        vec![IndicatorType::RSI(self.len)]
    }

    fn max_length(&self) -> usize {
        self.len
    }
}

impl Display for RsiBasic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RSI Basic")
    }
}
