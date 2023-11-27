use anyhow::{Context, Result};

use crate::{
    models::{candle::Candle, timeseries::TimeSeries},
    utils::math::std,
};

use super::{
    indicator::Indicator, indicator_args::IndicatorArgs, indicator_type::IndicatorType,
    is_indicator::IsIndicator, populates_candles::PopulatesCandles, sma::SMA,
};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct BollingerBands {
    pub upper: f64,
    pub lower: f64,
    pub sma: SMA,
    pub std: f64,
    pub len: usize,
}

impl PopulatesCandles for BollingerBands {
    fn populate_candles_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let (len, _) = args.extract_bb_res()?;
        let mut bb: Option<BollingerBands> = None;
        let new_bbs: Vec<Option<BollingerBands>> = (0..ts.candles.len())
            .map(|i| {
                bb = Self::calculate_rolling(args, i, &ts.candles, &bb);
                bb
            })
            .collect();

        let indicator_type = IndicatorType::BollingerBands(len);

        for (i, candle) in ts.candles.iter_mut().enumerate() {
            let new_bb = Indicator::BollingerBands(new_bbs[i]);

            candle.indicators.insert(indicator_type, new_bb);
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
        let indicator_type = IndicatorType::BollingerBands(len);

        let previous_bb =
            Indicator::get_second_last(ts, &indicator_type).and_then(|bb| bb.as_bollinger_bands());

        let new_bb = Self::calculate_rolling(args, ts.candles.len() - 1, &ts.candles, &previous_bb);

        let new_candle = ts.candles.last_mut().context("Failed to get last candle")?;

        new_candle
            .indicators
            .insert(indicator_type, Indicator::BollingerBands(new_bb));

        Ok(())
    }
}

impl IsIndicator for BollingerBands {
    fn default_args() -> IndicatorArgs {
        IndicatorArgs::BollingerBandArgs(20, 2.0)
    }

    fn calculate(_segmentt: &[Candle]) -> Option<Self>
    where Self: Sized {
        todo!()
    }

    fn calculate_by_mode(_segment: &[Candle],_modee: crate::models::calculation_mode::CalculationMode) -> Option<Self>
    where Self: Sized {
        todo!()
    }
}

impl BollingerBands {
    pub fn calculation_ok(i: usize, len: usize, arr_len: usize) -> bool {
        i < arr_len && len <= arr_len && i >= len - 1 && len > 0
    }

    #[allow(dead_code)]
    pub fn calculate(args: IndicatorArgs, i: usize, candles: &[Candle]) -> Option<BollingerBands> {
        let (len, std_n) = args.extract_bb_opt()?;
        if !Self::calculation_ok(i, len, candles.len()) {
            None
        } else {
            let start = i + 1 - len;
            let end = i + 1;
            let segment = &candles[start..end];

            // typical price sum
            let tps: Vec<f64> = segment.iter().map(|c| c.close).collect();

            let ma = tps.iter().sum::<f64>() / (len as f64);
            let std = std(&tps, ma);

            let upper = ma + std_n * std;
            let lower = ma - std_n * std;

            Some(BollingerBands {
                upper,
                lower,
                std,
                sma: SMA { len, value: ma },
                len,
            })
        }
    }

    pub fn calculate_rolling(
        args: IndicatorArgs,
        i: usize,
        candles: &[Candle],
        previous_bb: &Option<BollingerBands>,
    ) -> Option<BollingerBands> {
        let (len, _) = args.extract_bb_opt()?;
        if !Self::calculation_ok(i, len, candles.len()) {
            return None;
        } else if let Some(_prev_bb) = previous_bb {
            Self::calculate(args, i, candles)
            // TODO: BELOW PRODUCES INCORRECT RESULTS, FIND BETTER ALGORITHM
            // let f_len = len as f64;
            // let price_in = Self::typical_price(&candles[i]);
            // let price_out = Self::typical_price(&candles[i - len]);
            // let old_sma = prev_bb.sma.value;

            // let new_sma = old_sma + (price_in - price_out) / f_len;

            // let new_var = prev_bb.std.powi(2)
            //     + ((price_in - old_sma) * (price_in - new_sma)
            //         - (price_out - old_sma) * (price_out - new_sma))
            //         / f_len;

            // let new_std = new_var.sqrt();

            // let upper = new_sma + std_n * new_std;
            // let lower = new_sma - std_n * new_std;

            // Some(BollingerBands {
            //     upper,
            //     lower,
            //     std: new_std,
            //     sma: SMA {
            //         len,
            //         value: new_sma,
            //     },
            //     len,
            // })
        } else {
            Self::calculate(args, i, candles)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BollingerBands;
    use crate::{indicators::indicator_args::IndicatorArgs, models::candle::Candle};

    #[test]
    fn calculate_bollinger_bands() {
        let candles = Candle::dummy_data(20, "positive", 100.0);

        let args = IndicatorArgs::BollingerBandArgs(10, 2.0);
        let bb = BollingerBands::calculate(args, 19, &candles);
        assert!(bb.is_some());
        let bb = bb.unwrap();
        assert!(bb.upper - 315.5530070819 < 0.0001)
    }

    #[test]
    fn bb_not_enough_data() {
        let candles = Candle::dummy_data(2, "positive", 100.0);

        let args = IndicatorArgs::BollingerBandArgs(20, 2.0);
        let bb = BollingerBands::calculate(args, 19, &candles);

        assert!(bb.is_none());
    }

    #[test]
    fn bb_no_candles() {
        let candles: Vec<Candle> = Vec::new();

        let args = IndicatorArgs::BollingerBandArgs(20, 2.0);
        let bb = BollingerBands::calculate(args, 19, &candles);

        assert!(bb.is_none());
    }

    #[test]
    fn rolling_bb() {
        let n = 40;
        let len = 20;
        let args = IndicatorArgs::BollingerBandArgs(len, 2.0);
        let candles = Candle::dummy_data(n, "positive", 100.0);

        let mut bb: Option<BollingerBands> = None;
        let bbs: Vec<Option<BollingerBands>> = (0..candles.len())
            .map(|i| {
                bb = BollingerBands::calculate_rolling(args, i, &candles, &bb);
                bb
            })
            .collect();

        for (i, bb) in bbs.iter().enumerate() {
            if i < len - 1 {
                assert!(bb.is_none())
            } else {
                assert!(bb.is_some())
            }
        }

        assert!(bbs[n - 1].unwrap().upper - 523.3215956619 < 0.00001)
    }
}
