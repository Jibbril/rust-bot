use anyhow::{anyhow, Context, Result};
use super::{
    bbw::BBW, indicator::Indicator, indicator_args::IndicatorArgs, indicator_type::IndicatorType,
    is_indicator::IsIndicator, populates_candles::PopulatesCandles, sma::SMA,
};
use crate::{
    models::{candle::Candle, timeseries::TimeSeries},
    utils::math::sma,
};

/// Bollinger Band Width Percentile
///
/// Indicator based on Caretaker's Tradingview BBWP indicator found
/// [here](https://www.tradingview.com/script/tqitSsyG-Bollinger-Band-Width-Percentile/).
/// It measures the percentage of candles over a specified lookback period
/// where the Bollinger Band Width was less than the current Bollinger Band Width.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct BBWP {
    pub value: f64,
    pub len: usize,
    pub lookback: usize,
    pub sma: Option<SMA>,
}

// Default number of standard deviations for BBW calculation
const BBW_STD_N: f64 = 1.0;

impl PopulatesCandles for BBWP {
    fn populate_candles_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let (len, lookback, sma_len) = args.extract_bbwp_res()?;
        let indicator_type = IndicatorType::BBWP(len, lookback);

        for i in 0..ts.candles.len() {
            let end = i + 1;
            let bbwp = if end < len {
                None
            } else {
                let start = if end > lookback + len { end - lookback - len } else { 0 };

                Self::calculate(&ts.candles[start..end])
            };

            ts.candles[i]
                .indicators
                .insert(indicator_type, Indicator::BBWP(bbwp));

            // Not enough candles to populate bbwp sma so return
            if end < len + sma_len { continue; }

            let mut bbwp = bbwp.context("Unable to calculate BBWP")?;
            let sma_segment = &ts.candles[end - sma_len..end];
            let sma = Self::bbwp_sma(sma_segment, &indicator_type);

            if let Some(sma) = sma {
                bbwp.sma = Some(SMA {
                    len: sma_len,
                    value: sma,
                });

                ts.candles[i]
                    .indicators
                    .insert(indicator_type, Indicator::BBWP(Some(bbwp)));
            }
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
        let (len, lookback, sma_len) = args.extract_bbwp_res()?;
        let ctx_err = "Unable to get last candle";
        let indicator_type = IndicatorType::BBWP(len, lookback);
        let end = ts.candles.len();

        if end == 0 {
            return Err(anyhow!("No candle to populate"));
        }

        // Not enough candles to populate
        if end < len {
            ts.candles
                .last_mut()
                .context(ctx_err)?
                .indicators
                .insert(indicator_type, Indicator::BBWP(None));

            return Ok(());
        }

        let start = if end > lookback + len { end - lookback - len } else { 0 };

        let mut new_bbwp =
            Self::calculate(&ts.candles[start..end]).context("Unable to calculate BBWP")?;

        ts.candles
            .last_mut()
            .context(ctx_err)?
            .indicators
            .insert(indicator_type, Indicator::BBWP(Some(new_bbwp)));

        // Not enough candles to populate bbwp sma so return
        if end < len + sma_len {
            return Ok(());
        }

        let sma_segment = &ts.candles[end - sma_len..end];
        let sma = Self::bbwp_sma(sma_segment, &indicator_type);

        if let Some(sma) = sma {
            new_bbwp.sma = Some(SMA {
                len: sma_len,
                value: sma,
            });

            ts.candles
                .last_mut()
                .context(ctx_err)?
                .indicators
                .insert(indicator_type, Indicator::BBWP(Some(new_bbwp)));
        }

        Ok(())
    }
}

impl IsIndicator for BBWP {
    fn default_args() -> IndicatorArgs {
        IndicatorArgs::BBWPArgs(13, 252, 5) // len, lookback, sma_len
    }

    /// Note: Due to api based on segment length, the segment needs "len" 
    /// number of extra candles. This is due to the get_bbws function resulting
    /// in the "len" first being empty values.
    fn calculate(segment: &[Candle]) -> Option<Self>
    where
        Self: Sized,
    {
        let args = Self::default_args();
        let bbws = Self::get_bbws(segment, args).ok()?;
        let (len, lookback, _) = args.extract_bbwp_opt()?;
        let new_bbw = bbws.last()?.as_ref()?;

        let count = bbws
            .iter()
            .rev()
            .skip(1) // Skip last
            .filter(|&old_bbw| old_bbw.map_or(false, |old_bbw| old_bbw.value < new_bbw.value))
            .count();

        let bbwp = (count as f64) / (lookback as f64);

        Some(BBWP {
            len,
            lookback,
            value: bbwp,
            sma: None,
        })
    }
}

impl BBWP {
    fn bbwp_sma(segment: &[Candle], indicator_type: &IndicatorType) -> Option<f64> {
        let values: Vec<f64> = segment
            .iter()
            .filter_map(|c| c.get_indicator(indicator_type).ok()?.as_bbwp())
            .map(|bbwp| bbwp.value)
            .collect();

        (values.len() == segment.len()).then_some(sma(&values))
    }

    fn get_bbws(segment: &[Candle], args: IndicatorArgs) -> Result<Vec<Option<BBW>>> {
        // TODO: Refactor populates candles trait to include methods accepting
        // just candles instead of only TimeSeries to not need temporary
        // TimeSeries below.

        let temp_segment = segment.to_vec();
        let mut temp_ts = TimeSeries::dummy();
        temp_ts.set_candles(&temp_segment);

        let (len, _, _) = args.extract_bbwp_res()?;
        let bbw_args = IndicatorArgs::BollingerBandArgs(len, BBW_STD_N);
        BBW::populate_candles_args(&mut temp_ts, bbw_args)?;

        let ind_type = IndicatorType::BBW(len);

        let bbws: Vec<Option<BBW>> = temp_ts
            .candles
            .iter()
            .map(|candle| candle.get_indicator(&ind_type).unwrap().as_bbw())
            .collect();

        Ok(bbws)
    }
}

#[cfg(test)]
mod tests {
    use super::BBWP;
    use crate::{
        indicators::{indicator_type::IndicatorType, populates_candles::PopulatesCandles, is_indicator::IsIndicator},
        models::{candle::Candle, interval::Interval, timeseries::TimeSeries},
    };

    const BBWP_ARR: &[f64] = &[
            5.09929, 9.74627, -5.33611, 1.94130, -8.76965, 6.66391, 7.27652, 5.97592, -9.85165,
            2.48223, 0.73204, -3.49922, 7.75743, 1.92533, 8.93053, 4.81184, 8.60267, 1.67297,
            -8.45413, -3.34135, 1.88662, -7.90070, 7.66579, -1.48181, 0.45743, -5.20923, -5.57120,
            3.27894, 9.25476, 5.44872, 8.08520, 4.88609, 1.60502, -6.15534, -3.74093, 4.79746,
            -0.26614, -3.23332, -2.81233, 6.96754, -0.26375, 8.06637, 9.84540, 2.02764, 0.32202,
            4.01706, -1.57054, -5.70554, 1.69739, -3.24667, -5.57416, -2.78992, -5.95689, -0.10813,
            4.70929, 9.38824, -0.15406, 0.46125, 6.81064, -9.26099, -5.51741, -8.62995, 7.13186,
            5.61680, -5.19655, 8.61658, 1.19507, -9.31360, 5.16195, 8.08518, -7.37624, -6.52316,
            6.69292, -8.16211, -2.20704, -8.95979, -3.65263, -7.86101, -8.68198, 9.97881, -6.33012,
            -6.51086, -6.39386, 9.95169, 0.45764, -0.80704, -7.46162, 0.90591, 6.32008, -8.10575,
            -6.28365, -7.57228, 5.71232, -6.53414, 5.91684, -3.87323, 0.41023, 9.70348, -2.34391,
            -1.11017, -2.69082, 3.55004, -0.99868, 9.76437, -6.58008, 7.68341, -1.50724, 5.87289,
            7.68080, 0.98624, 1.73755, 5.19502, -3.67264, -2.45444, -4.53397, 2.63463, 7.68003,
            -3.53374, -8.97328, 3.51714, -8.43237, 7.60477, 5.22704, 9.09315, -0.27050, -4.34116,
            -4.96585, -2.40816, -3.42676, 6.73872, -7.67418, 4.12670, -7.67422, 0.85692, 1.09512,
            -2.55589, 6.12801, -4.65786, 4.00806, 2.63249, -3.37329, 4.77087, -6.21501, -0.67501,
            -4.05720, -0.91877, -2.46674, 3.68148, 9.46407, -9.40343, -6.72730, 9.05130, 9.85621,
            1.16687, 8.96887, 2.87725, -2.39741, -7.95583, 9.96677, 6.71668, 4.46728, 7.91307,
            1.81803, -0.33008, 4.73469, -2.28515, -8.37071, 8.95568, 3.36460, 8.98722, 0.58397,
            -9.76622, 0.25766, 0.95967, -6.78898, -4.30787, -0.29860, 9.72089, 0.66026, 3.34911,
            -5.68001, 5.35604, -9.60524, 1.55735, -9.10656, 8.98286, 9.39720, 1.76726, -5.51635,
            -2.04816, 2.39353, 4.24824, 5.42231, -1.47941, 1.49109, 5.38433, -3.98039, -4.24454,
            -5.39086, -5.61240, 8.65354, -1.92267, -8.07234, -4.11950, -5.37963, 6.59454, -4.83404,
            0.59662, -5.74362, 9.29519, 9.08617, 0.74084, 7.44952, 1.01144, 8.77533, 6.28296,
            -9.83937, -5.73880, 2.45188, 4.74567, -6.54124, -8.94585, 9.95212, 7.32448, -8.34788,
            0.86771, -1.70766, -1.39426, -0.48166, 2.72247, -6.89342, -3.57928, 2.45347, -3.55399,
            4.13076, -6.48023, -3.29791, -6.63057, 6.20341, -5.06252, 7.84040, 2.64205, -7.22499,
            1.27961, 4.03425, -2.83070, 1.17285, -2.11044, 2.73131, 5.46444, -0.68890, 1.02285,
            7.81546, -2.31716, 8.65694, -9.71960, -7.55042, -1.63520, -0.91716, 5.32822, -7.91676,
            -7.64299, -4.64315, -6.70301, 1.27140, 4.84216, -6.91571, -3.65495, -5.04685, 1.37528,
            -9.40131, 0.29588, -9.82542, 9.55198, -2.51155, 2.03442, 3.47383, -8.18181, 8.95236,
            -3.65556, 1.71486, -1.64510, 7.05365, 1.25140, -8.84391, -4.15882, -5.72397, 8.26261,
            -6.67113, -2.68339, 3.49222, 6.76455, 4.46842, -6.42984, 8.81598, -2.36711, -2.45753,
            -3.82352, -6.04056, 8.48688,
        ];
    const FINAL_VALUES: &[f64] = &[
        0.5238095238095238,
        0.5515873015873016,
        0.5436507936507936,
        0.5079365079365079,
        0.4722222222222222,
    ];

    #[test]
    fn calculate_bbwp() {
        let candles = Candle::dummy_from_increments(&BBWP_ARR);

        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);

        let _ = BBWP::populate_candles(&mut ts);

        let segment = &ts.candles[ts.candles.len() - 5..];

        let (len, lookback, _) = BBWP::default_args().extract_bbwp_opt().unwrap();
        for (i, val) in FINAL_VALUES.iter().enumerate() {
            let bbwp = segment[i]
                .get_indicator(&IndicatorType::BBWP(len, lookback))
                .unwrap()
                .as_bbwp()
                .unwrap();
            assert_eq!(*val, bbwp.value)
        }
    }

    #[test]
    fn bbwp_no_candles() {
        let candles = Vec::new();
        let sma = BBWP::calculate(&candles);
        assert!(sma.is_none());
    }

    #[test]
    fn bbwp_populate_candles() {
        let candles = Candle::dummy_from_increments(&BBWP_ARR);
        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);

        let _ = BBWP::populate_candles(&mut ts);

        let (len, lookback, _sma_len) = BBWP::default_args().extract_bbwp_opt().unwrap();
        let indicator_type = IndicatorType::BBWP(len,lookback);

        for (i, candle) in ts.candles.iter().enumerate() {
            let indicator = candle.indicators.get(&indicator_type).unwrap();
            let bbwp = indicator.as_bbwp();
            if i < len - 1 {
                assert!(bbwp.is_none());
            } else {
                assert!(bbwp.is_some());
            }
        }


        let segment = &ts.candles[ts.candles.len() - 5..];

        let (len, lookback, _) = BBWP::default_args().extract_bbwp_opt().unwrap();
        for (i, val) in FINAL_VALUES.iter().enumerate() {
            let bbwp = segment[i]
                .get_indicator(&IndicatorType::BBWP(len, lookback))
                .unwrap()
                .as_bbwp()
                .unwrap();
            assert_eq!(*val, bbwp.value)
        }
    }

    #[test]
    fn bbwp_populate_last_candle() {
        let mut candles = Candle::dummy_from_increments(&BBWP_ARR);
        candles.pop();

        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);
        let _ = BBWP::populate_candles(&mut ts);

        let candle = Candle::dummy_from_val(1768.6946700000012);
        let _ = ts.add_candle(candle);

        let (len, lookback, _sma_len) = BBWP::default_args().extract_bbwp_opt().unwrap();
        let indicator_type = IndicatorType::BBWP(len, lookback);

        for (i, candle) in ts.candles.iter().enumerate() {
            let indicator = candle.indicators.get(&indicator_type).unwrap();
            let bbwp = indicator.as_bbwp();
            if i < len - 1 {
                assert!(bbwp.is_none());
            } else {
                assert!(bbwp.is_some());
            }
        }

        let last_candle = ts.candles.last().unwrap();
        let last_bbwp = last_candle
            .indicators
            .get(&indicator_type)
            .unwrap()
            .as_bbwp()
            .unwrap();

        assert_eq!(&last_bbwp.value, FINAL_VALUES.last().unwrap());
    }
}
