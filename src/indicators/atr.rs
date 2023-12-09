use super::{
    indicator::Indicator,
    indicator_args::IndicatorArgs,
    indicator_type::IndicatorType,
    is_indicator::IsIndicator,
    populates_candles::PopulatesCandles,
};
use crate::models::{calculation_mode::CalculationMode, candle::Candle, timeseries::TimeSeries};
use anyhow::{Context, Result, anyhow};

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
        let len = args.extract_len_res()?;
        let indicator_type = IndicatorType::ATR(len);

        let mut prev: Option<ATR> = None;

        for i in 0..ts.candles.len() {
            let end = i;
            let atr = if end < len {
                None
            } else if end == len || prev.is_none() {
                let start = end - len;
                Self::calculate(&ts.candles[start..end + 1])
            } else {
                let prev = prev.unwrap();
                let current = &ts.candles[end];
                Self::calculate_rolling(&prev, current, len)
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
        let len = args.extract_len_res()?;
        let indicator_type = IndicatorType::ATR(len);
        let prev = Indicator::get_second_last(ts, &indicator_type)
            .and_then(|indicator| indicator.as_atr());

        let candle_len = ts.candles.len();
        if candle_len < len {
            return Err(anyhow!("Not enough candles to populate."));
        };

        let new_atr = if prev.is_none() {
            let start = candle_len - len;
            let end = candle_len - 1;
            Self::calculate(&ts.candles[start..end])
        } else {
            let prev = prev.unwrap();
            let current = ts.candles.last().unwrap();
            Self::calculate_rolling(&prev, current, len)
        };

        ts.candles
            .last_mut()
            .context("Unable to get last candle.")?
            .indicators
            .insert(IndicatorType::ATR(len), Indicator::ATR(new_atr));

        Ok(())
    }
}

impl ATR {
    fn calculate_rolling(prev: &ATR, current: &Candle, len: usize) -> Option<ATR> {
        let f_len = len as f64;
        let tr = Self::true_range(prev.value, current);
        let atr = (prev.value * (f_len - 1.0) + tr) / f_len;

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

    fn calculate(segment: &[Candle]) -> Option<Self> where Self: Sized, {
        Self::calculate_by_mode(segment, CalculationMode::Close)
    }

    fn calculate_by_mode(segment: &[Candle], mode: CalculationMode) -> Option<Self> where Self: Sized, {
        if segment.len() < 1 {
            return None;
        }

        let len = segment.len() - 1;
        let sum: f64 = (1..segment.len())
            .map(|i| Self::true_range(segment[i - 1].price_by_mode(&mode), &segment[i]))
            .sum();

        Some(ATR {
            len,
            value: sum / (len as f64),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ATR;
    use crate::{models::{candle::Candle, interval::Interval, timeseries::TimeSeries}, indicators::{is_indicator::IsIndicator, populates_candles::PopulatesCandles, indicator_type::IndicatorType}};

    #[test]
    fn calculate_atr() {
        let candles = Candle::dummy_data(6, "positive", 100.0);
        let atr = ATR::calculate(&candles[1..]);
        println!("{:#?}", atr);
        assert!(atr.is_some());
        let atr = atr.unwrap();
        assert_eq!(atr.value, 10.0);
    }

    #[test]
    fn atr_no_candles() {
        let candles = Vec::new();
        let sma = ATR::calculate(&candles);
        assert!(sma.is_none());
    }

    #[test]
    fn atr_populate_candles() {
        let candles = Candle::dummy_data(15, "positive", 100.0);
        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);

        let _ = ATR::populate_candles(&mut ts);

        let len = ATR::default_args().extract_len_opt().unwrap();
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
        let candles = Candle::dummy_data(14, "positive", 100.0);
        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);
        let _ = ATR::populate_candles(&mut ts);

        let candle = Candle::dummy_from_val(250.0);
        let _ = ts.add_candle(candle);

        let len = ATR::default_args().extract_len_opt().unwrap();
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
