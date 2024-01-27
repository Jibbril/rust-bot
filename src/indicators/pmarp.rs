use super::{
    indicator::Indicator, indicator_args::IndicatorArgs, indicator_type::IndicatorType,
    is_indicator::IsIndicator, pmar::PMAR, populates_candles::PopulatesCandles,
};
use crate::{
    models::{candle::Candle, timeseries::TimeSeries, ma_type::MAType},
    utils::math::sma,
};
use anyhow::{anyhow, Context, Result};

/// # Price Moving Average Ratio Percentile (PMARPP)
///
/// Indicator based on Caretaker's Tradingview PMARPP indicator found
/// [here](https://www.tradingview.com/script/QK6EciNv-Price-Moving-Average-Ratio-Percentile/).
/// It measures the percentile of the current PMARP value as compared to the
/// last n PMARP values.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PMARP {
    pub value: f64,
    pub len: usize,
    pub ma: Option<f64>,
    pub lookback: usize,
}

impl PopulatesCandles for PMARP {
    fn populate_candles(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_candles_args(ts, Self::default_args())
    }

    fn populate_candles_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let (len, lookback, ma_type) = args.pmarp_res()?;
        let sma_len = len; // TODO: Change argument type so this is provided
        let indicator_type = IndicatorType::PMARP(len, lookback, ma_type);

        for i in 0..ts.candles.len() {
            let end = i + 1;
            let pmarp = if end < len {
                None
            } else {
                let start = if end > lookback + len {
                    end - lookback - len
                } else {
                    0
                };

                Self::calculate(&ts.candles[start..end])
            };

            ts.candles[i]
                .indicators
                .insert(indicator_type, Indicator::PMARP(pmarp));

            // Not enough candles to populate pmarp sma so return
            if end < len + sma_len {
                continue;
            }

            let mut pmarp = pmarp.context("Unable to calculate PMARP")?;
            let sma_segment = &ts.candles[end - sma_len..end];
            let sma = Self::pmarp_sma(sma_segment, &indicator_type);

            if let Some(sma) = sma {
                pmarp.ma = Some(sma);

                ts.candles[i]
                    .indicators
                    .insert(indicator_type, Indicator::PMARP(Some(pmarp)));
            }
        }

        ts.indicators.insert(indicator_type);

        Ok(())
    }

    fn populate_last_candle(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_last_candle_args(ts, Self::default_args())
    }

    fn populate_last_candle_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let (len, lookback, ma_type) = args.pmarp_res()?;
        let sma_len = len; // TODO: Change argument type so this is provided
        let ctx_err = "Unable to get last candle";
        let indicator_type = IndicatorType::PMARP(len, lookback, ma_type);
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
                .insert(indicator_type, Indicator::PMARP(None));

            return Ok(());
        }

        let start = if end > lookback + len {
            end - lookback - len
        } else {
            0
        };

        let mut new_pmarp =
            Self::calculate(&ts.candles[start..end]).context("Unable to calculate PMARP")?;

        ts.candles
            .last_mut()
            .context(ctx_err)?
            .indicators
            .insert(indicator_type, Indicator::PMARP(Some(new_pmarp)));

        // Not enough candles to populate pmarp sma so return
        if end < len + sma_len {
            return Ok(());
        }

        let sma_segment = &ts.candles[end - sma_len..end];
        let sma = Self::pmarp_sma(sma_segment, &indicator_type);

        if let Some(sma) = sma {
            new_pmarp.ma = Some(sma);

            ts.candles
                .last_mut()
                .context(ctx_err)?
                .indicators
                .insert(indicator_type, Indicator::PMARP(Some(new_pmarp)));
        }

        Ok(())
    }
}

impl IsIndicator for PMARP {
    fn default_args() -> IndicatorArgs {
        IndicatorArgs::PMARPArgs(20, 350, MAType::VWMA)
    }

    fn calculate(segment: &[Candle]) -> Option<Self>
    where
        Self: Sized,
    {
        let args = Self::default_args();
        let mut pmars = Self::get_pmars(segment, args).ok()?;
        let (len, lookback,_) = args.pmarp_opt()?;
        let new_pmar = pmars.pop()??;

        let count = pmars
            .iter()
            .filter(|&old_pmar| old_pmar.map_or(false, |old_pmar| old_pmar.value < new_pmar.value))
            .count();

        let pmarp = (count as f64) / (lookback as f64);

        Some(PMARP {
            len,
            lookback,
            value: pmarp,
            ma: None,
        })
    }

    fn calculate_args(_segment: &[Candle], _args: &IndicatorArgs) -> Option<Self> 
    where 
        Self: Sized {
        todo!()
    }
}

impl PMARP {
    #[allow(dead_code)]
    pub fn new(value: f64, len: usize, lookback: usize) -> Self {
        Self {
            value,
            len,
            ma: None,
            lookback,
        }
    }

    fn pmarp_sma(segment: &[Candle], indicator_type: &IndicatorType) -> Option<f64> {
        let values: Vec<f64> = segment
            .iter()
            .filter_map(|c| c.clone_indicator(indicator_type).ok()?.as_pmarp())
            .map(|pmarp| pmarp.value)
            .collect();

        (values.len() == segment.len()).then_some(sma(&values))
    }

    fn get_pmars(segment: &[Candle], args: IndicatorArgs) -> Result<Vec<Option<PMAR>>> {
        // TODO: Refactor populates candles trait to include methods accepting
        // just candles instead of only TimeSeries to not need temporary
        // TimeSeries below.

        let temp_segment = segment.to_vec();
        let mut temp_ts = TimeSeries::dummy();
        temp_ts.set_candles(&temp_segment);

        let (len, _, ma_type) = args.pmarp_res()?;
        let pmar_args = IndicatorArgs::PMARArgs(len, ma_type);
        PMAR::populate_candles_args(&mut temp_ts, pmar_args)?;

        let ind_type = IndicatorType::PMAR(len, ma_type);

        let pmars: Vec<Option<PMAR>> = temp_ts
            .candles
            .iter()
            .map(|candle| candle.clone_indicator(&ind_type).unwrap().as_pmar())
            .collect();

        Ok(pmars)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        indicators::{
            indicator_type::IndicatorType, is_indicator::IsIndicator, pmarp::PMARP,
            populates_candles::PopulatesCandles,
        },
        models::{candle::Candle, interval::Interval, timeseries::TimeSeries},
        utils::data::dummy_data::PRICE_CHANGES,
    };

    const FINAL_VALUE: f64 = 0.3314285714285714;

    #[test]
    fn calculate_pmarp() {
        let candles = Candle::dummy_from_increments(&PRICE_CHANGES);
        let pmarp = PMARP::calculate(&candles);
        assert!(pmarp.is_some());

        let pmarp = pmarp.unwrap();
        assert_eq!(pmarp.value, 0.3314285714285714);
    }

    #[test]
    fn pmarp_no_candles() {
        let candles: Vec<Candle> = Vec::new();
        let pmarp = PMARP::calculate(&candles);
        assert!(pmarp.is_none());
    }

    #[test]
    fn calculate_pmarp_single() {
        let candles = Candle::dummy_from_increments(&PRICE_CHANGES);
        let sma = PMARP::calculate(&candles);
        assert!(sma.is_some());

        let sma = sma.unwrap();
        assert_eq!(sma.value, 0.3314285714285714);
    }

    #[test]
    fn pmarp_populate_candles() {
        let candles = Candle::dummy_from_increments(&PRICE_CHANGES);
        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);

        let _ = PMARP::populate_candles(&mut ts);

        let (len, lookback, ma_type) = PMARP::default_args().pmarp_opt().unwrap();
        let indicator_type = IndicatorType::PMARP(len, lookback, ma_type);

        for (i, candle) in ts.candles.iter().enumerate() {
            let indicator = candle.indicators.get(&indicator_type).unwrap();
            let pmarp = indicator.as_pmarp();
            if i < len - 1 {
                assert!(pmarp.is_none());
            } else {
                assert!(pmarp.is_some());
            }
        }

        let last_candle = ts.candles.last().unwrap();
        let last_pmarp = last_candle
            .indicators
            .get(&indicator_type)
            .unwrap()
            .as_pmarp()
            .unwrap();

        assert_eq!(last_pmarp.value, FINAL_VALUE);
    }

    #[test]
    fn pmarp_populate_last_candle() {
        let mut candles = Candle::dummy_from_increments(&PRICE_CHANGES);
        let candle = candles.pop().unwrap();

        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);
        let _ = PMARP::populate_candles(&mut ts);
        let _ = ts.add_candle(candle);

        let (len, lookback, ma_type) = PMARP::default_args().pmarp_opt().unwrap();
        let indicator_type = IndicatorType::PMARP(len, lookback, ma_type);

        for (i, candle) in ts.candles.iter().enumerate() {
            let indicator = candle.indicators.get(&indicator_type).unwrap();
            let pmarp = indicator.as_pmarp();
            if i < len - 1 {
                assert!(pmarp.is_none());
            } else {
                assert!(pmarp.is_some());
            }
        }

        let last_candle = ts.candles.last().unwrap();
        let last_pmarp = last_candle
            .indicators
            .get(&indicator_type)
            .unwrap()
            .as_pmarp()
            .unwrap();

        assert_eq!(last_pmarp.value, FINAL_VALUE);
    }
}
