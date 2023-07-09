use crate::utils::{generic_result::GenericResult, timeseries::Candle};

use super::{
    calculation_mode::{price_by_calc_mode, CalculationMode},
    Indicator, IndicatorType, PopulatesCandles,
};

#[derive(Debug, Copy, Clone)]
pub struct RSI {
    #[allow(dead_code)] // TODO: Remove once used
    length: usize,
    #[allow(dead_code)] // TODO: Remove once used
    value: f64,
    avg_gain: f64,
    avg_loss: f64,
}

impl PopulatesCandles for RSI {
    fn populate_candles(candles: &mut Vec<Candle>, length: usize) -> GenericResult<()> {
        if candles.len() < length {
            return Err("Length of candles is shorter than indicator length.".into());
        }

        let mut rsi: Option<RSI> = None;
        let new_rsis: Vec<Option<RSI>> = (0..candles.len())
            .map(|i| {
                rsi = Self::calculate_rolling(i, &candles, &rsi);
                rsi
            })
            .collect();

        let indicator_type = IndicatorType::RSI(length);

        for (i, candle) in candles.iter_mut().enumerate() {
            let new_rsi = Indicator::RSI(new_rsis[i]);

            candle.indicators.insert(indicator_type, new_rsi);
        }

        Ok(())
    }
}

impl RSI {
    // Default implementation using closing values and for calculations.
    pub fn calculate_rolling(
        i: usize,
        candles: &Vec<Candle>,
        prev_rsi: &Option<RSI>,
    ) -> Option<RSI> {
        Self::calc_mode_rolling(14, i, candles, CalculationMode::Close, prev_rsi)
    }

    fn calc_mode_rolling(
        length: usize,
        i: usize,
        candles: &Vec<Candle>,
        mode: CalculationMode,
        prev_rsi: &Option<RSI>,
    ) -> Option<RSI> {
        if i < length - 1 || i >= candles.len() || candles.len() < length {
            None
        } else if let Some(prev_rsi) = prev_rsi {
            let current = price_by_calc_mode(&candles[i], &mode);
            let previous = price_by_calc_mode(&candles[i - 1], &mode);
            let change = current / previous - 1.0;

            let f_length = length as f64;
            let mut gains = prev_rsi.avg_gain * (f_length - 1.0);
            let mut losses = prev_rsi.avg_loss * (f_length - 1.0);

            if change >= 0.0 {
                gains += change;
            } else {
                losses += -change;
            }

            let rs = if losses != 0.0 { gains / losses } else { 0.0 };

            let rsi = 100.0 - (100.0 / (1.0 + rs));

            Some(RSI {
                length,
                value: rsi,
                avg_gain: gains / f_length,
                avg_loss: losses / f_length,
            })
        } else {
            Self::calculate(i, candles)
        }
    }

    // Default implementation using closing values for calculations.
    pub fn calculate(i: usize, candles: &Vec<Candle>) -> Option<RSI> {
        Self::calculation_mode_rsi(14, i, candles, CalculationMode::Close)
    }

    fn calculation_mode_rsi(
        length: usize,
        i: usize,
        candles: &Vec<Candle>,
        mode: CalculationMode,
    ) -> Option<RSI> {
        if i < length - 1 || length > candles.len() {
            None
        } else {
            let f_length = length as f64;
            let start = i + 1 - length;
            let end = i + 1;
            let segment = &candles[start..end];

            let (gains, losses) = Self::get_outcomes(segment, mode);

            let rs = if losses != 0.0 { gains / losses } else { 0.0 };

            Some(RSI {
                length,
                value: 100.0 - 100.0 / (1.0 + rs),
                avg_gain: gains / f_length,
                avg_loss: losses / f_length,
            })
        }
    }

    fn get_outcomes(segment: &[Candle], mode: CalculationMode) -> (f64, f64) {
        let mut gains = 0.0;
        let mut losses = 0.0;

        for i in 1..segment.len() {
            let current = price_by_calc_mode(&segment[i], &mode);
            let previous = price_by_calc_mode(&segment[i - 1], &mode);

            let change = current / previous - 1.0;
            if change >= 0.0 {
                gains += change;
            } else {
                losses += -change;
            }
        }

        (gains, losses)
    }
}
