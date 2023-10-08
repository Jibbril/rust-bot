use anyhow::Result;

use crate::models::{calculation_mode::CalculationMode, candle::Candle, timeseries::TimeSeries};

use super::{
    indicator::Indicator, indicator_args::IndicatorArgs, indicator_type::IndicatorType,
    populates_candles::PopulatesCandles,
};

#[derive(Debug, Copy, Clone)]
pub struct ATR {
    #[allow(dead_code)] // TODO: Remove once used
    pub len: usize,
    pub value: f64,
}

impl PopulatesCandles for ATR {
    fn populate_candles(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let len = args.extract_len_res()?;
        let mut atr: Option<ATR> = None;

        let new_atrs: Vec<Option<ATR>> = (0..ts.candles.len())
            .map(|i| {
                atr = Self::calculate_rolling(len, i, &ts.candles, &atr);
                atr
            })
            .collect();

        let indicator_type = IndicatorType::ATR(len);

        for (i, candle) in ts.candles.iter_mut().enumerate() {
            let new_atr = Indicator::ATR(new_atrs[i]);

            candle.indicators.insert(indicator_type, new_atr);
        }

        ts.indicators.insert(indicator_type);

        Ok(())
    }

    fn populate_candles_default(ts: &mut TimeSeries) -> Result<()> {
        let args = IndicatorArgs::LengthArg(14);
        Self::populate_candles(ts, args)
    }
}

impl ATR {
    // Default implementation using closing values for calculations.
    pub fn calculate_rolling(
        len: usize,
        i: usize,
        candles: &Vec<Candle>,
        prev: &Option<ATR>,
    ) -> Option<ATR> {
        Self::calculate_rolling_with_opts(len, i, candles, CalculationMode::Close, prev)
    }

    fn calculate_rolling_with_opts(
        len: usize,
        i: usize,
        candles: &Vec<Candle>,
        mode: CalculationMode,
        prev: &Option<ATR>,
    ) -> Option<ATR> {
        let arr_len = candles.len();
        if i > arr_len || len > arr_len || i < len - 1 {
            None
        } else if let Some(prev) = prev {
            let f_len = len as f64;
            let tr = Self::true_range(&mode, &candles[i - 1], &candles[i]);
            let atr = (prev.value * (f_len - 1.0) + tr) / f_len;

            Some(ATR { len, value: atr })
        } else {
            Self::calculate(len, i, candles)
        }
    }

    // Default implementation using closing values for calculations.
    pub fn calculate(len: usize, i: usize, candles: &Vec<Candle>) -> Option<ATR> {
        Self::calculate_with_opts(len, i, candles, CalculationMode::Close)
    }

    fn calculate_with_opts(
        len: usize,
        i: usize,
        candles: &Vec<Candle>,
        mode: CalculationMode,
    ) -> Option<ATR> {
        let arr_len = candles.len();
        if i >= arr_len || len > arr_len || i <= len - 1 {
            None
        } else {
            let start = i + 1 - len;
            let end = i + 1;
            let sum: f64 = (start..end)
                .map(|i| Self::true_range(&mode, &candles[i - 1], &candles[i]))
                .sum();

            Some(ATR {
                len,
                value: sum / (len as f64),
            })
        }
    }

    fn true_range(mode: &CalculationMode, prev: &Candle, curr: &Candle) -> f64 {
        let prev_price = prev.price_by_mode(mode);
        let a = curr.high - curr.low;
        let b = (curr.high - prev_price).abs();
        let c = (curr.low - prev_price).abs();

        a.max(b).max(c)
    }
}

#[cfg(test)]
mod tests {
    use super::ATR;
    use crate::models::candle::Candle;

    #[test]
    fn calculate_atr() {
        let candles = Candle::dummy_data(6, "positive", 100.0);
        let atr = ATR::calculate(4, 5, &candles);
        assert!(atr.is_some());
        let atr = atr.unwrap();
        assert_eq!(atr.value, 10.0);
    }

    #[test]
    fn atr_not_enough_data() {
        let candles = Candle::dummy_data(2, "positive", 100.0);
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
        let len = 7;
        let candles = Candle::dummy_data(20, "positive", 100.0);
        let mut atr = None;

        let atrs: Vec<Option<ATR>> = (0..n)
            .map(|i| {
                atr = ATR::calculate_rolling(len, i, &candles, &atr);
                atr
            })
            .collect();

        for (i, atr) in atrs.iter().enumerate() {
            if i <= len - 1 {
                assert!(atr.is_none())
            } else {
                assert!(atr.is_some())
            }
        }

        assert_eq!(atrs[n - 1].unwrap().value, 10.0);
    }
}
