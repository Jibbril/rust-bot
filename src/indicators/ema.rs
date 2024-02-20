use super::{
    indicator::Indicator, indicator_args::IndicatorArgs, indicator_type::IndicatorType,
    is_indicator::IsIndicator, populates_candles::PopulatesCandles,
};
use crate::{
    models::{candle::Candle, timeseries::TimeSeries},
    utils::math::{ema_rolling, sma},
};
use anyhow::{anyhow, Context, Result};

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
        let len = args.len_res()?;
        let indicator_type = IndicatorType::EMA(len);

        let mut prev_ema: Option<EMA> = None;
        for i in 0..ts.candles.len() {
            let end = i;
            let ema = if end < len {
                None
            } else if end == len || prev_ema.is_none() {
                let start = end - len;
                Self::calculate_args(&ts.candles[start..end + 1], &args)
            } else {
                let prev = prev_ema.unwrap().value;
                let current = ts.candles[end].close;
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
        Self::populate_last_candle_args(ts, Self::default_args())
    }

    fn populate_last_candle_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let len = args.len_res()?;
        let indicator_type = IndicatorType::EMA(len);
        let end = ts.candles.len();
        let ctx_err = "Unable to get last candle";

        if end == 0 {
            return Err(anyhow!("No candle to populate"));
        }

        // Not enough candles to get new EMA
        if end < len {
            ts.candles
                .last_mut()
                .context(ctx_err)?
                .indicators
                .insert(indicator_type, Indicator::EMA(None));

            return Ok(());
        }

        // Calculate new and populate
        let prev = Indicator::get_second_last(ts, &indicator_type)
            .and_then(|indicator| indicator.as_ema());

        let new_ema = if prev.is_none() {
            let start = end - len;
            let end = end - 1;
            Self::calculate_args(&ts.candles[start..end], &args)
        } else {
            let prev = prev.unwrap();
            let current = ts.candles.last().unwrap().close;
            Self::calculate_rolling(prev.value, current, len)
        };

        ts.candles
            .last_mut()
            .context(ctx_err)?
            .indicators
            .insert(indicator_type, Indicator::EMA(new_ema));

        Ok(())
    }
}

impl IsIndicator for EMA {
    fn default_args() -> IndicatorArgs {
        IndicatorArgs::LengthArg(8)
    }

    /// Segment should be one candle longer than the length of EMA wanted.
    fn calculate(segment: &[Candle]) -> Option<Self>
    where
        Self: Sized,
    {
        if segment.len() == 0 {
            return None;
        }

        let len = segment.len() - 1;
        let initial_values = &segment[..len].to_vec();
        let initial_values: Vec<f64> = initial_values.iter().map(|c| c.close).collect();
        let initial_value = sma(&initial_values);
        let price = segment[len].close;

        let ema = ema_rolling(initial_value, price, len as f64);

        Some(EMA { len, value: ema })
    }

    fn calculate_args(segment: &[Candle], args: &IndicatorArgs) -> Option<Self> 
    where 
        Self: Sized {
        let len = args.len_opt()?;
        let candle_len = segment.len();

        if candle_len < len + 1 {
            return None;
        }

        Self::calculate(&segment[candle_len-len-1..candle_len])
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
            populates_candles::PopulatesCandles, indicator_args::IndicatorArgs,
        },
        models::{candle::Candle, interval::Interval, timeseries_builder::TimeSeriesBuilder},
    };

    #[test]
    fn ema_calculate() {
        let candles = Candle::dummy_data(8, "positive", 100.0);
        let ema = EMA::calculate(&candles);
        assert!(ema.is_some());

        let ema = ema.unwrap();
        assert_eq!(ema.value, 150.0);
    }

    #[test]
    fn ema_calculate_args() {
        let candles = Candle::dummy_data(8, "positive", 100.0);
        let args = IndicatorArgs::LengthArg(7);
        let ema = EMA::calculate_args(&candles, &args);
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
    fn ema_no_candles_args() {
        let candles: Vec<Candle> = Vec::new();
        let args = EMA::default_args();
        let ema = EMA::calculate_args(&candles, &args);
        assert!(ema.is_none());
    }

    #[test]
    fn ema_populate_candles() {
        let candles = Candle::dummy_data(10, "positive", 100.0);
        let mut ts = TimeSeriesBuilder::new()
            .symbol("DUMMY".to_string())
            .interval(Interval::Day1)
            .candles(candles)
            .build();

        let _ = EMA::populate_candles(&mut ts);

        let len = EMA::default_args().len_opt().unwrap();
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
        let mut candles = Candle::dummy_data(10, "positive", 100.0);
        let candle = candles.pop().unwrap();

        let mut ts = TimeSeriesBuilder::new()
            .symbol("DUMMY".to_string())
            .interval(Interval::Day1)
            .candles(candles)
            .build();
        let _ = EMA::populate_candles(&mut ts);
        let _ = ts.add_candle(&candle);

        let len = EMA::default_args().len_opt().unwrap();
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
        let last_ema = last_candle
            .indicators
            .get(&indicator_type)
            .unwrap()
            .as_ema()
            .unwrap();

        assert_eq!(last_ema.value, 165.0);
    }
}
