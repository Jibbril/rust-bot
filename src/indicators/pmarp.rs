use anyhow::{Result, Context, anyhow};
use crate::{models::{timeseries::TimeSeries, candle::Candle}, utils::math::sma};
use super::{populates_candles::PopulatesCandles, indicator_args::IndicatorArgs, is_indicator::IsIndicator, indicator_type::IndicatorType, indicator::Indicator, pmar::PMAR};


/// # Price Moving Average Ratio Percentile (PMARPP)
///
/// Indicator based on Caretaker's Tradingview PMARPP indicator found
/// [here](https://www.tradingview.com/script/QK6EciNv-Price-Moving-Average-Ratio-Percentile/).
/// It measures the percentile of the current PMARP value as compared to the 
/// last n PMARP values
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
        let (len, lookback) = args.extract_len_lookback_res()?;
        let sma_len = len; // TODO: Change argument type so this is provided
        let indicator_type = IndicatorType::PMARP(len,lookback);

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

            // Not enough candles to populate bbwp sma so return
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
        let (len, lookback) = args.extract_len_lookback_res()?;
        let sma_len = len; // TODO: Change argument type so this is provided
        let ctx_err = "Unable to get last candle";
        let indicator_type = IndicatorType::PMARP(len, lookback);
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

        let mut new_pmarp = Self::calculate(&ts.candles[start..end])
            .context("Unable to calculate PMARP")?;

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
        IndicatorArgs::LengthLookbackArgs(20,350)
    }

    fn calculate(segment: &[Candle]) -> Option<Self> where Self: Sized {
        let args = Self::default_args();
        let mut pmars = Self::get_pmars(segment, args).ok()?;
        let (len,lookback) = args.extract_len_lookback_opt()?;
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
        //
        // let segment_len = segment.len();
        //
        // if segment_len == 0 { return None }
        // if segment_len == 1 { return Some(PMARP::new(1.0, segment_len)) } 
        //
        // let values: Vec<f64> = segment.iter().map(|c| c.close).collect();
        //
        // // TODO: Change to using vwma instead of sma once that indicator is implemented
        // let pmar = segment[segment_len-1].close / sma(&values);
        //
        // Some(PMARPP::new(pmar, segment_len))
    }
}

impl PMARP {
    #[allow(dead_code)]
    pub fn new(value: f64, len: usize, lookback: usize) -> Self {
        Self { 
            value, 
            len, 
            ma: None,
            lookback
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

        let (len,_) = args.extract_len_lookback_res()?;
        let len_arg = IndicatorArgs::LengthArg(len);
        PMAR::populate_candles_args(&mut temp_ts, len_arg)?;

        let ind_type = IndicatorType::PMAR(len);

        let pmars: Vec<Option<PMAR>> = temp_ts
            .candles
            .iter()
            .map(|candle| candle.clone_indicator(&ind_type).unwrap().as_pmar())
            .collect();

        Ok(pmars)
    }
}
