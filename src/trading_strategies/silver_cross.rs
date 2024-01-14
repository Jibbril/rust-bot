use crate::{
    indicators::{indicator_type::IndicatorType, sma::SMA},
    models::{
        candle::Candle,
        setups::{setup::Setup, setup_builder::SetupBuilder},
        strategy_orientation::StrategyOrientation,
        timeseries::TimeSeries,
        traits::trading_strategy::TradingStrategy,
    }, resolution_strategies::{fixed_values::FixedValuesResolution, resolution_strategy::ResolutionStrategy},
};
use anyhow::Result;
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
    pub fn new_args(orientation: StrategyOrientation, short_len: usize, long_len: usize) -> Self {
        SilverCross {
            orientation,
            short_len,
            long_len,
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
        let _short_condition = prev_short > prev_long && current_short <= current_long;

        if long_condition {
            Some(StrategyOrientation::Long)
        // } else if short_condition {
        //     Some(StrategyOrientation::Short)
        } else {
            None
        }
    }
}

impl TradingStrategy for SilverCross {
    #[allow(dead_code)] // TODO: Remove once used
    fn new() -> Self {
        SilverCross {
            orientation: StrategyOrientation::Long,
            short_len: 21,
            long_len: 55,
        }
    }

    fn find_setups(&self, ts: &TimeSeries) -> Result<Vec<Setup>> {
        let mut setups: Vec<Setup> = Vec::new();
        let key_short = IndicatorType::SMA(self.short_len);
        let key_long = IndicatorType::SMA(self.long_len);

        for (i, candle) in ts.candles.iter().enumerate().skip(1) {
            let prev_candle = &ts.candles[i - 1];
            let prev_short = prev_candle.clone_indicator(&key_short)?.as_sma();
            let prev_long = prev_candle.clone_indicator(&key_long)?.as_sma();
            let current_short = candle.clone_indicator(&key_short)?.as_sma();
            let current_long = candle.clone_indicator(&key_long)?.as_sma();

            if let (Some(prev_short), Some(prev_long), Some(current_short), Some(current_long)) =
                (prev_short, prev_long, current_short, current_long)
            {
                let orientation =
                    self.get_orientation(&prev_short, &prev_long, &current_short, &current_long);

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
                    })
                }
            }
        }

        Ok(setups)
    }

    fn min_length(&self) -> usize {
        self.long_len
    }

    fn required_indicators(&self) -> Vec<IndicatorType> {
        vec![
            IndicatorType::SMA(self.short_len),
            IndicatorType::SMA(self.long_len),
        ]
    }

    fn candles_needed_for_setup(&self) -> usize {
        // TODO: Add real value
        self.long_len
    }

    fn check_last_for_setup(&self, _candles: &[Candle]) -> Option<SetupBuilder> {
        todo!()
    }

    fn clone_box(&self) -> Box<dyn TradingStrategy> {
        Box::new(self.clone())
    }
}

impl Display for SilverCross {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Silver Cross")
    }
}
