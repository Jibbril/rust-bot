use anyhow::Result;

use crate::{
    models::{
        calculation_mode::CalculationMode, candle::Candle, 
        timeseries::TimeSeries,
    },
    utils::math::{ema, ema_rolling},
};

use super::{
    indicator::{Indicator, MovingAverage},
    indicator_args::IndicatorArgs,
    indicator_type::IndicatorType,
    populates_candles::PopulatesCandles,
};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct EMA {
    #[allow(dead_code)] // TODO: Remove once used
    pub value: f64,
    pub len: usize,
}

impl PopulatesCandles for EMA {
    fn populate_candles_default(ts: &mut TimeSeries) -> Result<()> {
        let args = IndicatorArgs::LengthArg(8);
        Self::populate_candles(ts, args)
    }
    fn populate_candles(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let len = args.extract_len_res()?;
        let mut ema: Option<EMA> = None;
        let new_emas: Vec<Option<EMA>> = (0..ts.candles.len())
            .map(|i| {
                ema = Self::calculate_rolling(len, i, &ts.candles, &ema);
                ema
            })
            .collect();

        let indicator_type = IndicatorType::EMA(len);

        for (i, candle) in ts.candles.iter_mut().enumerate() {
            let new_ema = MovingAverage::Exponential(new_emas[i]);
            let new_ema = Indicator::MA(new_ema);

            candle.indicators.insert(indicator_type, new_ema);
        }

        ts.indicators.insert(indicator_type);

        Ok(())
    }
}

impl EMA {
    // Default implementation using closing values for calculations.
    pub fn calculate_rolling(
        len: usize,
        i: usize,
        candles: &Vec<Candle>,
        previous_ema: &Option<EMA>,
    ) -> Option<EMA> {
        Self::calculate_rolling_with_opts(len, i, candles, CalculationMode::Close, previous_ema)
    }

    fn calculate_rolling_with_opts(
        len: usize,
        i: usize,
        candles: &Vec<Candle>,
        mode: CalculationMode,
        previous_ema: &Option<EMA>,
    ) -> Option<EMA> {
        let arr_len = candles.len();
        if i > arr_len || len > arr_len || i < len - 1 {
            None
        } else if let Some(prev_ema) = previous_ema {
            let ema = ema_rolling(prev_ema.value, candles[i].price_by_mode(&mode), len as f64);

            Some(EMA { len, value: ema })
        } else {
            Self::calculate(len, i, candles)
        }
    }

    // Default implementation using closing values for calculations.
    pub fn calculate(len: usize, i: usize, candles: &Vec<Candle>) -> Option<EMA> {
        Self::calculate_with_opts(len, i, candles, CalculationMode::Close)
    }

    fn calculate_with_opts(
        len: usize,
        i: usize,
        candles: &Vec<Candle>,
        mode: CalculationMode,
    ) -> Option<EMA> {
        let arr_len = candles.len();
        if i > arr_len || len > arr_len || i < len - 1 {
            None
        } else {
            let start = i + 1 - len;
            let end = i + 1;
            let segment = &candles[start..end];

            let values: Vec<f64> = segment.iter().map(|c| c.price_by_mode(&mode)).collect();

            Some(EMA {
                len,
                value: ema(&values),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::candle::Candle;

    use super::EMA;

    #[test]
    fn calculate_ema() {
        let data: Vec<f64> = (0..7).map(|i| 100.0 + i as f64).collect();
        let candles = Candle::dummy_from_arr(&data);

        let ema = EMA::calculate(7, 6, &candles);
        assert!(ema.is_some());
        let ema = ema.unwrap();
        assert_eq!(ema.value, 103.75);
    }

    #[test]
    fn ema_not_enough_data() {
        let candles = Candle::dummy_data(2, "positive", 100.0);
        let ema = EMA::calculate(4, 3, &candles);
        assert!(ema.is_none());
    }

    #[test]
    fn ema_no_candles() {
        let candles: Vec<Candle> = Vec::new();
        let ema = EMA::calculate(4, 3, &candles);
        assert!(ema.is_none());
    }

    #[test]
    fn rolling_ema() {
        let len = 7;
        let data: Vec<f64> = (0..len).map(|i| 100.0 + i as f64).collect();
        let mut candles = Candle::dummy_from_arr(&data);
        let initial_ema = EMA::calculate(len, 6, &candles);

        candles.push(Candle::dummy_from_val(107.0));

        let ema = EMA::calculate_rolling(len, len, &candles, &initial_ema);

        assert!(ema.is_some());
        let ema = ema.unwrap();
        assert_eq!(ema.value, 104.5625);
    }
}