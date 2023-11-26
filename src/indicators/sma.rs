use anyhow::{Context, Result};
use crate::{
    models::{calculation_mode::CalculationMode, candle::Candle, timeseries::TimeSeries},
    utils::math::{sma, sma_rolling},
};
use super::{
    indicator::Indicator,
    indicator_args::IndicatorArgs,
    indicator_type::IndicatorType,
    is_indicator::IsIndicator,
    populates_candles::PopulatesCandles,
};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct SMA {
    pub value: f64,
    pub len: usize,
}

impl PopulatesCandles for SMA {
    fn populate_candles(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_candles_args(ts, Self::default_args())
    }

    fn populate_candles_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let len = args.extract_len_res()?;
        let mut sma: Option<SMA> = None;
        let new_smas: Vec<Option<SMA>> = (0..ts.candles.len())
            .map(|i| {
                sma = Self::calculate_rolling(len, i, &ts.candles, &sma);
                sma
            })
            .collect();

        let indicator_type = IndicatorType::SMA(len);

        for (i, candle) in ts.candles.iter_mut().enumerate() {
            if new_smas[i].is_none() {
                continue;
            }

            let new_sma = Indicator::SMA(new_smas[i].unwrap());

            candle.indicators.insert(indicator_type, new_sma);
        }

        ts.indicators.insert(indicator_type);

        Ok(())
    }

    fn populate_last_candle(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_last_candle_args(ts, Self::default_args())
    }

    fn populate_last_candle_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let len = args.extract_len_res()?;
        let indicator_type = IndicatorType::SMA(len);
        let prev = Indicator::get_second_last(ts, &indicator_type)
            .and_then(|indicator| indicator.as_sma());

        let new_sma = Self::calculate_rolling(len, ts.candles.len() - 1, &ts.candles, &prev);

        if new_sma.is_some() {
            let new_candle = ts.candles.last_mut().context("Failed to get last candle")?;
            let ma = Indicator::SMA(new_sma.unwrap());
            new_candle.indicators
                .insert(IndicatorType::SMA(len), ma);
        }

        Ok(())
    }
}

impl IsIndicator for SMA {
    fn default_args() -> IndicatorArgs {
        IndicatorArgs::LengthArg(8)
    }
}

impl SMA {
    // Default implementation using closing values for calculations.
    pub fn calculate_rolling(
        len: usize,
        i: usize,
        candles: &Vec<Candle>,
        previous_sma: &Option<SMA>,
    ) -> Option<SMA> {
        Self::calculate_rolling_with_opts(len, i, candles, CalculationMode::Close, previous_sma)
    }

    fn calculate_rolling_with_opts(
        len: usize,
        i: usize,
        candles: &Vec<Candle>,
        mode: CalculationMode,
        previous_sma: &Option<SMA>,
    ) -> Option<SMA> {
        let arr_len = candles.len();
        if i > arr_len || len > arr_len || i < len - 1 {
            None
        } else if let Some(prev_sma) = previous_sma {
            let sma = sma_rolling(
                candles[i].price_by_mode(&mode),
                candles[i - len].price_by_mode(&mode),
                prev_sma.value,
                len as f64,
            );

            Some(SMA { len, value: sma })
        } else {
            Self::calculate(len, i, candles)
        }
    }

    // Default implementation using closing values for calculations.
    pub fn calculate(len: usize, i: usize, candles: &Vec<Candle>) -> Option<SMA> {
        Self::calculate_with_opts(len, i, candles, CalculationMode::Close)
    }

    fn calculate_with_opts(
        len: usize,
        i: usize,
        candles: &Vec<Candle>,
        mode: CalculationMode,
    ) -> Option<SMA> {
        let arr_len = candles.len();
        if i > arr_len || len > arr_len || i < len - 1 {
            None
        } else {
            let start = i + 1 - len;
            let end = i + 1;
            let segment = &candles[start..end];

            let values: Vec<f64> = segment.iter().map(|c| c.price_by_mode(&mode)).collect();

            Some(SMA {
                len,
                value: sma(&values),
            })
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
        let len = 7;
        let candles = Candle::dummy_data(20, "positive", 100.0);
        let mut sma = None;

        let smas: Vec<Option<SMA>> = (0..n)
            .map(|i| {
                sma = SMA::calculate_rolling(len, i, &candles, &sma);
                sma
            })
            .collect();

        for (i, sma) in smas.iter().enumerate() {
            if i < len - 1 {
                assert!(sma.is_none())
            } else {
                assert!(sma.is_some())
            }
        }

        assert_eq!(smas[n - 1].unwrap().value, 270.0);
    }
}
