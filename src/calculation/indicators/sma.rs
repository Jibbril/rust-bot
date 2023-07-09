use super::{
    calculation_mode::{price_by_calc_mode, CalculationMode},
    Indicator, IndicatorType, PopulatesCandles,
};
use crate::utils::{generic_result::GenericResult, timeseries::Candle};

#[derive(Debug, Copy, Clone)]
pub struct SMA {
    length: usize,
    value: f64,
}

impl PopulatesCandles for SMA {
    fn populate_candles(candles: &mut Vec<Candle>, length: usize) -> GenericResult<()> {
        if candles.len() < length {
            return Err("Length of candles is shorter than indicator length.".into());
        }

        let mut sma: Option<SMA> = None;
        let new_smas: Vec<Option<SMA>> = (0..candles.len())
            .map(|i| {
                sma = Self::calculate_rolling(length, i, candles, &sma);
                sma
            })
            .collect();

        let indicator_type = IndicatorType::SMA(length);

        for (i, candle) in candles.iter_mut().enumerate() {
            let new_sma = Indicator::SMA(new_smas[i]);

            candle.indicators.insert(indicator_type, new_sma);
        }

        Ok(())
    }
}

impl SMA {
    // Default implementation using closing values for calculations.
    pub fn calculate_rolling(
        length: usize,
        i: usize,
        candles: &Vec<Candle>,
        previous_sma: &Option<SMA>,
    ) -> Option<SMA> {
        Self::calc_mode_rolling(length, i, candles, CalculationMode::Close, previous_sma)
    }

    fn calc_mode_rolling(
        length: usize,
        i: usize,
        candles: &Vec<Candle>,
        mode: CalculationMode,
        previous_sma: &Option<SMA>,
    ) -> Option<SMA> {
        let arr_length = candles.len();
        if i > arr_length || length > arr_length || i < length - 1 {
            None
        } else if let Some(prev_sma) = previous_sma {
            let price_out = price_by_calc_mode(&candles[i - length], &mode);
            let price_in = price_by_calc_mode(&candles[i], &mode);

            let sma = ((prev_sma.value * length as f64) - price_out + price_in) / length as f64;

            Some(SMA { length, value: sma })
        } else {
            Self::calculate(length, i, candles)
        }
    }

    // Default implementation using closing values for calculations.
    pub fn calculate(length: usize, i: usize, candles: &Vec<Candle>) -> Option<SMA> {
        Self::calculation_mode_sma(length, i, candles, CalculationMode::Close)
    }

    fn calculation_mode_sma(
        length: usize,
        i: usize,
        candles: &Vec<Candle>,
        mode: CalculationMode,
    ) -> Option<SMA> {
        let arr_length = candles.len();
        if i > arr_length || length > arr_length {
            None
        } else {
            let start = i + 1 - length;
            let end = i + 1;
            let segment = &candles[start..end];
            let f_length = length as f64;

            let sma = match mode {
                CalculationMode::Open => segment.iter().map(|c| c.open).sum::<f64>() / f_length,
                CalculationMode::High => segment.iter().map(|c| c.high).sum::<f64>() / f_length,
                CalculationMode::Low => segment.iter().map(|c| c.low).sum::<f64>() / f_length,
                CalculationMode::Close => segment.iter().map(|c| c.close).sum::<f64>() / f_length,
            };

            Some(SMA { length, value: sma })
        }
    }
}
