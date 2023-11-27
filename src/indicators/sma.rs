use anyhow::{Context, Result};
use crate::{
    models::{calculation_mode::CalculationMode, candle::Candle, timeseries::TimeSeries},
    utils::math::sma,
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
        let indicator_type = IndicatorType::SMA(len);

        for i in 0..ts.candles.len() {
            let end = i + 1;
            let sma = if end < len {
                None
            } else {
                let start= end - len;
                Self::calculate(&ts.candles[start..end])
            };

            ts.candles[i].indicators.insert(indicator_type, Indicator::SMA(sma));
        }

        ts.indicators.insert(indicator_type);

        Ok(())
    }

    fn populate_last_candle(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_last_candle_args(ts, Self::default_args())
    }

    fn populate_last_candle_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let len = args.extract_len_res()?;
        let start = ts.candles.len() - len;
        let end = ts.candles.len() - 1;

        let new_sma = Self::calculate(&ts.candles[start..end]);

        let new_candle = ts.candles.last_mut().context("Failed to get last candle")?;

        new_candle
            .indicators
            .insert(IndicatorType::SMA(len), Indicator::SMA(new_sma));

        Ok(())
    }
}

impl IsIndicator for SMA {
    fn default_args() -> IndicatorArgs {
        IndicatorArgs::LengthArg(8)
    }

    fn calculate(segment: &[Candle]) -> Option<Self>
    where Self: Sized {
        Self::calculate_by_mode(segment, CalculationMode::Close)
    }

    fn calculate_by_mode(segment: &[Candle], mode: CalculationMode) -> Option<Self>
    where Self: Sized {
        let len = segment.len();

        if len == 0 {
            return None;
        }

        let values: Vec<f64> = segment.iter().map(|c| c.price_by_mode(&mode)).collect();

        Some(SMA {
            len,
            value: sma(&values),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{models::candle::Candle, indicators::is_indicator::IsIndicator};
    use super::SMA;

    #[test]
    fn calculate_sma() {
        let candles = Candle::dummy_data(4, "positive", 100.0);
        println!("Candles:{:#?}", candles);
        let sma = SMA::calculate(&candles[1..4]);
        assert!(sma.is_some());
        let sma = sma.unwrap();
        assert_eq!(sma.value, 130.0);
    }

    #[test]
    fn calculate_sma_single() {
        let candles = Candle::dummy_data(1, "positive", 100.0);
        println!("Candles:{:#?}", candles);
        let sma = SMA::calculate(&candles);
        assert!(sma.is_some());
        let sma = sma.unwrap();
        assert_eq!(sma.value, 110.0);
    }

    #[test]
    fn sma_no_candles() {
        let candles: Vec<Candle> = Vec::new();
        let sma = SMA::calculate(&candles);
        assert!(sma.is_none());
    }
}
