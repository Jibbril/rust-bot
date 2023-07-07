use crate::utils::timeseries::Candle;
use super::CalculationMode;

pub struct RSI {
    length: usize,
    value: f64,
}

impl RSI {
    // Default implementation using closing values for calculations.
    pub fn calculate_rolling() -> Option<RSI> {
        Self::calc_mode_rolling()
    }

    fn calc_mode_rolling() -> Option<RSI> {
        Some(RSI {
            length: 14,
            value: 83.7
        })
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
            let start = i - length;
            let end = i + 1;
            let segment = &candles[start..end];

            let mut gains = 0.0;
            let mut losses = 0.0;

            for (j,candle) in segment.iter().enumerate().skip(1) {
                let close = match mode {
                    CalculationMode::Close => candle.close,
                    CalculationMode::Open => candle.open,
                    CalculationMode::High => candle.high,
                    CalculationMode::Low => candle.low,
                };
                let prev_close = match mode {
                    CalculationMode::Close => segment[j-1].close,
                    CalculationMode::Open => segment[j-1].open,
                    CalculationMode::High => segment[j-1].high,
                    CalculationMode::Low => segment[j-1].low,
                };

                let change = close/prev_close - 1.0;
                if change >= 0.0 {
                    gains += change;
                } else {
                    losses += -change;
                }
            }

            let rs = if losses != 0.0 { gains / losses } else { 0.0 };

            Some(RSI {
                length,
                value: 100.0 - 100.0 / ( 1.0 + rs)
            })
        }
    }
}