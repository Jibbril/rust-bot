use super::{
    bbw::BBW, indicator::Indicator, indicator_args::IndicatorArgs, indicator_type::IndicatorType,
    populates_candles::PopulatesCandles, sma::SMA,
};
use crate::models::{generic_result::GenericResult, timeseries::TimeSeries};

/// Bollinger Band Width Percentile
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct BBWP {
    pub value: f64,
    pub length: usize,
    pub lookback: usize,
    pub bbw: BBW,
    pub sma: SMA,
}

impl PopulatesCandles for BBWP {
    fn populate_candles(ts: &mut TimeSeries, args: IndicatorArgs) -> GenericResult<()> {
        let (length,_,sma_length) = args.extract_bbwp_args_res()?;
        let indicator_type = IndicatorType::BBW(length);
        let mut remove_bbws = false;

        // Populate candles with BBW if not already there
        if !ts.indicators.contains(&indicator_type) {
            let args = IndicatorArgs::BollingerBandArgs(length, 1.0);
            BBW::populate_candles(ts, args)?;
            remove_bbws = true;
        }

        // Calculate BBWP values for TimeSeries
        let mut new_bbwps = Self::calculate_bbwps(ts, &args)?;
        Self::populate_smas(&mut new_bbwps, sma_length)?;

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
    pub fn calculate_bbwps(ts: &mut TimeSeries, args: &IndicatorArgs) -> GenericResult<Vec<Option<BBWP>>> {
        let (length, lookback, _) = args.extract_bbwp_args_res()?;
        let indicator_type = IndicatorType::BBW(length);

        let bbwps = ts.candles
            .iter()
            .enumerate()
            .map(|(i, candle)| {
                if i < length {
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
                    length,
                    lookback,
                    bbw,
                    value: bbwp,
                    sma: SMA {
                        value: 0.0,
                        length: 0,
                    },
                })
            })
            .collect();

        Ok(bbwps)
    }

    pub fn populate_smas(bbwps: &mut [Option<BBWP>], length: usize) -> GenericResult<()> {
        // Calculate SMA for BBWP values
        for i in length..bbwps.len() {
            let start = i - length + 1;
            let end = i + 1;
            let sum: f64 = bbwps[start..end]
                .iter()
                .filter_map(|bbwp| bbwp.as_ref())
                .map(|bbwp| bbwp.value)
                .sum();

            if let Some(bbwp) = bbwps[i].as_mut() {
                bbwp.sma = SMA {
                    value: sum / (length as f64),
                    length,
                };
            }
        }

        Ok(())
    }

    pub fn insert_indicators(ts: &mut TimeSeries, bbwps: &[Option<BBWP>], args: &IndicatorArgs) -> GenericResult<()> {
        let (length, lookback, _) = args.extract_bbwp_args_res()?;
        let indicator_type = IndicatorType::BBWP(length, lookback);

        for (i, candle) in ts.candles.iter_mut().enumerate() {
            let bbwp = Indicator::BBWP(bbwps[i]);
            candle.indicators.insert(indicator_type, bbwp);
        }

        ts.indicators.insert(indicator_type);

        Ok(())
    }
}
