use anyhow::{Result, anyhow, Context};
use crate::{indicators::{
    is_indicator::IsIndicator,
    indicator_args::IndicatorArgs,
    populates_candles::PopulatesCandles,
    indicator_type::IndicatorType,
    indicator::Indicator,
}, models::{candle::Candle, timeseries::TimeSeries}};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Stochastic {
    pub k: f64,
    pub d: f64,
}

impl Stochastic {
    #[allow(dead_code)]
    pub fn krown_args() -> IndicatorArgs {
        IndicatorArgs::StochasticArgs(14, 3, 6)
    }
}

impl PopulatesCandles for Stochastic {
    fn populate_candles(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_candles_args(ts, Self::default_args())
    }

    fn populate_candles_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let (k_len, k_smoothing, d_smoothing) = args.stochastic_res()?;
        let indicator_type = IndicatorType::Stochastic(k_len, k_smoothing, d_smoothing);
        let needed_candles = Self::needed_candles(k_len, k_smoothing, d_smoothing);

        for i in 0..ts.candles.len() {
            let end = i + 1;
            let stoch = if end < needed_candles {
                None
            } else {
                Self::calculate_args(&ts.candles[end - needed_candles..end], &args)
            };

            ts.candles[i]
                .indicators
                .insert(indicator_type, Indicator::Stochastic(stoch));
        }

        ts.indicators.insert(indicator_type);

        Ok(())
    }

    fn populate_last_candle(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_last_candle_args(ts, Self::default_args())
    }

    fn populate_last_candle_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let (k_len, k_smoothing, d_smoothing) = args.stochastic_res()?;
        let end = ts.candles.len();
        let ctx_err = "Failed to get last candle";
        let indicator_type = IndicatorType::Stochastic(k_len, k_smoothing, d_smoothing);
        let needed_candles = Self::needed_candles(k_len, k_smoothing, d_smoothing);

        if end == 0 {
            return Err(anyhow!("No candle to populate"));
        } else if end < needed_candles {
            // Not enough candles to populate
            ts.candles
                .last_mut()
                .context(ctx_err)?
                .indicators
                .insert(indicator_type, Indicator::Stochastic(None));
        } else {
            let new_stoch = Self::calculate_args(&ts.candles[end - needed_candles..end], &args);

            ts.candles
                .last_mut()
                .context(ctx_err)?
                .indicators
                .insert(indicator_type, Indicator::Stochastic(new_stoch));
        }

        Ok(())
    }
}

impl IsIndicator for Stochastic {
    fn default_args() -> IndicatorArgs {
        IndicatorArgs::StochasticArgs(14, 1, 3)
    }

    fn calculate(segment: &[Candle]) -> Option<Self>
    where
        Self: Sized {
        Self::calculate_args(segment, &Self::default_args())
    }

    fn calculate_args(segment: &[Candle], args: &IndicatorArgs) -> Option<Self>
    where
        Self: Sized {
        let (k_len, k_smoothing, d_smoothing) = args.stochastic_opt()?;
        let needed_candles = Self::needed_candles(k_len, k_smoothing, d_smoothing);
        
        if segment.len() < needed_candles { return None; }

        let mut ks: Vec<f64> = segment.windows(k_len)
            .rev()
            .take(k_smoothing + d_smoothing)
            .map(|s| {
                let close = s.last()
                    .expect("Expected candle in slice.")
                    .close;

                let (low,high) = s.iter().fold((f64::MAX,f64::MIN), |(min,max), c| {
                    let mut low = min;
                    let mut high = max;

                    if c.low < low {
                        low = c.low
                    }

                    if c.high > high {
                        high = c.high
                    }

                    (low,high) 
                });

                if high == low { 
                    0.5 
                } else { 
                    (close - low) / (high - low)
                }
            })
            .collect();

        if k_smoothing > 1 {
            ks = ks.windows(k_smoothing)
                .map(|s| s.iter().sum::<f64>() / k_smoothing as f64)
                .collect();
        }

        let d = ks.iter()
            .take(d_smoothing)
            .sum::<f64>() / d_smoothing as f64;

        Some(Self { k: *ks.first()?, d })
    }
} 

impl Stochastic {
    fn needed_candles(k_len: usize, k_smoothing: usize, d_smoothing: usize) -> usize {
        k_len + k_smoothing + d_smoothing - 2
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        indicators::{
            indicator_type::IndicatorType, 
            populates_candles::PopulatesCandles, stochastic::Stochastic
        },
        models::{candle::Candle, interval::Interval, timeseries_builder::TimeSeriesBuilder},
        utils::data::dummy_data::PRICE_CHANGES,
    };

    const FINAL_VALUES: &[(f64,f64)] = &[
        (0.6542362500479966,0.3932293973692202),
        (0.72863012488384,0.5269333770227321),
        (0.6017964844106882,0.6002101651769305),
        (0.37588233499204177,0.5900912298007457),
        (0.39696803104797135,0.5685657964420793),
    ];

    #[test]
    fn stochastic_calculate() {
        let candles = Candle::dyn_dummy_from_increments(&PRICE_CHANGES);

        let mut ts = TimeSeriesBuilder::new()
            .symbol("DUMMY".to_string())
            .interval(Interval::Day1)
            .candles(candles)
            .build();

        let args = Stochastic::krown_args();
        let _ = Stochastic::populate_candles_args(&mut ts, args);

        let segment = &ts.candles[ts.candles.len() - 5..];

        let (k_len, k_smoothing, d_smoothing) = args.stochastic_opt().unwrap();
        for (i, (k_val,d_val)) in FINAL_VALUES.iter().enumerate() {
            let stochastic = segment[i]
                .clone_indicator(&IndicatorType::Stochastic(k_len, k_smoothing, d_smoothing))
                .unwrap()
                .as_stochastic()
                .unwrap();
            assert_eq!(*k_val, stochastic.k);
            assert_eq!(*d_val, stochastic.d);
        }
    }

    // #[test]
    // fn stochastic_no_candles() {
    //     let candles = Vec::new();
    //     let sma = BBWP::calculate(&candles);
    //     assert!(sma.is_none());
    // }
    //
    // #[test]
    // fn stochastic_no_candles_args() {
    //     let candles = Vec::new();
    //     let args = BBWP::default_args();
    //     let sma = BBWP::calculate_args(&candles, &args);
    //     assert!(sma.is_none());
    // }
    //
    // #[test]
    // fn stochastic_populate_candles() {
    //     let candles = Candle::dummy_from_increments(&PRICE_CHANGES);
    //     let mut ts = TimeSeriesBuilder::new()
    //         .symbol("DUMMY".to_string())
    //         .interval(Interval::Day1)
    //         .candles(candles)
    //         .build();
    //
    //     let _ = BBWP::populate_candles(&mut ts);
    //
    //     let (len, lookback, _sma_len) = BBWP::default_args().bbwp_opt().unwrap();
    //     let indicator_type = IndicatorType::BBWP(len, lookback);
    //
    //     for (i, candle) in ts.candles.iter().enumerate() {
    //         let indicator = candle.indicators.get(&indicator_type).unwrap();
    //         let bbwp = indicator.as_bbwp();
    //         if i < len - 1 {
    //             assert!(bbwp.is_none());
    //         } else {
    //             assert!(bbwp.is_some());
    //         }
    //     }
    //
    //     let segment = &ts.candles[ts.candles.len() - 5..];
    //
    //     let (len, lookback, _) = BBWP::default_args().bbwp_opt().unwrap();
    //     for (i, val) in FINAL_VALUES.iter().enumerate() {
    //         let bbwp = segment[i]
    //             .clone_indicator(&IndicatorType::BBWP(len, lookback))
    //             .unwrap()
    //             .as_bbwp()
    //             .unwrap();
    //         assert_eq!(*val, bbwp.value)
    //     }
    // }
    //
    // #[test]
    // fn stochastic_populate_last_candle() {
    //     let mut candles = Candle::dummy_from_increments(&PRICE_CHANGES);
    //     let candle = candles.pop().unwrap();
    //
    //     let mut ts = TimeSeriesBuilder::new()
    //         .symbol("DUMMY".to_string())
    //         .interval(Interval::Day1)
    //         .candles(candles)
    //         .build();
    //     let _ = BBWP::populate_candles(&mut ts);
    //     let _ = ts.add_candle(&candle);
    //
    //     let (len, lookback, _sma_len) = BBWP::default_args().bbwp_opt().unwrap();
    //     let indicator_type = IndicatorType::BBWP(len, lookback);
    //
    //     for (i, candle) in ts.candles.iter().enumerate() {
    //         let indicator = candle.indicators.get(&indicator_type).unwrap();
    //         let bbwp = indicator.as_bbwp();
    //         if i < len - 1 {
    //             assert!(bbwp.is_none());
    //         } else {
    //             assert!(bbwp.is_some());
    //         }
    //     }
    //
    //     let last_candle = ts.candles.last().unwrap();
    //     let last_bbwp = last_candle
    //         .indicators
    //         .get(&indicator_type)
    //         .unwrap()
    //         .as_bbwp()
    //         .unwrap();
    //
    //     assert_eq!(&last_bbwp.value, FINAL_VALUES.last().unwrap());
    // }
}
