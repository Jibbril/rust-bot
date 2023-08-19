use crate::models::{candle::Candle, generic_result::GenericResult, timeseries::TimeSeries};

use super::{bollinger_bands::BollingerBands,  indicator::Indicator, indicator_type::IndicatorType, populates_candles::PopulatesCandles};

/// Bollinger Band Width
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct BBW {
    #[allow(dead_code)] // TODO: Remove once used
    pub bb: BollingerBands,
    pub value: f64,
}

impl PopulatesCandles for BBW {
    fn populate_candles_default(ts: &mut TimeSeries) -> GenericResult<()> {
        Self::populate_candles(ts, 20)
    }

    fn populate_candles(ts: &mut TimeSeries, length: usize) -> GenericResult<()> {
        let mut bbw: Option<BBW> = None;
        let new_bbws: Vec<Option<BBW>> = (0..ts.candles.len())
            .map(|i| {
                bbw = Self::calculate_rolling(length, i, &ts.candles, &bbw);
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
    pub fn calculate(length: usize, i: usize, candles: &[Candle]) -> Option<BBW> {
        if !BollingerBands::calculation_ok(i, length, candles.len()) {
            None
        } else {
            let bb = BollingerBands::calculate(length, i, candles)?;
            Some(BBW {
                bb,
                value: Self::calculate_bbw(&bb),
            })
        }
    }

    pub fn calculate_rolling(
        length: usize,
        i: usize,
        candles: &[Candle],
        prev_bbw: &Option<BBW>,
    ) -> Option<BBW> {
        if !BollingerBands::calculation_ok(i, length, candles.len()) {
            return None;
        } else if let Some(prev_bbw) = prev_bbw {
            let prev_bb = Some(prev_bbw.bb);
            let bb = BollingerBands::calculate_rolling(length, i, candles, &prev_bb)?;

            Some(BBW {
                bb,
                value: Self::calculate_bbw(&bb),
            })
        } else {
            Self::calculate(length, i, candles)
        }
    }

    fn calculate_bbw(bb: &BollingerBands) -> f64 {
        (bb.upper - bb.lower) / bb.sma.value
    }
}

#[cfg(test)]
mod tests {
    use crate::{indicators::bbw::BBW, models::candle::Candle};

    #[test]
    fn calculate_bbw() {
        let candles = Candle::dummy_data(20, "positive", 100.0);

        let bbw = BBW::calculate(10, 19, &candles);
        assert!(bbw.is_some());
        let bbw = bbw.unwrap();
        assert!(bbw.value - 0.4749255457 < 0.0001)
    }

    #[test]
    fn bbw_not_enough_data() {
        let candles = Candle::dummy_data(2, "positive", 100.0);

        let bbw = BBW::calculate(20, 19, &candles);

        assert!(bbw.is_none());
    }

    #[test]
    fn bbw_no_candles() {
        let candles: Vec<Candle> = Vec::new();

        let bb = BBW::calculate(20, 19, &candles);

        assert!(bb.is_none());
    }

    #[test]
    fn rolling_bbw() {
        let n = 40;
        let length = 20;
        let candles = Candle::dummy_data(n, "positive", 100.0);

        let mut bbw: Option<BBW> = None;
        let bbws: Vec<Option<BBW>> = (0..candles.len())
            .map(|i| {
                bbw = BBW::calculate_rolling(length, i, &candles, &bbw);
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
