use crate::utils::timeseries::Candle;
use super::calculation_mode::{CalculationMode, price_by_calc_mode};

#[derive(Debug,Clone)]
pub struct SMA {
    length: usize,
    value: f64,
}

impl SMA {
    // Default implementation using closing values for calculations.
    pub fn calculate_rolling(
        length: usize,
        i: usize,
        candles: &Vec<Candle>
        , previous_sma: f64
    ) -> Option<SMA> {
        Self::calc_mode_rolling(
            length, 
            i, 
            candles, 
            CalculationMode::Close, 
            previous_sma
        )
    }

    fn calc_mode_rolling(
        length: usize,
        i: usize, 
        candles: &Vec<Candle>, 
        mode: CalculationMode,
        previous_sma: f64
    ) -> Option<SMA> {
        let arr_length = candles.len();
        if i > arr_length || length > arr_length {
            None
        } else {
            let price_out = price_by_calc_mode(&candles[i - length], &mode);
            let price_in = price_by_calc_mode(&candles[i], &mode);

            let sma = ((previous_sma * length as f64) - price_out + price_in) / length as f64;

            Some(SMA {
                length,
                value: sma,
            })
        }
    }

    // Default implementation using closing values for calculations.
    pub fn calculate(length:usize, i:usize, candles: &Vec<Candle>) -> Option<SMA> {
        Self::calculation_mode_sma(length, i, candles, CalculationMode::Close)
    }

    fn calculation_mode_sma(
        length: usize,
        i: usize, 
        candles: &Vec<Candle>, 
        mode: CalculationMode
    ) -> Option<SMA> {
        let arr_length = candles.len();
        if i > arr_length || length > arr_length {
            None
        } else {
            let start = i - length;
            let end = i + 1;
            let segment = &candles[start..end];
            let f_length = length as f64;

            let sma = match mode {
                CalculationMode::Open => segment.iter().map(|c| c.open).sum::<f64>() / f_length,
                CalculationMode::High => segment.iter().map(|c| c.high).sum::<f64>() / f_length,
                CalculationMode::Low => segment.iter().map(|c| c.low).sum::<f64>() / f_length,
                CalculationMode::Close => segment.iter().map(|c| c.close).sum::<f64>() / f_length,
            };

            Some(SMA {
                length,
                value: sma
            })
        }
    }
}