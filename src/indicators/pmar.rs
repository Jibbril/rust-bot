use super::{
    indicator::Indicator, indicator_args::IndicatorArgs, indicator_type::IndicatorType,
    is_indicator::IsIndicator, populates_candles::PopulatesCandles,
};
use crate::{
    models::{candle::Candle, timeseries::TimeSeries, ma_type::MAType},
    utils::math::{sma, vwma},
};
use anyhow::{anyhow, Context, Result};

/// # Price Moving Average Ratio (PMAR)
///
/// Indicator based on Caretaker's Tradingview PMAR indicator found
/// [here](https://www.tradingview.com/script/QK6EciNv-Price-Moving-Average-Ratio-Percentile/).
/// It measures the ratio of the current closing price to a moving average of
/// the n most recent candle closes. A high PMAR indicates that the current
/// closing price is significantly higher than the previous n candles have been.
///
/// ## Examples
/// For an SMA-based PMAR with length 3 the following is true
///
/// Current close = 120
/// previous 3 closes = [99,100,101]
/// PMAR = 120 / ((99 + 100 + 101) / 3) = 1.2
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PMAR {
    pub value: f64,
    pub len: usize,
    pub ma: Option<f64>,
}

impl PopulatesCandles for PMAR {
    fn populate_candles(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_candles_args(ts, Self::default_args())
    }

    fn populate_candles_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let (len,ma_type) = args.pmar_res()?;
        let indicator_type = IndicatorType::PMAR(len,ma_type);

        for i in 0..ts.candles.len() {
            let end = i + 1;
            let pmar = if end < len {
                None
            } else {
                let start = end - len;
                Self::calculate_args(&ts.candles[start..end], &args)
            };

            ts.candles[i]
                .indicators
                .insert(indicator_type, Indicator::PMAR(pmar));

            // Not enough candles to populate pmar sma
            if end < len {
                continue;
            }

            let mut pmar = pmar.context("Unable to calculate PMAR")?;
            let sma = Self::pmar_sma(&ts.candles[end - len..end], &indicator_type);

            if let Some(sma) = sma {
                pmar.ma = Some(sma);

                ts.candles[i]
                    .indicators
                    .insert(indicator_type, Indicator::PMAR(Some(pmar)));
            }
        }

        ts.indicators.insert(indicator_type);

        Ok(())
    }

    fn populate_last_candle(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_last_candle_args(ts, Self::default_args())
    }

    fn populate_last_candle_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let (len,ma_type) = args.pmar_res()?;
        let end = ts.candles.len();
        let ctx_err = "Failed to get last candle";
        let indicator_type = IndicatorType::PMAR(len,ma_type);

        if end == 0 {
            return Err(anyhow!("No candle to populate"));
        }

        if end < len {
            // Not enough candles to populate
            ts.candles
                .last_mut()
                .context(ctx_err)?
                .indicators
                .insert(indicator_type, Indicator::PMAR(None));

            return Ok(());
        }

        let new_pmar = Self::calculate_args(&ts.candles[end - len..end], &args);

        // Insert pmar without moving average
        ts.candles
            .last_mut()
            .context(ctx_err)?
            .indicators
            .insert(indicator_type, Indicator::PMAR(new_pmar));

        // Attempt to calculate and insert pmar moving average
        let sma_segment = &ts.candles[end - len..end];
        let sma = Self::pmar_sma(sma_segment, &indicator_type);

        ts.candles
            .last_mut()
            .and_then(|candle| candle.indicators.get_mut(&indicator_type))
            .and_then(|indicator| match indicator {
                Indicator::PMAR(pmar) => pmar.as_mut(),
                _ => None,
            })
            .map(|pmar| pmar.ma = sma);

        Ok(())
    }
}

impl IsIndicator for PMAR {
    fn default_args() -> super::indicator_args::IndicatorArgs {
        IndicatorArgs::PMARArgs(20, MAType::SMA)
    }

    /// Segment should be the same length as the EMA wanted.
    fn calculate(segment: &[Candle]) -> Option<Self>
    where
        Self: Sized,
    {
        let (_,ma_type) = Self::default_args().pmar_opt()?;
        let args = IndicatorArgs::PMARArgs(segment.len(), ma_type);

        Self::calculate_pmar(segment, &args)
    }

    fn calculate_args(segment: &[Candle], args: &IndicatorArgs) -> Option<Self> 
    where 
        Self: Sized {
        let (len,_) = args.pmar_opt()?;
        let candle_len = segment.len();

        if candle_len < len {
            return None;
        }

        Self::calculate_pmar(&segment[candle_len-len..candle_len], args)
    }
}

impl PMAR {
    fn calculate_pmar(segment: &[Candle], args: &IndicatorArgs) -> Option<Self> {
        let (len,ma_type) = args.pmar_opt()?;
        let segment_len = segment.len();

        if segment_len == 0 || segment_len < len {
            return None;
        }

        if segment_len == 1 {
            return Some(PMAR::new(1.0, segment_len));
        }

        // Reset in case longer segment than args has been passed in
        let segment = &segment[segment_len - len..segment_len];

        let pmar = match ma_type {
            MAType::SMA => {
                let values: Vec<f64> = segment.iter().map(|c| c.close).collect();

                segment.last()?.close / sma(&values)
            },
            MAType::EMA => {
                // REVISIT: This calculation depends on the EMA being populated
                // in the candles. This is due to the rolling nature of EMAs.
                // To properly calculate it here would require a large amount 
                // of iterations when populating further indicators (PMARP) and
                // so this shortcut is taken. Revisit at some later point to 
                // rewrite the entire indicator system.
                let candle = segment.last()?;
                let ema = candle.indicators
                    .get(&IndicatorType::EMA(len))?
                    .as_ema()?;
                candle.close / ema.value
            },
            MAType::VWMA => {
                let values: Vec<(f64,f64)> = segment.iter()
                    .map(|c| (c.close,c.volume))
                    .collect();
                segment.last()?.close / vwma(&values)
            }
        };


        Some(PMAR::new(pmar, segment_len))
    }

    pub fn new(value: f64, len: usize) -> Self {
        Self {
            value,
            len,
            ma: None,
        }
    }

    fn pmar_sma(segment: &[Candle], indicator_type: &IndicatorType) -> Option<f64> {
        let values: Vec<f64> = segment
            .iter()
            .filter_map(|c| c.clone_indicator(indicator_type).ok()?.as_pmar())
            .map(|pmar| pmar.value)
            .collect();

        (values.len() == segment.len()).then_some(sma(&values))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        indicators::{
            indicator_type::IndicatorType, is_indicator::IsIndicator, pmar::PMAR,
            populates_candles::PopulatesCandles, indicator_args::IndicatorArgs,
        },
        models::{candle::Candle, interval::Interval, timeseries::TimeSeries, ma_type::MAType},
        utils::data::dummy_data::PRICE_CHANGES,
    };

    const FINAL_VALUE: f64 = 1.0150163271810746;

    #[test]
    fn pmar_calculate() {
        let candles = Candle::dummy_from_increments(&PRICE_CHANGES);
        let pmar = PMAR::calculate(&candles[1..4]);
        assert!(pmar.is_some());

        let pmar = pmar.unwrap();
        assert_eq!(pmar.value, 1.0012744552973925);
    }

    #[test]
    fn pmar_calculate_args() {
        let candles = Candle::dummy_from_increments(&PRICE_CHANGES);
        let args = IndicatorArgs::PMARArgs(3, MAType::SMA);
        let pmar = PMAR::calculate_args(&candles[1..4], &args);
        assert!(pmar.is_some());
        
        let pmar = pmar.unwrap();
        assert_eq!(pmar.value, 1.0012744552973925);
    }

    #[test]
    fn pmar_no_candles() {
        let candles: Vec<Candle> = Vec::new();
        let sma = PMAR::calculate(&candles);
        assert!(sma.is_none());
    }

    #[test]
    fn pmar_no_candles_args() {
        let candles: Vec<Candle> = Vec::new();
        let args = PMAR::default_args();
        let sma = PMAR::calculate_args(&candles, &args);
        assert!(sma.is_none());
    }

    #[test]
    fn calculate_pmar_single() {
        let candles = Candle::dummy_data(1, "positive", 100.0);
        let sma = PMAR::calculate(&candles);
        assert!(sma.is_some());

        let sma = sma.unwrap();
        assert_eq!(sma.value, 1.0);
    }

    #[test]
    fn pmar_populate_candles() {
        let candles = Candle::dummy_from_increments(&PRICE_CHANGES);
        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);

        let _ = PMAR::populate_candles(&mut ts);

        let (len, ma_type)= PMAR::default_args().pmar_opt().unwrap();
        let indicator_type = IndicatorType::PMAR(len, ma_type);

        for (i, candle) in ts.candles.iter().enumerate() {
            let indicator = candle.indicators.get(&indicator_type).unwrap();
            let pmar = indicator.as_pmar();
            if i < len - 1 {
                assert!(pmar.is_none());
            } else {
                assert!(pmar.is_some());
            }
        }

        let last_candle = ts.candles.last().unwrap();
        let last_pmar = last_candle
            .indicators
            .get(&indicator_type)
            .unwrap()
            .as_pmar()
            .unwrap();
        assert_eq!(last_pmar.value, FINAL_VALUE);
    }

    #[test]
    fn pmar_populate_last_candle() {
        let mut candles = Candle::dummy_from_increments(&PRICE_CHANGES);
        let candle = candles.pop().unwrap();

        let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);
        let _ = PMAR::populate_candles(&mut ts);
        let _ = ts.add_candle(candle);

        let (len, ma_type) = PMAR::default_args().pmar_opt().unwrap();
        let indicator_type = IndicatorType::PMAR(len, ma_type);

        for (i, candle) in ts.candles.iter().enumerate() {
            let indicator = candle.indicators.get(&indicator_type).unwrap();
            let pmar = indicator.as_pmar();
            if i < len - 1 {
                assert!(pmar.is_none());
            } else {
                assert!(pmar.is_some());
            }
        }

        let last_candle = ts.candles.last().unwrap();
        let last_pmar = last_candle
            .indicators
            .get(&indicator_type)
            .unwrap()
            .as_pmar()
            .unwrap();

        assert_eq!(last_pmar.value, FINAL_VALUE);
    }
}
