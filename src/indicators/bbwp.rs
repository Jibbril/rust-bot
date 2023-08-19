use super::{bbw::BBW, PopulatesCandles, sma::SMA};
use crate::{models::{generic_result::GenericResult, timeseries::TimeSeries}, indicators::{IndicatorType, Indicator}};

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
    fn populate_candles(ts: &mut TimeSeries, length: usize) -> GenericResult<()> {
        let lookback = 252;
        let indicator_type = IndicatorType::BBW(length);

        // Populate candles with BBWP if not already there
        if !ts.indicators.contains(&indicator_type) {
            BBW::populate_candles(ts, length)?
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
        for i in 5..new_bbwps.len() {
            let sum: f64 = new_bbwps[i - 5..i].iter()
                .filter_map(|bbwp| bbwp.as_ref())
                .map(|bbwp| bbwp.value)
                .sum();

            if let Some(bbwp) = new_bbwps[i].as_mut() {
                bbwp.sma = SMA {
                    value: sum / 5.0,
                    length: 5
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
        Self::populate_candles(ts, 13)
    }
}

impl BBWP {}
