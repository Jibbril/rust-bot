use crate::{
    indicators::{
        indicator::Indicator, indicator_args::IndicatorArgs, indicator_type::IndicatorType,
        is_indicator::IsIndicator, populates_candles::PopulatesCandles,
    },
    models::{candle::Candle, timeseries::TimeSeries},
    utils::math::{sma, std},
};
use anyhow::{anyhow, Context, Result};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct BollingerBands {
    pub upper: f64,
    pub lower: f64,
    pub sma: f64,
    pub std: f64,
    pub len: usize,
}

impl PopulatesCandles for BollingerBands {
    fn populate_candles(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_candles_args(ts, Self::default_args())
    }

    fn populate_candles_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let (len, _) = args.bb_res()?;
        let indicator_type = IndicatorType::BollingerBands(len);

        for i in 0..ts.candles.len() {
            let end = i + 1;
            let bb = if end < len {
                None
            } else {
                Self::calculate_args(&ts.candles[end - len..end], &args)
            };

            ts.candles[i]
                .indicators
                .insert(indicator_type, Indicator::BollingerBands(bb));
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
        let indicator_type = IndicatorType::BollingerBands(len);
        let end = ts.candles.len();

        if end == 0 {
            return Err(anyhow!("No candle to populate"));
        } else if end < len {
            // Not enough candles to populate
            ts.candles
                .last_mut()
                .context(ctx_err)?
                .indicators
                .insert(indicator_type, Indicator::BollingerBands(None));
        } else {
            let new_bb = Self::calculate_args(&ts.candles[end - len..end], &args);

            ts.candles
                .last_mut()
                .context(ctx_err)?
                .indicators
                .insert(indicator_type, Indicator::BollingerBands(new_bb));
        }

        Ok(())
    }
}

impl IsIndicator for BollingerBands {
    fn default_args() -> IndicatorArgs {
        IndicatorArgs::BollingerBandArgs(20, 2.0)
    }

    /// Segment should have the same number of candles as the desired length of
    /// BollingerBands wanted.
    fn calculate(segment: &[Candle]) -> Option<Self>
    where
        Self: Sized,
    {
        if segment.len() == 0 {
            return None;
        }

        let (_, std_n) = Self::default_args().bb_opt()?;
        let args = IndicatorArgs::BollingerBandArgs(segment.len(), std_n);

        Self::calculate_bb(segment, &args)
    }

    fn calculate_args(segment: &[Candle], args: &IndicatorArgs) -> Option<Self>
    where
        Self: Sized,
    {
        let (arg_len, _) = args.bb_opt()?;
        let candle_len = segment.len();

        if candle_len < arg_len {
            return None;
        }

        Self::calculate_bb(&segment[candle_len - arg_len..candle_len], args)
    }
}

impl BollingerBands {
    fn calculate_bb(segment: &[Candle], args: &IndicatorArgs) -> Option<Self>
    where
        Self: Sized,
    {
        let (_, std_n) = args.bb_opt()?;
        let values: Vec<f64> = segment.iter().map(|c| c.close).collect();

        let sma = sma(&values);
        let std = std(&values, sma);

        let upper = sma + std_n * std;
        let lower = sma - std_n * std;

        Some(BollingerBands {
            upper,
            lower,
            std,
            sma,
            len: segment.len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        indicators::{
            bollinger_bands::BollingerBands, indicator_type::IndicatorType,
            is_indicator::IsIndicator, populates_candles::PopulatesCandles,
        },
        models::{candle::Candle, interval::Interval, timeseries_builder::TimeSeriesBuilder},
    };

    #[test]
    fn bb_calculate() {
        let candles = Candle::dummy_data(20, "positive", 100.0);
        let bb = BollingerBands::calculate(&candles);
        assert!(bb.is_some());
        let bb = bb.unwrap();
        println!("BB: {}", bb.upper);
        assert!(bb.upper - 323.32159566199 < 0.0001)
    }

    #[test]
    fn bb_calculate_args() {
        let candles = Candle::dummy_data(20, "positive", 100.0);
        let args = BollingerBands::default_args();
        let bb = BollingerBands::calculate_args(&candles, &args);
        assert!(bb.is_some());
        let bb = bb.unwrap();
        println!("BB: {}", bb.upper);
        assert!(bb.upper - 323.32159566199 < 0.0001)
    }

    #[test]
    fn bb_no_candles() {
        let candles = Vec::new();
        let sma = BollingerBands::calculate(&candles);
        assert!(sma.is_none());
    }

    #[test]
    fn bb_populate_candles() {
        let candles = Candle::dummy_data(25, "positive", 100.0);
        let mut ts = TimeSeriesBuilder::new()
            .symbol("DUMMY".to_string())
            .interval(Interval::Day1)
            .candles(candles)
            .build();

        let _ = BollingerBands::populate_candles(&mut ts);

        let (len, _) = BollingerBands::default_args().bb_opt().unwrap();
        let indicator_type = IndicatorType::BollingerBands(len);

        for (i, candle) in ts.candles.iter().enumerate() {
            let indicator = candle.indicators.get(&indicator_type).unwrap();
            let bb = indicator.as_bollinger_bands();
            if i < len - 1 {
                assert!(bb.is_none());
            } else {
                assert!(bb.is_some());
            }
        }

        let last_candle = ts.candles.last().unwrap();
        let last_bb = last_candle
            .indicators
            .get(&indicator_type)
            .unwrap()
            .as_bollinger_bands()
            .unwrap();

        assert_eq!(last_bb.upper, 373.3215956619923);
    }

    #[test]
    fn bb_populate_last_candle() {
        let mut candles = Candle::dummy_data(25, "positive", 100.0);
        let candle = candles.pop().unwrap();
        let mut ts = TimeSeriesBuilder::new()
            .symbol("DUMMY".to_string())
            .interval(Interval::Day1)
            .candles(candles)
            .build();
        let _ = BollingerBands::populate_candles(&mut ts);

        let _ = ts.add_candle(&candle);

        let (len, _) = BollingerBands::default_args().bb_opt().unwrap();
        let indicator_type = IndicatorType::BollingerBands(len);

        for (i, candle) in ts.candles.iter().enumerate() {
            let indicator = candle.indicators.get(&indicator_type).unwrap();
            let bb = indicator.as_bollinger_bands();
            if i < len - 1 {
                assert!(bb.is_none());
            } else {
                assert!(bb.is_some());
            }
        }

        let last_candle = ts.candles.last().unwrap();
        let last_bb = last_candle
            .indicators
            .get(&indicator_type)
            .unwrap()
            .as_bollinger_bands()
            .unwrap();

        assert_eq!(last_bb.upper, 373.3215956619923);
    }
}
