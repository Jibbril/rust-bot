use super::CalculatesStopLosses;
use crate::{
    indicators::indicator_type::IndicatorType,
    models::{candle::Candle, strategy_orientation::StrategyOrientation},
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicPivotResolution {
    pub len: usize,
}

impl CalculatesStopLosses for DynamicPivotResolution {
    fn calculate_stop_loss(
        &self,
        candles: &Vec<Candle>,
        i: usize,
        orientation: &StrategyOrientation,
        len: usize,
    ) -> Result<f64> {
        let mut j = i;

        if i == 0 {
            return Err(anyhow!(
                "Unable to calculate stop_loss for first candle in series"
            ));
        }

        loop {
            let indicator = candles[j].get_indicator(&IndicatorType::DynamicPivot(len))?;

            if let Some(pivot) = indicator.as_dynamic_pivots() {
                return match orientation {
                    StrategyOrientation::Long => {
                        Ok(pivot.low.expect("Unable to find previous low"))
                    }
                    StrategyOrientation::Short => {
                        Ok(pivot.high.expect("Unable to find previous high"))
                    }
                };
            } else {
                j -= 1;
            }

            if j == 0 {
                break;
            }
        }

        Err(anyhow!(
            "Unable to find DynamicPivot indicator in TimeSeries."
        ))
    }
}

impl DynamicPivotResolution {
    #[allow(dead_code)]
    pub fn new(len: usize) -> Self {
        Self { len }
    }
}
