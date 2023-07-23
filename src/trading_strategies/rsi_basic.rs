use super::{
    setup::{FindsSetups, Setup, FindsReverseSetups},
    strategy_orientation::StrategyOrientation,
};
use crate::{
    indicators::{rsi::RSI, Indicator, IndicatorType},
    models::{candle::Candle, generic_result::GenericResult, timeseries::TimeSeries},
    resolution_strategies::{
        atr_resolution::AtrResolution, CalculatesTradeBounds, ResolutionStrategy,
    },
};
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
    pub length: usize,
    pub upper_band: f64,
    pub lower_band: f64,
    pub orientation: StrategyOrientation,
}

impl Display for RsiBasic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RSI Basic")
    }
}

impl RsiBasic {
    #[allow(dead_code)] // TODO: Remove once used
    pub fn new(
        length: usize,
        upper_band: f64,
        lower_band: f64,
        orientation: StrategyOrientation,
    ) -> Self {
        RsiBasic {
            length,
            upper_band,
            lower_band,
            orientation,
        }
    }

    pub fn new_default() -> Self {
        RsiBasic {
            length: 14,
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
        if let Some(orientation) = self.get_orientation(prev,current) {
            match orientation {
                StrategyOrientation::Long => Some(StrategyOrientation::Short),
                StrategyOrientation::Short => Some(StrategyOrientation::Long)
            }
        } else {
            None
        }
    }

    fn find_setups_by_direction(&self, ts: &TimeSeries,  reversed: bool) -> GenericResult<Vec<Setup>> {
        let length = 14;
        let key = IndicatorType::RSI(length);
        let mut setups: Vec<Setup> = Vec::new();

        for (i, candle) in ts.candles.iter().enumerate().skip(1) {
            let prev_candle = &ts.candles[i - 1];

            let prev_rsi = get_indicator(&prev_candle, &key, length)?;
            let current_rsi = get_indicator(candle, &key, length)?;

            if let (Some(prev), Some(current)) = (prev_rsi, current_rsi) {
                let orientation = if reversed {
                    self.get_reverse_orientation(&prev, &current)
                } else  {
                    self.get_orientation(&prev, &current)
                };
                
                if let Some(orientation) = orientation {
                    let atr = AtrResolution::new(14, 1.0, 1.5);
                    let resolution_strategy = ResolutionStrategy::ATR(atr);
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
                    });
                }
            }
        }

        Ok(setups)

    }
}

impl FindsSetups for RsiBasic {
    fn find_setups(&self, ts: &TimeSeries) -> GenericResult<Vec<Setup>> {
        self.find_setups_by_direction(ts, false)
    }
}

impl FindsReverseSetups for RsiBasic {
    fn find_reverse_setups(&self, ts: &TimeSeries) -> GenericResult<Vec<Setup>> {
        self.find_setups_by_direction(ts, true)
    }
}

fn get_indicator(
    candle: &Candle,
    key: &IndicatorType,
    length: usize,
) -> GenericResult<Option<RSI>> {
    candle
        .indicators
        .get(key)
        .ok_or_else(|| format!("No RSI of length {}", length).into())
        .and_then(|indicator| match indicator {
            Indicator::RSI(rsi) => Ok(rsi.clone()),
            _ => Ok(None),
        })
}
