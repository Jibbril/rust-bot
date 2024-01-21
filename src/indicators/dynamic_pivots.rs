use super::{
    indicator::Indicator, indicator_args::IndicatorArgs, indicator_type::IndicatorType,
    is_indicator::IsIndicator, populates_candles::PopulatesCandles,
};
use crate::models::{candle::Candle, timeseries::TimeSeries};
use anyhow::{anyhow, Context, Result};

/// #DynamicPivot indicator
///
/// Inspired by the TradingView indicator [Support and Resistance Levels](https://www.tradingview.com/script/JDFoWQbL-Support-and-Resistance-Levels-with-Breaks-LuxAlgo/) by LuxAlgo
#[derive(Debug, Copy, Clone)]
pub struct DynamicPivots {
    pub len: usize,
    pub high: Option<f64>,
    pub low: Option<f64>,
}

impl PopulatesCandles for DynamicPivots {
    fn populate_candles(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_candles_args(ts, Self::default_args())
    }

    fn populate_candles_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let len = args.len_res()?;
        let indicator_type = IndicatorType::DynamicPivot(len);
        let min_len = 2 * len + 1;

        for i in 0..ts.candles.len() {
            let end = i + 1;
            let pivot = if end < min_len {
                None
            } else {
                Self::calculate(&ts.candles[end - min_len..end])
            };

            // Since the dynamic pivots are populated for the "len/2"-nth
            // (by default 15+1 = 16) candle we need extra handling to select
            // the correct index when populating.
            let j = if end <= len { i } else { i - len };

            ts.candles[j]
                .indicators
                .insert(indicator_type, Indicator::DynamicPivot(pivot));
        }

        ts.indicators.insert(indicator_type);

        Ok(())
    }

    fn populate_last_candle(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_candles_args(ts, Self::default_args())
    }

    fn populate_last_candle_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let len = args.len_res()?;
        let ctx_err = "Unable to get last candle";
        let indicator_type = IndicatorType::DynamicPivot(len);
        let candle_len = ts.candles.len();
        let min_len = 2 * len + 1;
        if candle_len == 0 {
            return Err(anyhow!("No candle to populate"));
        }

        if candle_len < len {
            // Not enough candles to populate
            ts.candles
                .last_mut()
                .context(ctx_err)?
                .indicators
                .insert(indicator_type, Indicator::DynamicPivot(None));

            return Ok(());
        }

        let new_pivot = Self::calculate(&ts.candles[candle_len - min_len..candle_len]);

        ts.candles
            .get_mut(candle_len - (len + 1))
            .context(ctx_err)?
            .indicators
            .insert(indicator_type, Indicator::DynamicPivot(new_pivot));

        Ok(())
    }
}

impl IsIndicator for DynamicPivots {
    fn default_args() -> IndicatorArgs {
        IndicatorArgs::LengthArg(15)
    }

    fn calculate(segment: &[Candle]) -> Option<Self>
    where
        Self: Sized,
    {
        // Unable to calculate for even number of Candles
        if segment.len() % 2 == 0 {
            return None;
        };

        let len = (segment.len() - 1) / 2;
        let candle = &segment[len];

        let is_high = segment.iter().all(|c| c.high <= candle.high);
        let is_low = segment.iter().all(|c| c.low >= candle.low);

        let mut pivots = DynamicPivots::new_empty(len);
        let prev = segment[len - 1]
            .indicators
            .get(&IndicatorType::DynamicPivot(len))
            .and_then(|p| p.as_dynamic_pivots());

        pivots.high = if is_high {
            Some(candle.high)
        } else {
            prev.and_then(|p| p.high)
        };

        pivots.low = if is_low {
            Some(candle.low)
        } else {
            prev.and_then(|p| p.low)
        };

        Some(pivots)
    }

    fn calculate_args(_segment: &[Candle], _args: &IndicatorArgs) -> Option<Self> 
    where 
        Self: Sized {
        todo!()
    }
}

impl DynamicPivots {
    pub fn new_empty(len: usize) -> Self {
        Self {
            len,
            high: None,
            low: None,
        }
    }
}
