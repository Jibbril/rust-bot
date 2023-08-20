use super::{bbw::BBW, sma::SMA, indicator::Indicator, indicator_type::IndicatorType, populates_candles::PopulatesCandles, indicator_args::IndicatorArgs};
use crate::models::{generic_result::GenericResult, timeseries::TimeSeries};

/// Bollinger Band Width Percentile
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct BBWP {
    #[allow(dead_code)] // TODO: Remove once used
    pub value: f64,
    pub length: usize,
    pub lookback: usize,
    pub bbw: BBW,
    pub sma: SMA
}

impl PopulatesCandles for BBWP {
    fn populate_candles(ts: &mut TimeSeries, args: IndicatorArgs) -> GenericResult<()> {
        let (length,lookback, sma_length) = args.extract_bbwp_args_res()?;
        let indicator_type = IndicatorType::BBW(length);

        // Populate candles with BBWP if not already there
        if !ts.indicators.contains(&indicator_type) {
            let args = IndicatorArgs::BollingerBandArgs(length, 1.0);
            BBW::populate_candles(ts, args)?
        }
        
        // Calculate BBWP values for TimeSeries
        let mut new_bbwps:Vec<Option<BBWP>> = ts.candles.iter()
            .enumerate()
            .map(|(i,candle)| {
                if i < length { return None }

                let bbw = candle.indicators.get(&indicator_type)?.as_bbw()?;

                let start = if i >= lookback { i - lookback } else { 0 };
                let segment = &ts.candles[start..i];
                
                let count = segment.iter()
                    .filter(|s| {
                        s.indicators.get(&indicator_type)
                            .and_then(|old_ind| old_ind.as_bbw())
                            .map_or(false, |old_bbw| old_bbw.value < bbw.value)
                    })
                    .count();

                let bbwp = (count as f64) / (segment.len() as f64);

                Some(BBWP {
                    length,
                    lookback,
                    bbw,
                    value: bbwp,
                    sma: SMA { value: 0.0, length: 0 }
                })
            })
            .collect();

        // Calculate 5-period SMA for BBWP values
        for i in sma_length..new_bbwps.len() {
            let sum: f64 = new_bbwps[i - sma_length..i].iter()
                .filter_map(|bbwp| bbwp.as_ref())
                .map(|bbwp| bbwp.value)
                .sum();

            if let Some(bbwp) = new_bbwps[i].as_mut() {
                bbwp.sma = SMA {
                    value: sum / (sma_length as f64),
                    length: sma_length
                };
            }
        } 

        let indicator_type = IndicatorType::BBWP(length);

        for (i, candle) in ts.candles.iter_mut().enumerate() {
            let new_bbwp = Indicator::BBWP(new_bbwps[i]);

            candle.indicators.insert(indicator_type, new_bbwp);
        }

        Ok(())
    }

    fn populate_candles_default(ts: &mut TimeSeries) -> GenericResult<()> {
        let args = IndicatorArgs::BBWPArgs(13, 252, 5);
        Self::populate_candles(ts, args)
    }
}

impl BBWP {}
