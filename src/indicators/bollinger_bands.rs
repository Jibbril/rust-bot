use anyhow::{Context, Result, anyhow};

use crate::{
    models::{candle::Candle, timeseries::TimeSeries},
    utils::math::{std, sma},
};

use super::{
    indicator::Indicator, indicator_args::IndicatorArgs, indicator_type::IndicatorType,
    is_indicator::IsIndicator, populates_candles::PopulatesCandles, sma::SMA,
};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct BollingerBands {
    pub upper: f64,
    pub lower: f64,
    pub sma: f64,
    pub std: f64,
    pub len: usize,
}

impl PopulatesCandles for BollingerBands {
    fn populate_candles_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let (len, _) = args.extract_bb_res()?;
        let indicator_type = IndicatorType::BollingerBands(len);

        for i in 0..ts.candles.len() {
            let bb = if i < len {
                None
            } else {
                let start = i - len;
                Self::calculate(&ts.candles[start..i+1])
            };

            ts.candles[i]
                .indicators
                .insert(indicator_type, Indicator::BollingerBands(bb));
        }

        ts.indicators.insert(indicator_type);

        Ok(())
    }

    fn populate_candles(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_candles_args(ts, Self::default_args())
    }

    fn populate_last_candle(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_last_candle_args(ts, Self::default_args())
    }

    fn populate_last_candle_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let (len, _) = args.extract_bb_res()?;
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
            let new_bb = Self::calculate(&ts.candles[end-len..end]);

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

    fn calculate(segment: &[Candle]) -> Option<Self>
    where
        Self: Sized,
    {
        if segment.len() == 0 {
            return None;
        }

        let (_, std_n) = Self::default_args().extract_bb_opt()?;
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
    use super::BollingerBands;
    use crate::{indicators::{indicator_args::IndicatorArgs, is_indicator::IsIndicator, populates_candles::PopulatesCandles, indicator_type::IndicatorType}, models::{candle::Candle, timeseries::TimeSeries, interval::Interval}};

    #[test]
    fn calculate_bollinger_bands() {
        let candles = Candle::dummy_data(20, "positive", 100.0);
        let bb = BollingerBands::calculate(&candles);
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
        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);

        let _ = BollingerBands::populate_candles(&mut ts);

        let (len,_) = BollingerBands::default_args().extract_bb_opt().unwrap();
        let indicator_type = IndicatorType::BollingerBands(len);

        for (i, candle) in ts.candles.iter().enumerate() {
            let indicator = candle.indicators.get(&indicator_type).unwrap();
            let bb = indicator.as_bollinger_bands();
            if i < len {
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

        assert_eq!(last_bb.upper, 374.0967364599086);
    }

    #[test]
    fn bb_populate_last_candle() {
        let candles = Candle::dummy_data(25, "positive", 100.0);
        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);
        let _ = BollingerBands::populate_candles(&mut ts);

        let candle = Candle::dummy_from_val(250.0);
        let _ = ts.add_candle(candle);

        let (len,_) = BollingerBands::default_args().extract_bb_opt().unwrap();
        let indicator_type = IndicatorType::BollingerBands(len);

        for (i, candle) in ts.candles.iter().enumerate() {
            let indicator = candle.indicators.get(&indicator_type).unwrap();
            let bb = indicator.as_bollinger_bands();
            if i < len {
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

        assert_eq!(last_bb.upper, 369.13576058932597);
    }
}
