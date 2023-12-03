use super::{
    indicator::Indicator, indicator_args::IndicatorArgs, indicator_type::IndicatorType,
    is_indicator::IsIndicator, populates_candles::PopulatesCandles,
};
use crate::{
    models::{calculation_mode::CalculationMode, candle::Candle, timeseries::TimeSeries},
    utils::math::{ema_rolling, sma},
};
use anyhow::{Context, Result};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct EMA {
    #[allow(dead_code)] // TODO: Remove once used
    pub value: f64,
    pub len: usize,
}

impl PopulatesCandles for EMA {
    fn populate_candles(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_candles_args(ts, Self::default_args())
    }

    fn populate_candles_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let len = args.extract_len_res()?;
        let indicator_type = IndicatorType::EMA(len);

        let mut prev_ema: Option<EMA> = None;
        for i in 0..ts.candles.len() {
            let end = i + 1;
            let ema = if end <= len {
                None
            } else if end == len + 1 || prev_ema.is_none() {
                let start = end - len - 1;
                Self::calculate(&ts.candles[start..end])
            } else {
                let prev = prev_ema.unwrap().value;
                let current = ts.candles[end - 1].close;
                Self::calculate_rolling(prev, current, len)
            };

            ts.candles[i]
                .indicators
                .insert(indicator_type, Indicator::EMA(ema));
            prev_ema = ema;
        }

        ts.indicators.insert(indicator_type);

        Ok(())
    }

    fn populate_last_candle(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_candles_args(ts, Self::default_args())
    }

    fn populate_last_candle_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let len = args.extract_len_res()?;
        let indicator_type = IndicatorType::EMA(len);

        let prev = ts.candles[ts.candles.len() - 2]
            .indicators
            .get(&indicator_type)
            .and_then(|indicator| indicator.as_ema());

        let new_ema = if prev.is_none() {
            let start = ts.candles.len() - len;
            let end = ts.candles.len() - 1;
            Self::calculate(&ts.candles[start..end])
        } else {
            let prev = prev.unwrap();
            let current = ts.candles.last().unwrap().close;
            Self::calculate_rolling(prev.value, current, len)
        };

        let new_candle = ts.candles.last_mut().context("Failed to get last candle")?;
        new_candle
            .indicators
            .insert(indicator_type, Indicator::EMA(new_ema));

        Ok(())
    }
}

impl IsIndicator for EMA {
    fn default_args() -> IndicatorArgs {
        IndicatorArgs::LengthArg(8)
    }

    fn calculate(segment: &[Candle]) -> Option<Self> where Self: Sized, {
        Self::calculate_by_mode(segment, CalculationMode::Close)
    }

    fn calculate_by_mode(segment: &[Candle], mode: CalculationMode) -> Option<Self>
    where
        Self: Sized,
    {
        let len = segment.len();

        if len == 0 {
            return None;
        }

        let len = len - 1; // Exclude current candle

        let initial_values = &segment[..len].to_vec();
        let initial_values: Vec<f64> = initial_values
            .iter()
            .map(|c| c.price_by_mode(&mode))
            .collect();
        let initial_value = sma(&initial_values);
        let price = segment[len].price_by_mode(&mode);

        let ema = ema_rolling(initial_value, price, len as f64);

        Some(EMA { len, value: ema })
    }
}

impl EMA {
    pub fn calculate_rolling(prev: f64, x: f64, len: usize) -> Option<Self> {
        Some(EMA {
            len,
            value: ema_rolling(prev, x, len as f64),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::EMA;
    use crate::{
        indicators::{
            indicator_type::IndicatorType, is_indicator::IsIndicator,
            populates_candles::PopulatesCandles,
        },
        models::{candle::Candle, interval::Interval, timeseries::TimeSeries},
    };

    #[test]
    fn calculate_ema() {
        let candles = Candle::dummy_data(8, "positive", 100.0);
        let ema = EMA::calculate(&candles);
        assert!(ema.is_some());

        let ema = ema.unwrap();
        assert_eq!(ema.value, 150.0);
    }

    #[test]
    fn ema_no_candles() {
        let candles: Vec<Candle> = Vec::new();
        let ema = EMA::calculate(&candles);
        assert!(ema.is_none());
    }

    #[test]
    fn ema_populate_candles() {
        let candles = Candle::dummy_data(10, "positive", 100.0);
        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);

        let _ = EMA::populate_candles(&mut ts);

        let len = EMA::default_args().extract_len_opt().unwrap();
        let indicator_type = IndicatorType::EMA(len);

        for (i, candle) in ts.candles.iter().enumerate() {
            let indicator = candle.indicators.get(&indicator_type).unwrap();
            let ema = indicator.as_ema();
            if i < len {
                assert!(ema.is_none());
            } else {
                assert!(ema.is_some());
            }
        }

        let last_candle = ts.candles.last().unwrap();
        let last_sma = last_candle
            .indicators
            .get(&indicator_type)
            .unwrap()
            .as_ema()
            .unwrap();

        assert_eq!(last_sma.value, 165.0);
    }

    #[test]
    fn ema_populate_last_candle() {
        let candles = Candle::dummy_data(9, "positive", 100.0);
        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);

        let _ = EMA::populate_candles(&mut ts);

        let candle = Candle::dummy_from_val(200.0);

        let _ = ts.add_candle(candle);

        let len = EMA::default_args().extract_len_opt().unwrap();
        let indicator_type = IndicatorType::EMA(len);

        for (i, candle) in ts.candles.iter().enumerate() {
            let indicator = candle.indicators.get(&indicator_type).unwrap();
            let ema = indicator.as_ema();
            if i < len {
                assert!(ema.is_none());
            } else {
                assert!(ema.is_some());
            }
        }

        let last_candle = ts.candles.last().unwrap();
        let last_sma = last_candle
            .indicators
            .get(&indicator_type)
            .unwrap()
            .as_ema()
            .unwrap();

        assert_eq!(last_sma.value, 165.0);
    }
}
