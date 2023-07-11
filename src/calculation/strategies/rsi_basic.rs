use crate::{utils::{timeseries::{TimeSeries, Candle}, generic_result::GenericResult}, calculation::indicators::{IndicatorType, Indicator, rsi::RSI}};

use super::{StrategyOrientation, FindsSetups, Setup};


#[derive(Debug, Clone)]
pub struct RsiBasic {
    pub length: usize,
    pub upper_band: f64,
    pub lower_band: f64,
    pub orientation: StrategyOrientation
}

impl RsiBasic {
    #[allow(dead_code)] // TODO: Remove once used
    pub fn new(length: usize, upper_band: f64, lower_band: f64, orientation: StrategyOrientation) -> Self {
        RsiBasic { length, upper_band, lower_band, orientation }
    }

    pub fn new_default() -> Self {
        RsiBasic {
            length: 14,
            upper_band: 70.0,
            lower_band: 30.0,
            orientation: StrategyOrientation::Both
        }
    }
}

impl FindsSetups for RsiBasic {
    fn find_setups(&self, ts: &mut TimeSeries) -> GenericResult<Vec<Setup>> {
        let length = 14;
        let key = IndicatorType::RSI(length);
        let mut setups: Vec<Setup> = Vec::new();

        for (i,candle) in ts.candles.iter().enumerate().skip(1) {
            let prev_candle = &ts.candles[i-1];

            let prev_rsi = get_indicator(&prev_candle, &key, length)?; 

            if let Some(prev) = prev_rsi {
                let current = get_indicator(candle, &key, length)?;
                let current = match current {
                    Some(rsi) => rsi,
                    _ => return Err("Unable to retrieve current RSI.".into())
                };

                if prev.value < self.lower_band && current.value > self.lower_band {
                    // Go long
                    setups.push(Setup {
                        candle: candle.clone(),
                        interval: ts.interval.clone(),
                        orientation: StrategyOrientation::Long
                    })
                } else if prev.value > self.upper_band && current.value < self.upper_band {
                    setups.push(Setup {
                        candle: candle.clone(),
                        interval: ts.interval.clone(),
                        orientation: StrategyOrientation::Short
                    })
                } 
            }
        }

        Ok(setups)
    }
}

fn get_indicator(candle: &Candle, key: &IndicatorType, length: usize) -> GenericResult<Option<RSI>> {
    let err_message = format!("No RSI of length {}", length);
    let prev_rsi = candle.indicators.get(key);
    let indicator = match prev_rsi {
        None => return Err(err_message.into()),
        Some(indicator) => match indicator {
            Indicator::RSI(rsi) => rsi,
            _ => &None
        }
    };

    Ok(indicator.clone())
}