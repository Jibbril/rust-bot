use crate::{
    indicators::{atr::ATR, IndicatorType},
    models::{
        calculation_mode::{price_by_calc_mode, CalculationMode},
        candle::Candle,
        generic_result::GenericResult,
        strategy_orientation::StrategyOrientation,
    },
};

use super::CalculatesTradeBounds;

#[derive(Debug, Clone)]
pub struct AtrResolution {
    #[allow(dead_code)] // TODO: Remove once used
    length: usize,
    stop_loss_multiple: f64,
    take_profit_multiple: f64,
}

impl CalculatesTradeBounds for AtrResolution {
    fn get_trade_bounds(
        &self,
        candles: &Vec<Candle>,
        i: usize,
        orientation: &StrategyOrientation,
    ) -> GenericResult<(f64, f64)> {
        // Check if atr indicator is available on candle, if so, use it
        let length = 14;
        let price = price_by_calc_mode(&candles[i], &CalculationMode::Close);
        let indicator_type = IndicatorType::ATR(length);
        let indicator = candles[i].indicators.get(&indicator_type);

        if let Some(atr) = indicator.and_then(|i| i.get_scalar_value()) {
            return AtrResolution::get_bounds(&self, price, atr, &orientation);
        }
        // If atr indicator not available on candle, calculate it from previous candles
        let atr = ATR::calculate(length, i, candles);
        if let Some(atr) = atr {
            AtrResolution::get_bounds(self, price, atr.value, &orientation)
        } else {
            Err("Unable to calculate trade bounds.".into())
        }
    }
}

impl AtrResolution {
    pub fn new(length: usize, stop_loss_multiple: f64, take_profit_multiple: f64) -> Self {
        AtrResolution {
            length,
            stop_loss_multiple,
            take_profit_multiple,
        }
    }

    pub fn get_bounds(
        &self,
        price: f64,
        atr: f64,
        orientation: &StrategyOrientation,
    ) -> GenericResult<(f64, f64)> {
        match orientation {
            StrategyOrientation::Long => {
                let stop_loss = price - self.stop_loss_multiple * atr;
                let take_profit = price + self.take_profit_multiple * atr;
                return Ok((take_profit, stop_loss));
            }
            StrategyOrientation::Short => {
                let stop_loss = price + self.stop_loss_multiple * atr;
                let take_profit = price - self.take_profit_multiple * atr;
                return Ok((take_profit, stop_loss));
            }
        }
    }
}
