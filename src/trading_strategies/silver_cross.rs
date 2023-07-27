use crate::{
    indicators::{sma::SMA, IndicatorType},
    models::{
        generic_result::GenericResult,
        setup::{FindsSetups, Setup},
        strategy_orientation::StrategyOrientation,
        timeseries::TimeSeries,
    },
    resolution_strategies::{
        atr_resolution::AtrResolution, CalculatesTradeBounds, ResolutionStrategy,
    },
};
use std::fmt::{Display, Formatter};

/// # Silver Cross Strategy
///
/// Strategy built on the silver cross event where the 21 SMA crosses the 55
/// SMA in either orientation.
#[derive(Debug, Clone)]
pub struct SilverCross {
    pub orientation: StrategyOrientation,
}

impl SilverCross {
    pub fn new(orientation: StrategyOrientation) -> Self {
        SilverCross { orientation }
    }

    pub fn new_default() -> Self {
        SilverCross {
            orientation: StrategyOrientation::Long,
        }
    }

    fn get_orientation(
        &self,
        prev_21: &SMA,
        prev_55: &SMA,
        current_21: &SMA,
        current_55: &SMA,
    ) -> Option<StrategyOrientation> {
        let long_condition = prev_21 < prev_55 && current_21 >= current_55;
        let short_condition = prev_21 > prev_55 && current_21 <= current_55;

        if long_condition {
            Some(StrategyOrientation::Long)
        } else if short_condition {
            Some(StrategyOrientation::Short)
        } else {
            None
        }
    }
}

impl FindsSetups for SilverCross {
    fn find_setups(&self, ts: &TimeSeries) -> GenericResult<Vec<Setup>> {
        let mut setups: Vec<Setup> = Vec::new();
        let key_21 = IndicatorType::SMA(21);
        let key_55 = IndicatorType::SMA(55);

        for (i, candle) in ts.candles.iter().enumerate().skip(1) {
            let prev_candle = &ts.candles[i - 1];
            let prev_21 = prev_candle.get_indicator(&key_21)?.as_sma();
            let prev_55 = prev_candle.get_indicator(&key_55)?.as_sma();
            let current_21 = candle.get_indicator(&key_21)?.as_sma();
            let current_55 = candle.get_indicator(&key_55)?.as_sma();

            if let (Some(prev_21), Some(prev_55), Some(current_21), Some(current_55)) =
                (prev_21, prev_55, current_21, current_55)
            {
                let orientation =
                    self.get_orientation(&prev_21, &prev_55, &current_21, &current_55);

                if let Some(orientation) = orientation {
                    let resolution_strategy =
                        ResolutionStrategy::ATR(AtrResolution::new(14, 1.0, 1.5));

                    let (take_profit, stop_loss) =
                        resolution_strategy.get_trade_bounds(&ts.candles, i, &orientation)?;

                    setups.push(Setup {
                        ticker: ts.ticker.clone(),
                        candle: candle.clone(),
                        interval: ts.interval.clone(),
                        orientation,
                        resolution_strategy,
                        stop_loss,
                        take_profit,
                    })
                }
            }
        }

        Ok(setups)
    }
}

impl Display for SilverCross {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Silver Cross")
    }
}
