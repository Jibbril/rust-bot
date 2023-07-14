use crate::{
    utils::calculation_mode::{price_by_calc_mode, CalculationMode},
    utils::{generic_result::GenericResult, timeseries::Candle},
};

use super::{Indicator, IndicatorType, PopulatesCandles};

#[derive(Debug, Copy, Clone)]
pub struct ATR {
    #[allow(dead_code)] // TODO: Remove once used
    pub length: usize,
    pub value: f64,
}

impl PopulatesCandles for ATR {
    fn populate_candles(candles: &mut Vec<Candle>, length: usize) -> GenericResult<()> {
        if candles.len() < length {
            return Err("Length of candles is shorter than indicator length.".into());
        }

        let mut atr: Option<ATR> = None;

        let new_atrs: Vec<Option<ATR>> = (0..candles.len())
            .map(|i| {
                atr = Self::calculate_rolling(length, i, candles, &atr);
                atr
            })
            .collect();

        let indicator_type = IndicatorType::ATR(length);

        for (i, candle) in candles.iter_mut().enumerate() {
            let new_atr = Indicator::ATR(new_atrs[i]);

            candle.indicators.insert(indicator_type, new_atr);
        }

        Ok(())
    }
}

impl ATR {
    // Default implementation using closing values for calculations.
    pub fn calculate_rolling(
        length: usize,
        i: usize,
        candles: &Vec<Candle>,
        prev: &Option<ATR>,
    ) -> Option<ATR> {
        Self::calculate_rolling_with_opts(length, i, candles, CalculationMode::Close, prev)
    }

    fn calculate_rolling_with_opts(
        length: usize,
        i: usize,
        candles: &Vec<Candle>,
        mode: CalculationMode,
        prev: &Option<ATR>,
    ) -> Option<ATR> {
        let arr_length = candles.len();
        if i > arr_length || length > arr_length || i < length - 1 {
            None
        } else if let Some(prev) = prev {
            let f_length = length as f64;
            let tr = Self::true_range(&mode, &candles[i - 1], &candles[i]);
            let atr = (prev.value * (f_length - 1.0) + tr) / f_length;

            Some(ATR { length, value: atr })
        } else {
            Self::calculate(length, i, candles)
        }
    }

    // Default implementation using closing values for calculations.
    pub fn calculate(length: usize, i: usize, candles: &Vec<Candle>) -> Option<ATR> {
        Self::calculate_with_opts(length, i, candles, CalculationMode::Close)
    }

    fn calculate_with_opts(
        length: usize,
        i: usize,
        candles: &Vec<Candle>,
        mode: CalculationMode,
    ) -> Option<ATR> {
        let arr_length = candles.len();
        if i >= arr_length || length > arr_length || i <= length - 1 {
            None
        } else {
            let start = i + 1 - length;
            let end = i + 1;
            let sum: f64 = (start..end)
                .map(|i| Self::true_range(&mode, &candles[i - 1], &candles[i]))
                .sum();

            Some(ATR {
                length,
                value: sum / (length as f64),
            })
        }
    }

    fn true_range(mode: &CalculationMode, prev: &Candle, curr: &Candle) -> f64 {
        let prev_price = price_by_calc_mode(&prev, mode);
        let a = curr.high - curr.low;
        let b = (curr.high - prev_price).abs();
        let c = (curr.low - prev_price).abs();

        a.max(b).max(c)
    }
}

#[cfg(test)]
mod tests {
    use super::ATR;
    use crate::utils::timeseries::Candle;

    #[test]
    fn calculate_atr() {
        let candles = Candle::dummy_data(6, "positive");
        let atr = ATR::calculate(4, 5, &candles);
        println!("ATR: {:#?}", atr);
        assert!(atr.is_some());
        let atr = atr.unwrap();
        assert_eq!(atr.value, 10.0);
    }

    #[test]
    fn atr_not_enough_data() {
        let candles = Candle::dummy_data(2, "positive");
        let sma = ATR::calculate(4, 3, &candles);
        assert!(sma.is_none());
    }

    #[test]
    fn atr_no_candles() {
        let candles: Vec<Candle> = Vec::new();
        let sma = ATR::calculate(4, 3, &candles);
        assert!(sma.is_none());
    }

    #[test]
    fn rolling_atr() {
        let n = 20;
        let length = 7;
        let candles = Candle::dummy_data(20, "positive");
        let mut atr = None;

        let atrs: Vec<Option<ATR>> = (0..n)
            .map(|i| {
                atr = ATR::calculate_rolling(length, i, &candles, &atr);
                atr
            })
            .collect();

        for (i, atr) in atrs.iter().enumerate() {
            if i <= length - 1 {
                assert!(atr.is_none())
            } else {
                assert!(atr.is_some())
            }
        }

        assert_eq!(atrs[n - 1].unwrap().value, 10.0);
    }
}
