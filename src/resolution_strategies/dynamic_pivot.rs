use crate::{models::{
    candle::Candle, generic_result::GenericResult, strategy_orientation::StrategyOrientation, 
}, indicators::IndicatorType};
use serde::{Deserialize, Serialize};
use super::CalculatesStopLosses;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicPivotResolution {
    pub length: usize,
}

impl CalculatesStopLosses for DynamicPivotResolution {
    fn calculate_stop_loss(
        &self,
        candles: &Vec<Candle>,
        i: usize,
        orientation: &StrategyOrientation,
        length: usize
    ) -> GenericResult<f64> {
        let mut j = i;

        if i == 0 {
            return Err("Unable to calculate stop_loss for first candle in series".into())
        }

        loop {
            let indicator = candles[j].get_indicator(&IndicatorType::DynamicPivot(length))?;

            if let Some(pivot) = indicator.as_dynamic_pivots() {
                return match orientation {
                    StrategyOrientation::Long => Ok(pivot.low),
                    StrategyOrientation::Short => Ok(pivot.high)
                }
            } else {
                j -= 1;
            }

            if j == 0 {
                break;
            }
        }
        
        Err("Unable to find DynamicPivot indicator in TimeSeries.".into())
    }
}

impl DynamicPivotResolution {
    pub fn new(length: usize) -> Self {
        Self { length }
    }
}