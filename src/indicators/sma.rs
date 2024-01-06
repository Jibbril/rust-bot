use super::{
    indicator::Indicator, indicator_args::IndicatorArgs, indicator_type::IndicatorType,
    is_indicator::IsIndicator, populates_candles::PopulatesCandles,
};
use crate::{
    models::{candle::Candle, timeseries::TimeSeries},
    utils::math::sma,
};
use anyhow::{anyhow, Context, Result};

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
        let indicator_type = IndicatorType::SMA(len);

        for i in 0..ts.candles.len() {
            let end = i + 1;
            let sma = if end < len {
                None
            } else {
                let start = end - len;
                Self::calculate(&ts.candles[start..end])
            };

            ts.candles[i]
                .indicators
                .insert(indicator_type, Indicator::SMA(sma));
        }

        ts.indicators.insert(indicator_type);

        Ok(())
    }

    fn populate_last_candle(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_last_candle_args(ts, Self::default_args())
    }

    fn populate_last_candle_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let len = args.extract_len_res()?;
        let end = ts.candles.len();
        let ctx_err = "Failed to get last candle";
        let indicator_type = IndicatorType::SMA(len);

        if end == 0 {
            return Err(anyhow!("No candle to populate"));
        } else if end < len {
            // Not enough candles to populate
            ts.candles
                .last_mut()
                .context(ctx_err)?
                .indicators
                .insert(indicator_type, Indicator::SMA(None));
        } else {
            let new_sma = Self::calculate(&ts.candles[end - len..end]);

            ts.candles
                .last_mut()
                .context(ctx_err)?
                .indicators
                .insert(indicator_type, Indicator::SMA(new_sma));
        }

        Ok(())
    }
}

impl IsIndicator for SMA {
    fn default_args() -> IndicatorArgs {
        IndicatorArgs::LengthArg(8)
    }

    fn calculate(segment: &[Candle]) -> Option<Self>
    where
        Self: Sized,
    {
        let len = segment.len();
        if len == 0 {
            return None;
        }

        let values: Vec<f64> = segment.iter().map(|c| c.close).collect();

        Some(SMA {
            len,
            value: sma(&values),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::SMA;
    use crate::{
        indicators::{
            indicator_type::IndicatorType, is_indicator::IsIndicator,
            populates_candles::PopulatesCandles,
        },
        models::{candle::Candle, interval::Interval, timeseries::TimeSeries},
    };

    #[test]
    fn calculate_sma() {
        let candles = Candle::dummy_data(4, "positive", 100.0);
        let sma = SMA::calculate(&candles[1..4]);
        assert!(sma.is_some());
        let sma = sma.unwrap();
        assert_eq!(sma.value, 130.0);
    }

    #[test]
    fn calculate_sma_single() {
        let candles = Candle::dummy_data(1, "positive", 100.0);
        let sma = SMA::calculate(&candles);
        assert!(sma.is_some());
        let sma = sma.unwrap();
        assert_eq!(sma.value, 110.0);
    }

    #[test]
    fn sma_populate_candles() {
        let candles = Candle::dummy_data(10, "positive", 100.0);
        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);

        let _ = SMA::populate_candles(&mut ts);

        let len = SMA::default_args().extract_len_opt().unwrap();
        let indicator_type = IndicatorType::SMA(len);

        for (i, candle) in ts.candles.iter().enumerate() {
            let indicator = candle.indicators.get(&indicator_type).unwrap();
            let sma = indicator.as_sma();
            if i < len - 1 {
                assert!(sma.is_none());
            } else {
                assert!(sma.is_some());
            }
        }

        let last_candle = ts.candles.last().unwrap();
        let last_sma = last_candle
            .indicators
            .get(&indicator_type)
            .unwrap()
            .as_sma()
            .unwrap();
        assert_eq!(last_sma.value, 165.0);
    }

    #[test]
    fn sma_no_candles() {
        let candles: Vec<Candle> = Vec::new();
        let sma = SMA::calculate(&candles);
        assert!(sma.is_none());
    }

    #[test]
    fn sma_populate_last_candle() {
        let mut candles = Candle::dummy_data(10, "positive", 100.0);
        let candle = candles.pop().unwrap();

        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);
        let _ = SMA::populate_candles(&mut ts);
        let _ = ts.add_candle(candle);

        let len = SMA::default_args().extract_len_opt().unwrap();
        let indicator_type = IndicatorType::SMA(len);

        for (i, candle) in ts.candles.iter().enumerate() {
            let indicator = candle.indicators.get(&indicator_type).unwrap();
            let sma = indicator.as_sma();
            if i < len - 1 {
                assert!(sma.is_none());
            } else {
                assert!(sma.is_some());
            }
        }

        let last_candle = ts.candles.last().unwrap();
        let last_sma = last_candle
            .indicators
            .get(&indicator_type)
            .unwrap()
            .as_sma()
            .unwrap();

        assert_eq!(last_sma.value, 165.0);
    }
}
