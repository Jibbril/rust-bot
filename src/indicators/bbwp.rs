use super::{
    bbw::BBW, indicator::Indicator, indicator_args::IndicatorArgs, indicator_type::IndicatorType,
    is_indicator::IsIndicator, populates_candles::PopulatesCandles, sma::SMA,
};
use crate::{
    models::{candle::Candle, timeseries::TimeSeries},
    utils::math::sma,
};
use anyhow::{anyhow, Context, Result};

/// # Bollinger Band Width Percentile
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
    fn populate_candles(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_candles_args(ts, Self::default_args())
    }

    fn populate_candles_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let (len, lookback, sma_len) = args.bbwp_res()?;
        let indicator_type = IndicatorType::BBWP(len, lookback);

        for i in 0..ts.candles.len() {
            let end = i + 1;
            let bbwp = if end < len {
                None
            } else {
                let start = if end > lookback + len {
                    end - lookback - len
                } else {
                    0
                };

                Self::calculate_args(&ts.candles[start..end], &args)
            };

            ts.candles[i]
                .indicators
                .insert(indicator_type, Indicator::BBWP(bbwp));

            // Not enough candles to populate bbwp sma so return
            if end < len + sma_len {
                continue;
            }

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

    fn populate_last_candle(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_last_candle_args(ts, Self::default_args())
    }

    fn populate_last_candle_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let (len, lookback, sma_len) = args.bbwp_res()?;
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

        let start = if end > lookback + len {
            end - lookback - len
        } else {
            0
        };

        let mut new_bbwp = Self::calculate_args(&ts.candles[start..end], &args)
            .context("Unable to calculate BBWP")?;

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
    fn calculate(segment: &[Candle]) -> Option<Self> where Self: Sized, {
        Self::calculate_bbwp(segment, &Self::default_args())
    }

    fn calculate_args(segment: &[Candle], args: &IndicatorArgs) -> Option<Self> 
    where 
        Self: Sized {
        Self::calculate_bbwp(segment, args)
    }
}

impl BBWP {
    fn calculate_bbwp(segment: &[Candle], args: &IndicatorArgs) -> Option<Self> {
        let bbws = Self::get_bbws(segment, args).ok()?;
        let (len, lookback, _) = args.bbwp_opt()?;
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

    fn bbwp_sma(segment: &[Candle], indicator_type: &IndicatorType) -> Option<f64> {
        let values: Vec<f64> = segment
            .iter()
            .filter_map(|c| c.clone_indicator(indicator_type).ok()?.as_bbwp())
            .map(|bbwp| bbwp.value)
            .collect();

        (values.len() == segment.len()).then_some(sma(&values))
    }

    fn get_bbws(segment: &[Candle], args: &IndicatorArgs) -> Result<Vec<Option<BBW>>> {
        // TODO: Refactor populates candles trait to include methods accepting
        // just candles instead of only TimeSeries to not need temporary
        // TimeSeries below.

        let temp_segment = segment.to_vec();
        let mut temp_ts = TimeSeries::dummy();
        temp_ts.set_candles(&temp_segment);

        let (len, _, _) = args.bbwp_res()?;
        let bbw_args = IndicatorArgs::BollingerBandArgs(len, BBW_STD_N);
        BBW::populate_candles_args(&mut temp_ts, bbw_args)?;

        let ind_type = IndicatorType::BBW(len);

        let bbws: Vec<Option<BBW>> = temp_ts
            .candles
            .iter()
            .map(|candle| candle.clone_indicator(&ind_type).unwrap().as_bbw())
            .collect();

        Ok(bbws)
    }
}

#[cfg(test)]
mod tests {
    use super::BBWP;
    use crate::{
        indicators::{
            indicator_type::IndicatorType, is_indicator::IsIndicator,
            populates_candles::PopulatesCandles,
        },
        models::{candle::Candle, interval::Interval, timeseries::TimeSeries},
        utils::data::dummy_data::PRICE_CHANGES,
    };

    const FINAL_VALUES: &[f64] = &[
        0.5238095238095238,
        0.5515873015873016,
        0.5436507936507936,
        0.5079365079365079,
        0.4722222222222222,
    ];

    #[test]
    fn bbwp_calculate() {
        let candles = Candle::dummy_from_increments(&PRICE_CHANGES);

        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);

        let _ = BBWP::populate_candles(&mut ts);

        let segment = &ts.candles[ts.candles.len() - 5..];

        let (len, lookback, _) = BBWP::default_args().bbwp_opt().unwrap();
        for (i, val) in FINAL_VALUES.iter().enumerate() {
            let bbwp = segment[i]
                .clone_indicator(&IndicatorType::BBWP(len, lookback))
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
    fn bbwp_no_candles_args() {
        let candles = Vec::new();
        let args = BBWP::default_args();
        let sma = BBWP::calculate_args(&candles, &args);
        assert!(sma.is_none());
    }

    #[test]
    fn bbwp_populate_candles() {
        let candles = Candle::dummy_from_increments(&PRICE_CHANGES);
        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);

        let _ = BBWP::populate_candles(&mut ts);

        let (len, lookback, _sma_len) = BBWP::default_args().bbwp_opt().unwrap();
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

        let segment = &ts.candles[ts.candles.len() - 5..];

        let (len, lookback, _) = BBWP::default_args().bbwp_opt().unwrap();
        for (i, val) in FINAL_VALUES.iter().enumerate() {
            let bbwp = segment[i]
                .clone_indicator(&IndicatorType::BBWP(len, lookback))
                .unwrap()
                .as_bbwp()
                .unwrap();
            assert_eq!(*val, bbwp.value)
        }
    }

    #[test]
    fn bbwp_populate_last_candle() {
        let mut candles = Candle::dummy_from_increments(&PRICE_CHANGES);
        let candle = candles.pop().unwrap();

        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);
        let _ = BBWP::populate_candles(&mut ts);
        let _ = ts.add_candle(candle);

        let (len, lookback, _sma_len) = BBWP::default_args().bbwp_opt().unwrap();
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
