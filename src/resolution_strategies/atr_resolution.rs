use super::{CalculatesStopLosses, CalculatesTakeProfits};
use crate::{
    indicators::{atr::ATR, indicator_type::IndicatorType, is_indicator::IsIndicator},
    models::{
        calculation_mode::CalculationMode, candle::Candle,
        strategy_orientation::StrategyOrientation,
    },
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtrResolution {
    pub len: usize,
    pub stop_loss_multiple: f64,
    pub take_profit_multiple: f64,
}

impl CalculatesStopLosses for AtrResolution {
    fn calculate_stop_loss(
        &self,
        candles: &Vec<Candle>,
        i: usize,
        orientation: &StrategyOrientation,
        len: usize,
    ) -> Result<f64> {
        let price = candles[i].price_by_mode(&CalculationMode::Close);
        let atr = self.get_atr(candles, i, len);

        if let Some(atr) = atr {
            Ok(AtrResolution::get_stop_loss(
                self,
                price,
                atr.value,
                &orientation,
            ))
        } else {
            Err(anyhow!("Unable to calculate stop-loss."))
        }
    }
}

impl CalculatesTakeProfits for AtrResolution {
    fn calculate_take_profit(
        &self,
        candles: &Vec<Candle>,
        i: usize,
        orientation: &StrategyOrientation,
        len: usize,
    ) -> Result<f64> {
        let price = candles[i].price_by_mode(&CalculationMode::Close);
        let atr = self.get_atr(candles, i, len);

        if let Some(atr) = atr {
            Ok(AtrResolution::get_take_profit(
                self,
                price,
                atr.value,
                &orientation,
            ))
        } else {
            Err(anyhow!("Unable to calculate take-profit"))
        }
    }
}

impl AtrResolution {
    fn get_atr(&self, candles: &Vec<Candle>, i: usize, len: usize) -> Option<ATR> {
        // Check if atr indicator is available on candle, if so, use it
        let indicator_type = IndicatorType::ATR(len);
        let indicator = candles[i].indicators.get(&indicator_type);

        if let Some(atr) = indicator.and_then(|i| i.as_atr()) {
            return Some(atr);
        }

        if i < len + 1 { return None }

        // If atr indicator is not available on candle, calculate it from previous candles
        ATR::calculate(&candles[i-len-1..i+1])
    }

    pub fn new(len: usize, stop_loss_multiple: f64, take_profit_multiple: f64) -> Self {
        AtrResolution {
            len,
            stop_loss_multiple,
            take_profit_multiple,
        }
    }

    pub fn get_stop_loss(&self, price: f64, atr: f64, orientation: &StrategyOrientation) -> f64 {
        match orientation {
            StrategyOrientation::Long => price - self.stop_loss_multiple * atr,
            StrategyOrientation::Short => price + self.stop_loss_multiple * atr,
        }
    }

    pub fn get_take_profit(&self, price: f64, atr: f64, orientation: &StrategyOrientation) -> f64 {
        match orientation {
            StrategyOrientation::Long => price + self.take_profit_multiple * atr,
            StrategyOrientation::Short => price - self.take_profit_multiple * atr,
        }
    }
}
