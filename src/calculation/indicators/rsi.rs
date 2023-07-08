use crate::utils::timeseries::Candle;

use super::calculation_mode::{CalculationMode, price_by_calc_mode};


#[derive(Debug,Clone)]
pub struct RSI {
    length: usize,
    value: f64,
    avg_gain: f64,
    avg_loss: f64,
}

impl RSI {
    // Default implementation using closing values and for calculations.
    pub fn calculate_rolling(
        i: usize,
        candles: &Vec<Candle>,
        prev_rsi: &RSI
    ) -> Option<RSI> {
        Self::calc_mode_rolling(
            14,
            i,
            candles,
            CalculationMode::Close,
            prev_rsi
        )
    }

    fn calc_mode_rolling(
        length: usize,
        i: usize,
        candles: &Vec<Candle>,
        mode: CalculationMode,
        prev_rsi: &RSI
    ) -> Option<RSI> {
        if i < length || i >= candles.len() || candles.len() < length {
            None
        } else {
            let current = price_by_calc_mode(&candles[i], &mode);
            let previous = price_by_calc_mode(&candles[i-1], &mode);
            let change = current/previous - 1.0;
            
            let f_length = length as f64;
            let mut gains = prev_rsi.avg_gain * (f_length - 1.0);
            let mut losses = prev_rsi.avg_loss * (f_length - 1.0);

            if change  >= 0.0 {
                gains += change;
            } else {
                losses += -change;
            }

            let rs = if losses != 0.0 { gains/losses } else { 0.0 };
    
            let rsi = 100.0 - ( 100.0 / (1.0 + rs) );

            Some(RSI {
                length,
                value: rsi,
                avg_gain: gains/f_length,
                avg_loss: losses/f_length,
            })
        }
    }

    // Default implementation using closing values for calculations.
    pub fn calculate(i: usize, candles: &Vec<Candle>) -> Option<RSI> {
        Self::calculation_mode_rsi(
            14,
            i,
            candles,
            CalculationMode::Close
        )
    }

    fn calculation_mode_rsi(
        length: usize,
        i: usize,
        candles: &Vec<Candle>,
        mode: CalculationMode
    ) -> Option<RSI> {
        if i < length - 1 || length > candles.len() {
            None
        } else {
            let f_length = length as f64;
            let start = i - length;
            let end = i + 1;
            let segment = &candles[start..end];

            let (gains,losses) = Self::get_outcomes(segment,mode);

            let rs = if losses != 0.0 { gains / losses } else { 0.0 };

            Some(RSI {
                length,
                value: 100.0 - 100.0 / ( 1.0 + rs),
                avg_gain: gains/f_length,
                avg_loss: losses/f_length,
            })
        }
    }

    fn get_outcomes(segment: &[Candle], mode: CalculationMode) -> (f64,f64) {
        let mut gains = 0.0;
        let mut losses = 0.0;

        for (j,candle) in segment.iter().enumerate().skip(1) {
            let current = match mode {
                CalculationMode::Close => candle.close,
                CalculationMode::Open => candle.open,
                CalculationMode::High => candle.high,
                CalculationMode::Low => candle.low,
            };
            let previous = match mode {
                CalculationMode::Close => segment[j-1].close,
                CalculationMode::Open => segment[j-1].open,
                CalculationMode::High => segment[j-1].high,
                CalculationMode::Low => segment[j-1].low,
            };

            let change = current/previous - 1.0;
            if change >= 0.0 {
                gains += change;
            } else {
                losses += -change;
            }
        }

        (gains,losses)
    }
}