use super::{
    indicator::Indicator, indicator_args::IndicatorArgs, indicator_type::IndicatorType,
    is_indicator::IsIndicator, populates_candles::PopulatesCandles,
};
use crate::models::{candle::Candle, timeseries::TimeSeries};
use anyhow::{anyhow, Context, Result};

#[derive(Debug, Copy, Clone)]
pub struct ATR {
    #[allow(dead_code)] // TODO: Remove once used
    pub len: usize,
    pub value: f64,
}

impl PopulatesCandles for ATR {
    fn populate_candles(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_candles_args(ts, Self::default_args())
    }

    fn populate_candles_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let len = args.len_res()?;
        let indicator_type = IndicatorType::ATR(len);

        let mut prev: Option<ATR> = None;

        for i in 0..ts.candles.len() {
            let atr = if i < len {
                None
            } else if i == len || prev.is_none() {
                let start = i - len;
                Self::calculate_args(&ts.candles[start..i + 1], &args)
            } else {
                let candles = (&ts.candles[i - 1], &ts.candles[i]);
                let prev = prev.unwrap();
                Self::calculate_rolling(candles, prev.value, len)
            };

            ts.candles[i]
                .indicators
                .insert(indicator_type, Indicator::ATR(atr));
            prev = atr;
        }

        ts.indicators.insert(indicator_type);

        Ok(())
    }

    fn populate_last_candle(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_last_candle_args(ts, Self::default_args())
    }

    fn populate_last_candle_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let len = args.len_res()?;
        let indicator_type = IndicatorType::ATR(len);
        let prev = Indicator::get_second_last(ts, &indicator_type)
            .and_then(|indicator| indicator.as_atr());
        let ctx_err = "Unable to get last candle.";

        let candle_len = ts.candles.len();
        if candle_len == 0 {
            return Err(anyhow!("Not enough candles to populate."));
        }

        // Not enough candles to populate
        if candle_len <= len {
            ts.candles
                .last_mut()
                .context(ctx_err)?
                .indicators
                .insert(indicator_type, Indicator::ATR(None));

            return Ok(());
        };

        // Calculate new and populate
        let new_atr = if prev.is_none() {
            let start = candle_len - len - 1;
            let end = candle_len;
            Self::calculate_args(&ts.candles[start..end], &args)
        } else {
            let candles = (&ts.candles[candle_len - 2], &ts.candles[candle_len - 1]);
            Self::calculate_rolling(candles, prev.unwrap().value, len)
        };

        ts.candles
            .last_mut()
            .context(ctx_err)?
            .indicators
            .insert(indicator_type, Indicator::ATR(new_atr));

        Ok(())
    }
}

impl ATR {
    fn calculate_rolling(
        (prev_candle, curr_candle): (&Candle, &Candle),
        prev_atr: f64,
        len: usize,
    ) -> Option<ATR> {
        let f_len = len as f64;
        let tr = Self::true_range(prev_candle.close, curr_candle);
        let atr = (prev_atr * (f_len - 1.0) + tr) / f_len;

        Some(ATR { len, value: atr })
    }

    fn true_range(prev: f64, curr: &Candle) -> f64 {
        let a = curr.high - curr.low;
        let b = (curr.high - prev).abs();
        let c = (curr.low - prev).abs();

        a.max(b).max(c)
    }
}

impl IsIndicator for ATR {
    fn default_args() -> IndicatorArgs {
        IndicatorArgs::LengthArg(14)
    }

    fn calculate(segment: &[Candle]) -> Option<Self>
    where
        Self: Sized,
    {
        if segment.len() < 1 {
            return None;
        }

        let len = segment.len() - 1;
        let sum: f64 = (1..segment.len())
            .map(|i| Self::true_range(segment[i - 1].close, &segment[i]))
            .sum();

        Some(ATR {
            len,
            value: sum / (len as f64),
        })
    }

    fn calculate_args(segment: &[Candle], args: &IndicatorArgs) -> Option<Self> where Self: Sized {
        let arg_len = args.len_opt()?;
        let candle_len = segment.len();

        if arg_len >= candle_len {
            return None;
        }
        
        Self::calculate(&segment[candle_len-arg_len-1..candle_len])
    }
}

#[cfg(test)]
mod tests {
    use super::ATR;
    use crate::{
        indicators::{
            indicator_type::IndicatorType, is_indicator::IsIndicator,
            populates_candles::PopulatesCandles, indicator_args::IndicatorArgs,
        },
        models::{candle::Candle, interval::Interval, timeseries::TimeSeries},
    };

    #[test]
    fn atr_calculate() {
        let candles = Candle::dummy_data(6, "positive", 100.0);
        let args = IndicatorArgs::LengthArg(5);
        let atr = ATR::calculate_args(&candles, &args);
        println!("{:#?}", atr);
        assert!(atr.is_some());
        let atr = atr.unwrap();
        assert_eq!(atr.value, 10.0);
    }

    #[test]
    fn atr_no_candles() {
        let candles = Vec::new();
        let args = ATR::default_args();
        let sma = ATR::calculate_args(&candles, &args);
        assert!(sma.is_none());
    }

    #[test]
    fn atr_populate_candles() {
        let candles = Candle::dummy_data(15, "positive", 100.0);
        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);

        let _ = ATR::populate_candles(&mut ts);

        let len = ATR::default_args().len_opt().unwrap();
        let indicator_type = IndicatorType::ATR(len);

        for (i, candle) in ts.candles.iter().enumerate() {
            let indicator = candle.indicators.get(&indicator_type).unwrap();
            let atr = indicator.as_atr();
            if i < len {
                assert!(atr.is_none());
            } else {
                assert!(atr.is_some());
            }
        }

        let last_candle = ts.candles.last().unwrap();
        let last_atr = last_candle
            .indicators
            .get(&indicator_type)
            .unwrap()
            .as_atr()
            .unwrap();

        assert_eq!(last_atr.value, 10.0);
    }

    #[test]
    fn atr_populate_last_candle() {
        let mut candles = Candle::dummy_data(15, "positive", 100.0);
        let candle = candles.pop().unwrap();
        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);
        let _ = ATR::populate_candles(&mut ts);

        let _ = ts.add_candle(candle);
        let len = ATR::default_args().len_opt().unwrap();
        let indicator_type = IndicatorType::ATR(len);

        for (i, candle) in ts.candles.iter().enumerate() {
            let indicator = candle.indicators.get(&indicator_type).unwrap();
            let atr = indicator.as_atr();
            if i < len {
                assert!(atr.is_none());
            } else {
                assert!(atr.is_some());
            }
        }

        let last_candle = ts.candles.last().unwrap();
        let last_atr = last_candle
            .indicators
            .get(&indicator_type)
            .unwrap()
            .as_atr()
            .unwrap();

        assert_eq!(last_atr.value, 10.0);
    }
}
