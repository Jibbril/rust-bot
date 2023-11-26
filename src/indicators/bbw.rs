use anyhow::{Context, Result};

use crate::models::{candle::Candle, timeseries::TimeSeries};

use super::{
    bollinger_bands::BollingerBands, indicator::Indicator, indicator_args::IndicatorArgs,
    indicator_type::IndicatorType, is_indicator::IsIndicator, populates_candles::PopulatesCandles,
};

/// Bollinger Band Width
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct BBW {
    pub bb: BollingerBands,
    pub value: f64,
}

impl PopulatesCandles for BBW {
    fn populate_candles(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_candles_args(ts, Self::default_args())
    }

    fn populate_candles_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let (len, _) = args.extract_bb_res()?;
        let new_bbws: Vec<Option<BBW>> = (0..ts.candles.len())
            .map(|i| Self::calculate_rolling(args, i, &ts.candles))
            .collect();

        let indicator_type = IndicatorType::BBW(len);

        for (i, candle) in ts.candles.iter_mut().enumerate() {
            let new_bb = Indicator::BBW(new_bbws[i]);

            candle.indicators.insert(indicator_type, new_bb);
        }

        ts.indicators.insert(indicator_type);

        Ok(())
    }

    fn populate_last_candle(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_last_candle_args(ts, Self::default_args())
    }

    fn populate_last_candle_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let (len, _) = args.extract_bb_res()?;
        let indicator_type = IndicatorType::BBW(len);

        let new_bbw = Self::calculate_rolling(args, ts.candles.len() - 1, &ts.candles);

        let new_candle = ts.candles.last_mut().context("Failed to get last candle")?;

        new_candle
            .indicators
            .insert(indicator_type, Indicator::BBW(new_bbw));

        Ok(())
    }
}

impl IsIndicator for BBW {
    fn default_args() -> IndicatorArgs {
        IndicatorArgs::BollingerBandArgs(20, 2.0)
    }
}

impl BBW {
    pub fn calculate(args: IndicatorArgs, i: usize, candles: &[Candle]) -> Option<BBW> {
        let (len, _) = args.extract_bb_opt()?;

        if !BollingerBands::calculation_ok(i, len, candles.len()) {
            None
        } else {
            let bb = BollingerBands::calculate(args, i, candles)?;
            Some(BBW {
                bb,
                value: Self::calculate_bbw(&bb),
            })
        }
    }

    pub fn calculate_rolling(args: IndicatorArgs, i: usize, candles: &[Candle]) -> Option<BBW> {
        let (len, _) = args.extract_bb_opt()?;

        if !BollingerBands::calculation_ok(i, len, candles.len()) {
            return None;
        } else {
            Self::calculate(args, i, candles)
        }
    }

    fn calculate_bbw(bb: &BollingerBands) -> f64 {
        (bb.upper - bb.lower) / bb.sma.value
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        indicators::{bbw::BBW, indicator_args::IndicatorArgs},
        models::candle::Candle,
    };

    #[test]
    fn calculate_bbw() {
        let candles = Candle::dummy_data(20, "positive", 100.0);

        let args = IndicatorArgs::BollingerBandArgs(10, 2.0);
        let bbw = BBW::calculate(args, 19, &candles);
        assert!(bbw.is_some());
        let bbw = bbw.unwrap();
        assert!(bbw.value - 0.4749255457 < 0.0001)
    }

    #[test]
    fn bbw_not_enough_data() {
        let candles = Candle::dummy_data(2, "positive", 100.0);

        let args = IndicatorArgs::BollingerBandArgs(20, 2.0);
        let bbw = BBW::calculate(args, 19, &candles);

        assert!(bbw.is_none());
    }

    #[test]
    fn bbw_no_candles() {
        let candles: Vec<Candle> = Vec::new();

        let args = IndicatorArgs::BollingerBandArgs(20, 2.0);
        let bb = BBW::calculate(args, 19, &candles);

        assert!(bb.is_none());
    }

    #[test]
    fn rolling_bbw() {
        let n = 40;
        let len = 20;
        let args = IndicatorArgs::BollingerBandArgs(len, 2.0);
        let candles = Candle::dummy_data(n, "positive", 100.0);

        let bbws: Vec<Option<BBW>> = (0..candles.len())
            .map(|i| BBW::calculate_rolling(args, i, &candles))
            .collect();

        for (i, bbw) in bbws.iter().enumerate() {
            if i < len - 1 {
                assert!(bbw.is_none())
            } else {
                assert!(bbw.is_some())
            }
        }

        assert!(bbws[n - 1].unwrap().value - 0.58430417610 < 0.00001)
    }
}
