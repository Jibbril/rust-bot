use super::{CalculatesStopLosses, CalculatesTakeProfits};
use crate::{
    indicators::{atr::ATR, IndicatorType},
    models::{
        calculation_mode::CalculationMode, candle::Candle, generic_result::GenericResult,
        strategy_orientation::StrategyOrientation,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtrResolution {
    pub length: usize,
    pub stop_loss_multiple: f64,
    pub take_profit_multiple: f64,
}

impl CalculatesStopLosses for AtrResolution {
    fn calculate_stop_loss(
        &self,
        candles: &Vec<Candle>,
        i: usize,
        orientation: &StrategyOrientation,
        length: usize,
    ) -> GenericResult<f64> {
        let price = candles[i].price_by_mode(&CalculationMode::Close);
        let atr = self.get_atr(candles, i, length);

        if let Some(atr) = atr {
            Ok(AtrResolution::get_stop_loss(
                self,
                price,
                atr.value,
                &orientation,
            ))
        } else {
            Err("Unable to calculate stop-loss.".into())
        }
    }
}

impl CalculatesTakeProfits for AtrResolution {
    fn calculate_take_profit(
        &self,
        candles: &Vec<Candle>,
        i: usize,
        orientation: &StrategyOrientation,
        length: usize,
    ) -> GenericResult<f64> {
        let price = candles[i].price_by_mode(&CalculationMode::Close);
        let atr = self.get_atr(candles, i, length);

        if let Some(atr) = atr {
            Ok(AtrResolution::get_take_profit(
                self,
                price,
                atr.value,
                &orientation,
            ))
        } else {
            Err("Unable to calculate take-profit".into())
        }
    }
}

impl AtrResolution {
    fn get_atr(&self, candles: &Vec<Candle>, i: usize, length: usize) -> Option<ATR> {
        // Check if atr indicator is available on candle, if so, use it
        let indicator_type = IndicatorType::ATR(length);
        let indicator = candles[i].indicators.get(&indicator_type);

        if let Some(atr) = indicator.and_then(|i| i.as_atr()) {
            return Some(atr);
        }

        // If atr indicator not available on candle, calculate it from previous candles
        ATR::calculate(length, i, candles)
    }

    pub fn new(length: usize, stop_loss_multiple: f64, take_profit_multiple: f64) -> Self {
        AtrResolution {
            length,
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
