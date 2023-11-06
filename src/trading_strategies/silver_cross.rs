use anyhow::Result;

use crate::{
    indicators::{indicator_type::IndicatorType, sma::SMA},
    models::{
        setups::setup::Setup, strategy_orientation::StrategyOrientation, timeseries::TimeSeries,
        traits::trading_strategy::TradingStrategy,
    },
    resolution_strategies::{
        atr_resolution::AtrResolution, CalculatesStopLosses, CalculatesTakeProfits,
        ResolutionStrategy,
    },
};
use std::fmt::{Display, Formatter};

/// # Silver Cross Strategy
///
/// Strategy built on the silver cross event where the 21 SMA crosses the 55
/// SMA in either orientation.
#[derive(Debug, Clone)]
pub struct SilverCross {
    pub short_len: usize,
    pub long_len: usize,
    pub orientation: StrategyOrientation,
}

impl SilverCross {
    #[allow(dead_code)] // TODO: Remove once used
    pub fn new(orientation: StrategyOrientation, short_len: usize, long_len: usize) -> Self {
        SilverCross {
            orientation,
            short_len,
            long_len,
        }
    }

    #[allow(dead_code)] // TODO: Remove once used
    pub fn new_default() -> Self {
        SilverCross {
            orientation: StrategyOrientation::Long,
            short_len: 21,
            long_len: 55,
        }
    }

    fn get_orientation(
        &self,
        prev_short: &SMA,
        prev_long: &SMA,
        current_short: &SMA,
        current_long: &SMA,
    ) -> Option<StrategyOrientation> {
        let long_condition = prev_short < prev_long && current_short >= current_long;
        let short_condition = prev_short > prev_long && current_short <= current_long;

        if long_condition {
            Some(StrategyOrientation::Long)
        } else if short_condition {
            Some(StrategyOrientation::Short)
        } else {
            None
        }
    }
}

impl TradingStrategy for SilverCross {
    fn find_setups(&self, ts: &TimeSeries) -> Result<Vec<Setup>> {
        let mut setups: Vec<Setup> = Vec::new();
        let key_short = IndicatorType::SMA(self.short_len);
        let key_long = IndicatorType::SMA(self.long_len);

        for (i, candle) in ts.candles.iter().enumerate().skip(1) {
            let prev_candle = &ts.candles[i - 1];
            let prev_short = prev_candle.get_indicator(&key_short)?.as_sma();
            let prev_long = prev_candle.get_indicator(&key_long)?.as_sma();
            let current_short = candle.get_indicator(&key_short)?.as_sma();
            let current_long = candle.get_indicator(&key_long)?.as_sma();

            if let (Some(prev_short), Some(prev_long), Some(current_short), Some(current_long)) =
                (prev_short, prev_long, current_short, current_long)
            {
                let orientation =
                    self.get_orientation(&prev_short, &prev_long, &current_short, &current_long);

                if let Some(orientation) = orientation {
                    let resolution_strategy =
                        ResolutionStrategy::ATR(AtrResolution::new(14, 1.0, 1.5));

                    let len = 14;
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
                        stop_loss_resolution: resolution_strategy.clone(),
                        take_profit_resolution: resolution_strategy,
                        stop_loss,
                        take_profit,
                    })
                }
            }
        }

        Ok(setups)
    }

    fn max_length(&self) -> usize {
        self.long_len
    }

    fn find_reverse_setups(&self, _ts: &TimeSeries) -> Result<Vec<Setup>> {
        todo!()
    }

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
