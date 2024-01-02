use anyhow::{Context, anyhow, Result};
use crate::{models::{candle::Candle, timeseries::TimeSeries}, utils::math::sma};
use super::{is_indicator::IsIndicator, indicator_args::IndicatorArgs, populates_candles::PopulatesCandles, indicator_type::IndicatorType, indicator::Indicator};

/// Price Moving Average Ratio
///
/// Indicator based on Caretaker's Tradingview PMAR indicator found
/// [here](https://www.tradingview.com/script/QK6EciNv-Price-Moving-Average-Ratio-Percentile/).
/// It measures...

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PMAR {
    value: f64,
    len: usize
}

impl PopulatesCandles for PMAR {
    fn populate_candles(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_candles_args(ts, Self::default_args())
    }
    
    fn populate_candles_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let len = args.extract_len_res()?;
        let indicator_type = IndicatorType::PMAR(len);

        for i in 0..ts.candles.len() {
            let end = i + 1;
            let pmar = if end < len {
                None
            } else {
                let start = end - len;
                Self::calculate(&ts.candles[start..end])
            };

            ts.candles[i]
                .indicators
                .insert(indicator_type, Indicator::PMAR(pmar));
        }

        ts.indicators.insert(indicator_type);

        Ok(())
    }

    fn populate_last_candle(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_last_candle_args(ts, Self::default_args())
    }

    fn populate_last_candle_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let len = args.extract_len_res()?;
        let end = ts.candles.len();
        let ctx_err = "Failed to get last candle";
        let indicator_type = IndicatorType::PMAR(len);

        if end == 0 {
            return Err(anyhow!("No candle to populate"));
        } else if end < len {
            // Not enough candles to populate
            ts.candles
                .last_mut()
                .context(ctx_err)?
                .indicators
                .insert(indicator_type, Indicator::PMAR(None));
        } else {
            let new_pmar = Self::calculate(&ts.candles[end - len..end]);

            ts.candles
                .last_mut()
                .context(ctx_err)?
                .indicators
                .insert(indicator_type, Indicator::PMAR(new_pmar));
        }

        Ok(())
    }
}

impl IsIndicator for PMAR {
    fn default_args() -> super::indicator_args::IndicatorArgs {
        IndicatorArgs::LengthArg(20)
    }

    fn calculate(segment: &[Candle]) -> Option<Self>
    where
        Self: Sized {
        let segment_len = segment.len();

        if segment_len == 0 { return None }
        if segment_len == 1 { return Some(PMAR::new(1.0, segment_len)) } 

        let values: Vec<f64> = segment.iter().map(|c| c.close).collect();
        let pmar = segment[segment_len-1].close / sma(&values);

        Some(PMAR::new(pmar, segment_len))
    }
}

impl PMAR {
    pub fn new(value: f64, len: usize) -> Self {
        Self { value, len }
    }
}
