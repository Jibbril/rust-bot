use super::{
    bbw::BBW, indicator::Indicator, indicator_args::IndicatorArgs, indicator_type::IndicatorType,
    populates_candles::PopulatesCandles, sma::SMA,
};
use crate::models::{generic_result::GenericResult, timeseries::TimeSeries};

/// Bollinger Band Width Percentile
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct BBWP {
    pub value: f64,
    pub len: usize,
    pub lookback: usize,
    pub bbw: BBW,
    pub sma: SMA,
}

impl PopulatesCandles for BBWP {
    fn populate_candles(ts: &mut TimeSeries, args: IndicatorArgs) -> GenericResult<()> {
        let (len, _, sma_len) = args.extract_bbwp_res()?;
        let indicator_type = IndicatorType::BBW(len);
        let mut remove_bbws = false;

        // Populate candles with BBW if not already there
        if !ts.indicators.contains(&indicator_type) {
            let args = IndicatorArgs::BollingerBandArgs(len, 1.0);
            BBW::populate_candles(ts, args)?;
            remove_bbws = true;
        }

        // Calculate BBWP values for TimeSeries
        let mut new_bbwps = Self::calculate_bbwps(ts, &args)?;
        Self::populate_smas(&mut new_bbwps, sma_len)?;

        // Remove bbws again if temporarily added
        if remove_bbws {
            for candle in ts.candles.iter_mut() {
                candle.indicators.remove(&indicator_type);
            }
            ts.indicators.remove(&indicator_type);
        }

        Self::insert_indicators(ts, &new_bbwps, &args)?;

        Ok(())
    }

    fn populate_candles_default(ts: &mut TimeSeries) -> GenericResult<()> {
        let args = IndicatorArgs::BBWPArgs(13, 252, 5);
        Self::populate_candles(ts, args)
    }
}

impl BBWP {
    pub fn calculate_bbwps(
        ts: &mut TimeSeries,
        args: &IndicatorArgs,
    ) -> GenericResult<Vec<Option<BBWP>>> {
        let (len, lookback, _) = args.extract_bbwp_res()?;
        let indicator_type = IndicatorType::BBW(len);

        let bbwps = ts
            .candles
            .iter()
            .enumerate()
            .map(|(i, candle)| {
                if i < len {
                    return None;
                }

                let bbw = candle.indicators.get(&indicator_type)?.as_bbw()?;
                let start = if i >= lookback { i - lookback } else { 0 };
                let segment = &ts.candles[start..i];

                let count = segment
                    .iter()
                    .filter(|s| {
                        s.indicators
                            .get(&indicator_type)
                            .and_then(|old_ind| old_ind.as_bbw())
                            .map_or(false, |old_bbw| old_bbw.value < bbw.value)
                    })
                    .count();

                let bbwp = (count as f64) / (segment.len() as f64);

                Some(BBWP {
                    len,
                    lookback,
                    bbw,
                    value: bbwp,
                    sma: SMA {
                        value: 0.0,
                        len: 0,
                    },
                })
            })
            .collect();

        Ok(bbwps)
    }

    pub fn populate_smas(bbwps: &mut [Option<BBWP>], len: usize) -> GenericResult<()> {
        // Calculate SMA for BBWP values
        for i in len..bbwps.len() {
            let start = i - len + 1;
            let end = i + 1;
            let sum: f64 = bbwps[start..end]
                .iter()
                .filter_map(|bbwp| bbwp.as_ref())
                .map(|bbwp| bbwp.value)
                .sum();

            if let Some(bbwp) = bbwps[i].as_mut() {
                bbwp.sma = SMA {
                    value: sum / (len as f64),
                    len,
                };
            }
        }

        Ok(())
    }

    pub fn insert_indicators(
        ts: &mut TimeSeries,
        bbwps: &[Option<BBWP>],
        args: &IndicatorArgs,
    ) -> GenericResult<()> {
        let (len, lookback, _) = args.extract_bbwp_res()?;
        let indicator_type = IndicatorType::BBWP(len, lookback);

        for (i, candle) in ts.candles.iter_mut().enumerate() {
            let bbwp = Indicator::BBWP(bbwps[i]);
            candle.indicators.insert(indicator_type, bbwp);
        }

        ts.indicators.insert(indicator_type);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::BBWP;
    use crate::{
        indicators::{indicator_type::IndicatorType, populates_candles::PopulatesCandles},
        models::{candle::Candle, timeseries::TimeSeries},
    };
    use std::collections::HashSet;

    #[test]
    fn calculate_bbwp() {
        let arr = [
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
        let candles = Candle::dummy_from_increments(&arr);

        let mut ts = TimeSeries {
            candles,
            ticker: "DUMMY".to_string(),
            interval: crate::models::interval::Interval::Day1,
            indicators: HashSet::new(),
        };

        BBWP::populate_candles_default(&mut ts).unwrap();

        let segment = &ts.candles[ts.candles.len() - 5..];
        let correct_values = [
            0.5238095238095238,
            0.5515873015873016,
            0.5436507936507936,
            0.5079365079365079,
            0.4722222222222222,
        ];

        for (i, val) in correct_values.iter().enumerate() {
            let bbwp = segment[i]
                .get_indicator(&IndicatorType::BBWP(13, 252))
                .unwrap()
                .as_bbwp()
                .unwrap();
            assert_eq!(*val, bbwp.value)
        }
    }

    #[test]
    fn bbwp_not_enough_data() {
        let candles = Candle::dummy_data(2, "positive", 100.0);

        let mut ts = TimeSeries {
            candles,
            ticker: "DUMMY".to_string(),
            interval: crate::models::interval::Interval::Day1,
            indicators: HashSet::new(),
        };

        BBWP::populate_candles_default(&mut ts).unwrap();
        println!("BBWP:{:#?}", ts.candles);

        for candle in ts.candles.iter() {
            let bbwp = candle
                .indicators
                .get(&IndicatorType::BBWP(13, 252))
                .unwrap()
                .as_bbwp();

            assert!(bbwp.is_none())
        }
    }

    #[test]
    fn bbwp_no_candles() {
        let candles = Vec::new();

        let mut ts = TimeSeries {
            candles,
            ticker: "DUMMY".to_string(),
            interval: crate::models::interval::Interval::Day1,
            indicators: HashSet::new(),
        };

        BBWP::populate_candles_default(&mut ts).unwrap();
        println!("BBWP:{:#?}", ts.candles);

        for candle in ts.candles.iter() {
            let bbwp = candle
                .indicators
                .get(&IndicatorType::BBWP(13, 252))
                .unwrap()
                .as_bbwp();

            assert!(bbwp.is_none())
        }
    }
}
