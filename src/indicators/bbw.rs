use crate::models::{candle::Candle, generic_result::GenericResult, timeseries::TimeSeries};

use super::{bollinger_bands::BollingerBands,  indicator::Indicator, indicator_type::IndicatorType, populates_candles::{PopulatesCandles, IndicatorArgs}};

/// Bollinger Band Width
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct BBW {
    pub bb: BollingerBands,
    pub value: f64,
}

impl PopulatesCandles for BBW {
    fn populate_candles_default(ts: &mut TimeSeries) -> GenericResult<()> {
        let args = IndicatorArgs::BollingerBandArgs(20, 2.0);
        Self::populate_candles(ts, args)
    }

    fn populate_candles(ts: &mut TimeSeries, args: IndicatorArgs) -> GenericResult<()> {
        let (length,_) = args.extract_bb_args_res()?;
        let mut bbw: Option<BBW> = None;
        let new_bbws: Vec<Option<BBW>> = (0..ts.candles.len())
            .map(|i| {
                bbw = Self::calculate_rolling(args, i, &ts.candles, &bbw);
                bbw
            })
            .collect();

        let indicator_type = IndicatorType::BBW(length);

        for (i, candle) in ts.candles.iter_mut().enumerate() {
            let new_bb = Indicator::BBW(new_bbws[i]);

            candle.indicators.insert(indicator_type, new_bb);
        }

        Ok(())
    }
}

impl BBW {
    pub fn calculate(args: IndicatorArgs, i: usize, candles: &[Candle]) -> Option<BBW> {
        let (length, _) = args.extract_bb_args_opt()?;

        if !BollingerBands::calculation_ok(i, length, candles.len()) {
            None
        } else {
            let bb = BollingerBands::calculate(args, i, candles)?;
            Some(BBW {
                bb,
                value: Self::calculate_bbw(&bb),
            })
        }
    }

    pub fn calculate_rolling(
        args: IndicatorArgs,
        i: usize,
        candles: &[Candle],
        prev_bbw: &Option<BBW>,
    ) -> Option<BBW> {
        let (length, _) = args.extract_bb_args_opt()?;
        
        if !BollingerBands::calculation_ok(i, length, candles.len()) {
            return None;
        } else if let Some(prev_bbw) = prev_bbw {
            let prev_bb = Some(prev_bbw.bb);
            let bb = BollingerBands::calculate_rolling(args, i, candles, &prev_bb)?;

            Some(BBW {
                bb,
                value: Self::calculate_bbw(&bb),
            })
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
    use crate::{indicators::{bbw::BBW, populates_candles::IndicatorArgs}, models::candle::Candle};

    #[test]
    fn calculate_bbw() {
        let candles = Candle::dummy_data(20, "positive", 100.0);

        let args = IndicatorArgs::BollingerBandArgs(10,2.0);
        let bbw = BBW::calculate(args, 19, &candles);
        assert!(bbw.is_some());
        let bbw = bbw.unwrap();
        assert!(bbw.value - 0.4749255457 < 0.0001)
    }

    #[test]
    fn bbw_not_enough_data() {
        let candles = Candle::dummy_data(2, "positive", 100.0);

        let args = IndicatorArgs::BollingerBandArgs(20,2.0);
        let bbw = BBW::calculate(args, 19, &candles);

        assert!(bbw.is_none());
    }

    #[test]
    fn bbw_no_candles() {
        let candles: Vec<Candle> = Vec::new();

        let args = IndicatorArgs::BollingerBandArgs(20,2.0);
        let bb = BBW::calculate(args, 19, &candles);

        assert!(bb.is_none());
    }

    #[test]
    fn rolling_bbw() {
        let n = 40;
        let length = 20;
        let args = IndicatorArgs::BollingerBandArgs(length, 2.0);
        let candles = Candle::dummy_data(n, "positive", 100.0);

        let mut bbw: Option<BBW> = None;
        let bbws: Vec<Option<BBW>> = (0..candles.len())
            .map(|i| {
                bbw = BBW::calculate_rolling(args, i, &candles, &bbw);
                bbw
            })
            .collect();

        for (i, bbw) in bbws.iter().enumerate() {
            if i < length - 1 {
                assert!(bbw.is_none())
            } else {
                assert!(bbw.is_some())
            }
        }

        assert!(bbws[n - 1].unwrap().value - 0.58430417610 < 0.00001)
    }
}
