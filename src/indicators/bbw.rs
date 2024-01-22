use super::{
    bollinger_bands::BollingerBands, indicator::Indicator, indicator_args::IndicatorArgs,
    indicator_type::IndicatorType, is_indicator::IsIndicator, populates_candles::PopulatesCandles,
};
use crate::models::{candle::Candle, timeseries::TimeSeries};
use anyhow::{anyhow, Context, Result};

/// Bollinger Band Width
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct BBW {
    pub value: f64,
    pub len: usize,
}

impl PopulatesCandles for BBW {
    fn populate_candles(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_candles_args(ts, Self::default_args())
    }

    fn populate_candles_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let (len, _) = args.bb_res()?;
        let indicator_type = IndicatorType::BBW(len);

        for i in 0..ts.candles.len() {
            let end = i + 1;
            let bbw = if end < len {
                None
            } else {
                Self::calculate_args(&ts.candles[end - len..end], &args)
            };

            ts.candles[i]
                .indicators
                .insert(indicator_type, Indicator::BBW(bbw));
        }

        ts.indicators.insert(indicator_type);

        Ok(())
    }

    fn populate_last_candle(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_last_candle_args(ts, Self::default_args())
    }

    fn populate_last_candle_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let (len, _) = args.bb_res()?;
        let ctx_err = "Unable to get last candle";
        let indicator_type = IndicatorType::BBW(len);
        let end = ts.candles.len();

        if end == 0 {
            return Err(anyhow!("No candle to populate"));
        } else if end < len {
            // Not enough candles to populate
            ts.candles
                .last_mut()
                .context(ctx_err)?
                .indicators
                .insert(indicator_type, Indicator::BBW(None));
        } else {
            let new_bbw = Self::calculate_args(&ts.candles[end - len..end], &args);

            ts.candles
                .last_mut()
                .context(ctx_err)?
                .indicators
                .insert(indicator_type, Indicator::BBW(new_bbw));
        }

        Ok(())
    }
}

impl IsIndicator for BBW {
    fn default_args() -> IndicatorArgs {
        IndicatorArgs::BollingerBandArgs(20, 2.0)
    }

    fn calculate(segment: &[Candle]) -> Option<Self>
    where
        Self: Sized,
    {
        let bb = BollingerBands::calculate(segment)?;
        Some(BBW {
            value: (bb.upper - bb.lower) / bb.sma,
            len: segment.len(),
        })
    }

    fn calculate_args(segment: &[Candle], args: &IndicatorArgs) -> Option<Self> 
    where 
        Self: Sized {
        let bb = BollingerBands::calculate_args(segment, args)?;

        Some(BBW {
            value: (bb.upper - bb.lower) / bb.sma,
            len: segment.len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        indicators::{
            bbw::BBW, indicator_type::IndicatorType, is_indicator::IsIndicator,
            populates_candles::PopulatesCandles,
        },
        models::{candle::Candle, interval::Interval, timeseries::TimeSeries},
    };

    #[test]
    fn bbw_calculate() {
        let candles = Candle::dummy_data(20, "positive", 100.0);
        let bbw = BBW::calculate(&candles);
        assert!(bbw.is_some());
        let bbw = bbw.unwrap();
        assert_eq!(bbw.value, 1.1543570308487054);
    }

    #[test]
    fn bbw_calculate_args() {
        let candles = Candle::dummy_data(20, "positive", 100.0);
        let args = BBW::default_args();
        let bbw = BBW::calculate_args(&candles,&args);
        assert!(bbw.is_some());
        let bbw = bbw.unwrap();
        assert_eq!(bbw.value, 1.1543570308487054);
    }

    #[test]
    fn bbw_no_candles() {
        let candles = Vec::new();
        let sma = BBW::calculate(&candles);
        assert!(sma.is_none());
    }

    #[test]
    fn bbw_no_candles_args() {
        let candles = Vec::new();
        let args = BBW::default_args();
        let sma = BBW::calculate_args(&candles, &args);
        assert!(sma.is_none());
    }

    #[test]
    fn bbw_populate_candles() {
        let candles = Candle::dummy_data(25, "positive", 100.0);
        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);

        let _ = BBW::populate_candles(&mut ts);

        let (len, _) = BBW::default_args().bb_opt().unwrap();
        let indicator_type = IndicatorType::BBW(len);

        for (i, candle) in ts.candles.iter().enumerate() {
            let indicator = candle.indicators.get(&indicator_type).unwrap();
            let bbw = indicator.as_bbw();
            if i < len - 1 {
                assert!(bbw.is_none());
            } else {
                assert!(bbw.is_some());
            }
        }

        let last_candle = ts.candles.last().unwrap();
        let last_bbw = last_candle
            .indicators
            .get(&indicator_type)
            .unwrap()
            .as_bbw()
            .unwrap();

        assert_eq!(last_bbw.value, 0.9280125149960182);
    }

    #[test]
    fn bbw_populate_last_candle() {
        let mut candles = Candle::dummy_data(25, "positive", 100.0);
        let candle = candles.pop().unwrap();
        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);
        let _ = BBW::populate_candles(&mut ts);

        let _ = ts.add_candle(candle);
        let (len, _) = BBW::default_args().bb_opt().unwrap();
        let indicator_type = IndicatorType::BBW(len);

        for (i, candle) in ts.candles.iter().enumerate() {
            let indicator = candle.indicators.get(&indicator_type).unwrap();
            let bbw = indicator.as_bbw();
            if i < len - 1 {
                assert!(bbw.is_none());
            } else {
                assert!(bbw.is_some());
            }
        }

        let last_candle = ts.candles.last().unwrap();
        let last_bbw = last_candle
            .indicators
            .get(&indicator_type)
            .unwrap()
            .as_bbw()
            .unwrap();

        assert_eq!(last_bbw.value, 0.9280125149960182);
    }
}
