use crate::{
    calculation::indicators::{rsi::RSI, Indicator, IndicatorType},
    utils::{
        generic_result::GenericResult,
        timeseries::{Candle, TimeSeries},
    },
};

use super::{
    resolution_strategy::{self, AtrResolution, CalculatesTradeBounds, ResolutionStrategy},
    setup::{FindsSetups, Setup},
    strategy::StrategyOrientation,
};

#[derive(Debug, Clone)]
pub struct RsiBasic {
    pub length: usize,
    pub upper_band: f64,
    pub lower_band: f64,
    pub orientation: StrategyOrientation,
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
            orientation: StrategyOrientation::Both,
        }
    }
}

fn get_orientation(strategy: &RsiBasic, prev: &RSI, current: &RSI) -> Option<StrategyOrientation> {
    let long_condition = prev.value < strategy.lower_band && current.value > strategy.lower_band;
    let short_condition = prev.value > strategy.upper_band && current.value < strategy.upper_band;

    if long_condition {
        Some(StrategyOrientation::Long)
    } else if short_condition {
        Some(StrategyOrientation::Short)
    } else {
        None
    }
}

impl FindsSetups for RsiBasic {
    fn find_setups(&self, ts: &mut TimeSeries) -> GenericResult<Vec<Setup>> {
        let length = 14;
        let key = IndicatorType::RSI(length);
        let mut setups: Vec<Setup> = Vec::new();

        for (i, candle) in ts.candles.iter().enumerate().skip(1) {
            let prev_candle = &ts.candles[i - 1];

            let prev_rsi = get_indicator(&prev_candle, &key, length)?;

            if let Some(prev) = prev_rsi {
                let current = get_indicator(candle, &key, length)?;
                let current = match current {
                    Some(rsi) => rsi,
                    _ => return Err("Unable to retrieve current RSI.".into()),
                };

                if let Some(orientation) = get_orientation(&self, &prev, &current) {
                    let atr = AtrResolution::new(14, 1.0, 2.0);
                    let resolution_strategy = ResolutionStrategy::ATR(atr);
                    let (take_profit, stop_loss) =
                        resolution_strategy.get_trade_bounds(&ts.candles, i, &orientation)?;

                    setups.push(Setup {
                        candle: candle.clone(),
                        interval: ts.interval.clone(),
                        orientation,
                        resolution_strategy,
                        stop_loss,
                        take_profit,
                    });
                } else {
                    continue
                }
            }
        }

        Ok(setups)
    }
}

fn get_indicator(
    candle: &Candle,
    key: &IndicatorType,
    length: usize,
) -> GenericResult<Option<RSI>> {
    let err_message = format!("No RSI of length {}", length);
    let prev_rsi = candle.indicators.get(key);
    let indicator = match prev_rsi {
        None => return Err(err_message.into()),
        Some(indicator) => match indicator {
            Indicator::RSI(rsi) => rsi,
            _ => &None,
        },
    };

    Ok(indicator.clone())
}
