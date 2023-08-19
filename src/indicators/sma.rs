use crate::models::{
    calculation_mode::CalculationMode, candle::Candle, generic_result::GenericResult,
    timeseries::TimeSeries,
};

use super::{ indicator::Indicator, indicator_type::IndicatorType, populates_candles::PopulatesCandles};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct SMA {
    #[allow(dead_code)] // TODO: Remove once used
    pub value: f64,
    pub length: usize,
}

impl PopulatesCandles for SMA {
    fn populate_candles_default(ts: &mut TimeSeries) -> GenericResult<()> {
        Self::populate_candles(ts, 8)
    }
    fn populate_candles(ts: &mut TimeSeries, length: usize) -> GenericResult<()> {
        let mut sma: Option<SMA> = None;
        let new_smas: Vec<Option<SMA>> = (0..ts.candles.len())
            .map(|i| {
                sma = Self::calculate_rolling(length, i, &ts.candles, &sma);
                sma
            })
            .collect();

        let indicator_type = IndicatorType::SMA(length);

        for (i, candle) in ts.candles.iter_mut().enumerate() {
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
        Self::calculate_rolling_with_opts(length, i, candles, CalculationMode::Close, previous_sma)
    }

    fn calculate_rolling_with_opts(
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
            let price_out = candles[i - length].price_by_mode(&mode);
            let price_in = candles[i].price_by_mode(&mode);

            let sma = ((prev_sma.value * length as f64) - price_out + price_in) / length as f64;

            Some(SMA { length, value: sma })
        } else {
            Self::calculate(length, i, candles)
        }
    }

    // Default implementation using closing values for calculations.
    pub fn calculate(length: usize, i: usize, candles: &Vec<Candle>) -> Option<SMA> {
        Self::calculate_with_opts(length, i, candles, CalculationMode::Close)
    }

    fn calculate_with_opts(
        length: usize,
        i: usize,
        candles: &Vec<Candle>,
        mode: CalculationMode,
    ) -> Option<SMA> {
        let arr_length = candles.len();
        if i > arr_length || length > arr_length || i < length - 1 {
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

#[cfg(test)]
mod tests {
    use crate::models::candle::Candle;

    use super::SMA;

    #[test]
    fn calculate_sma() {
        let candles = Candle::dummy_data(4, "positive", 100.0);
        let sma = SMA::calculate(4, 3, &candles);
        assert!(sma.is_some());
        let sma = sma.unwrap();
        assert_eq!(sma.value, 125.0);
    }

    #[test]
    fn sma_not_enough_data() {
        let candles = Candle::dummy_data(2, "positive", 100.0);
        let sma = SMA::calculate(4, 3, &candles);
        assert!(sma.is_none());
    }

    #[test]
    fn sma_no_candles() {
        let candles: Vec<Candle> = Vec::new();
        let sma = SMA::calculate(4, 3, &candles);
        assert!(sma.is_none());
    }

    #[test]
    fn rolling_sma() {
        let n = 20;
        let length = 7;
        let candles = Candle::dummy_data(20, "positive", 100.0);
        let mut sma = None;

        let smas: Vec<Option<SMA>> = (0..n)
            .map(|i| {
                sma = SMA::calculate_rolling(length, i, &candles, &sma);
                sma
            })
            .collect();

        for (i, sma) in smas.iter().enumerate() {
            if i < length - 1 {
                assert!(sma.is_none())
            } else {
                assert!(sma.is_some())
            }
        }

        assert_eq!(smas[n - 1].unwrap().value, 270.0);
    }
}
