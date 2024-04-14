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
        let (k_len, k_smoothing, d_len) = args.stochastic_opt()?;
        
        if segment.len() < k_len + (k_smoothing - 1) + (d_len - 1) { return None; }

        let mut ks: Vec<f64> = segment.windows(k_len)
            .rev()
            .take(k_smoothing + d_len)
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
            .take(d_len)
            .sum::<f64>() / d_len as f64;

        Some(Self { k: *ks.first()?, d })
    }
} 

impl Stochastic {
    fn needed_candles(k_len: usize, k_smoothing: usize, d_smoothing: usize) -> usize {
        k_len + k_smoothing + d_smoothing - 2
    }
}
