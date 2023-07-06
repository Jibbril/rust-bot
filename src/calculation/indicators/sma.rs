use crate::utils::timeseries::Candle;
use super::CalculationMode;

pub fn rolling_sma(
    length: usize,
    i: usize,
    candles: &Vec<Candle>
    , previous_sma: f32
) -> Option<f32> {
    calculation_mode_rolling_sma(
        length, 
        i, 
        candles, 
        CalculationMode::Close, 
        previous_sma
    )
}

pub fn calculation_mode_rolling_sma(
    length: usize,
    i: usize, 
    candles: &Vec<Candle>, 
    mode: CalculationMode,
    previous_sma: f32
) -> Option<f32> {
    let arr_length = candles.len();
    if i > arr_length || length > arr_length {
        None
    } else {
        let price_out = match mode {
            CalculationMode::Open => candles[i - length].open,
            CalculationMode::High => candles[i - length].high,
            CalculationMode::Low => candles[i - length].low,
            CalculationMode::Close => candles[i - length].close,
        };
        let price_in = match mode {
            CalculationMode::Open => candles[i].open,
            CalculationMode::High => candles[i].high,
            CalculationMode::Low => candles[i].low,
            CalculationMode::Close => candles[i].close,
        };

        let sma = ((previous_sma * length as f32) - price_out + price_in) / length as f32;

        Some(sma)
    }
}

pub fn calculation_mode_sma(
    length: usize,
    i: usize, 
    candles: &Vec<Candle>, 
    mode: CalculationMode
) -> Option<f32> {
    let arr_length = candles.len();
    if i > arr_length || length > arr_length {
        None
    } else {
        let start = i + 1 - length;
        let end = i + 1;
        let segment = &candles[start..end];
        let length = length as f32;

        let sma = match mode {
            CalculationMode::Open => segment.iter().map(|c| c.open).sum::<f32>() / length,
            CalculationMode::High => segment.iter().map(|c| c.high).sum::<f32>() / length,
            CalculationMode::Low => segment.iter().map(|c| c.low).sum::<f32>() / length,
            CalculationMode::Close => segment.iter().map(|c| c.close).sum::<f32>() / length,
        };

        Some(sma)
    }
}

// Default implementation using closing values for calculations.
pub fn sma(length:usize, i:usize, candles: &Vec<Candle>) -> Option<f32> {
    calculation_mode_sma(length, i, candles, CalculationMode::Close)
}