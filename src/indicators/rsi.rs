use super::{
    indicator::Indicator, indicator_args::IndicatorArgs, indicator_type::IndicatorType,
    is_indicator::IsIndicator, populates_candles::PopulatesCandles,
};
use crate::models::{candle::Candle, timeseries::TimeSeries};
use anyhow::{anyhow, Context, Result};
use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize, PartialEq, PartialOrd)]
pub struct RSI {
    pub value: f64,
    pub len: usize,
    pub avg_gain: f64,
    pub avg_loss: f64,
}

impl PopulatesCandles for RSI {
    fn populate_candles(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_candles_args(ts, Self::default_args())
    }

    fn populate_candles_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let len = args.len_res()?;
        let indicator_type = IndicatorType::RSI(len);

        let mut prev: Option<RSI> = None;

        for i in 0..ts.candles.len() {
            let rsi = if i < len {
                None
            } else if i == len || prev.is_none() {
                let start = i - len;
                Self::calculate_args(&ts.candles[start..i + 1], &args)
            } else {
                let candles = (&ts.candles[i - 1], &ts.candles[i]);
                let prev = prev.unwrap();
                Self::calculate_rolling(candles, prev, len)
            };

            ts.candles[i]
                .indicators
                .insert(indicator_type, Indicator::RSI(rsi));
            prev = rsi;
        }

        ts.indicators.insert(indicator_type);

        Ok(())
    }

    fn populate_last_candle(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_last_candle_args(ts, Self::default_args())
    }

    fn populate_last_candle_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let len = args.len_res()?;
        let indicator_type = IndicatorType::RSI(len);
        let ctx_err = "Unable to get last candle";
        let candle_len = ts.candles.len();

        if candle_len == 0 {
            return Err(anyhow!("No candle to populate"));
        }

        // Not enough candles to populate
        if candle_len < len {
            ts.candles
                .last_mut()
                .context(ctx_err)?
                .indicators
                .insert(indicator_type, Indicator::RSI(None));

            return Ok(());
        };

        // Calculate new rsi and populate
        let prev_rsi = Indicator::get_second_last(ts, &indicator_type).and_then(|rsi| rsi.as_rsi());

        let new_rsi = if prev_rsi.is_none() {
            let start = candle_len - len;
            let end = candle_len - 1;
            Self::calculate_args(&ts.candles[start..end], &args)
        } else {
            let candles = (&ts.candles[candle_len - 2], &ts.candles[candle_len - 1]);
            Self::calculate_rolling(candles, prev_rsi.unwrap(), len)
        };

        ts.candles
            .last_mut()
            .context(ctx_err)?
            .indicators
            .insert(indicator_type, Indicator::RSI(new_rsi));

        Ok(())
    }
}

impl IsIndicator for RSI {
    fn default_args() -> IndicatorArgs {
        IndicatorArgs::LengthArg(14)
    }

    /// Segment should be the length + 1 for the RSI wanted.
    fn calculate(segment: &[Candle]) -> Option<Self>
    where
        Self: Sized,
    {
        if segment.len() == 0 {
            return None;
        }
        let len = segment.len() - 1;

        let (avg_gain, avg_loss) = Self::get_outcomes(segment);

        let rs = if avg_loss != 0.0 {
            avg_gain / avg_loss
        } else {
            f64::INFINITY
        };

        Self::calculate_rsi(rs, len, (avg_gain, avg_loss))
    }

    fn calculate_args(segment: &[Candle], args: &IndicatorArgs) -> Option<Self>
    where
        Self: Sized,
    {
        let len = args.len_opt()?;
        let candle_len = segment.len();

        if candle_len < len + 1 {
            return None;
        }

        Self::calculate(&segment[candle_len - len - 1..candle_len])
    }
}

impl RSI {
    fn calculate_rsi(rs: f64, len: usize, (avg_gain, avg_loss): (f64, f64)) -> Option<RSI> {
        let rsi = if rs.is_infinite() {
            100.0
        } else {
            100.0 - 100.0 / (1.0 + rs)
        };

        Some(RSI {
            len,
            value: rsi,
            avg_gain,
            avg_loss,
        })
    }

    fn calculate_rolling(
        (prev_candle, curr_candle): (&Candle, &Candle),
        prev_rsi: Self,
        len: usize,
    ) -> Option<RSI> {
        let f_len = len as f64;
        let mut gains = prev_rsi.avg_gain * (f_len - 1.0);
        let mut losses = prev_rsi.avg_loss * (f_len - 1.0);
        let change = curr_candle.close - prev_candle.close;

        if change >= 0.0 {
            gains += change;
        } else {
            losses += -change;
        }

        let rs = if losses != 0.0 {
            gains / losses
        } else {
            f64::INFINITY
        };

        Self::calculate_rsi(rs, len, (gains / f_len, losses / f_len))
    }

    fn get_outcomes(segment: &[Candle]) -> (f64, f64) {
        let mut gains = 0.0;
        let mut losses = 0.0;
        let len = segment.len();

        for i in 1..len {
            let current = segment[i].close;
            let previous = segment[i - 1].close;

            let change = current - previous;
            if change > 0.0 {
                gains += change;
            } else {
                losses += -change;
            }
        }

        // Segment contains one extra candle since an initial value is needed
        let f_len = (len - 1) as f64;
        (gains / f_len, losses / f_len)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        indicators::{
            indicator_args::IndicatorArgs, indicator_type::IndicatorType,
            is_indicator::IsIndicator, populates_candles::PopulatesCandles, rsi::RSI,
        },
        models::{candle::Candle, interval::Interval, timeseries_builder::TimeSeriesBuilder},
    };

    #[test]
    fn rsi_calculate() {
        let candles = Candle::dummy_data(14, "alternating", 100.0);
        let rsi = RSI::calculate(&candles);

        assert!(rsi.is_some());
        let rsi = rsi.unwrap();
        assert!(rsi.value >= 0.0 && rsi.value <= 100.0)
    }

    #[test]
    fn rsi_calculate_args() {
        let candles = Candle::dummy_data(14, "alternating", 100.0);
        let args = IndicatorArgs::LengthArg(13);
        let rsi = RSI::calculate_args(&candles, &args);

        assert!(rsi.is_some());
        let rsi = rsi.unwrap();
        assert!(rsi.value >= 0.0 && rsi.value <= 100.0)
    }

    #[test]
    fn rsi_no_candles() {
        let candles: Vec<Candle> = Vec::new();
        let rsi = RSI::calculate(&candles);
        assert!(rsi.is_none());
    }

    #[test]
    fn rsi_no_candles_args() {
        let candles: Vec<Candle> = Vec::new();
        let args = RSI::default_args();
        let rsi = RSI::calculate_args(&candles, &args);
        assert!(rsi.is_none());
    }

    #[test]
    fn rsi_populate_candles() {
        let candles = Candle::dummy_data(130, "alternating", 100.0);
        let mut ts = TimeSeriesBuilder::new()
            .symbol("DUMMY".to_string())
            .interval(Interval::Day1)
            .candles(candles)
            .build();

        let _ = RSI::populate_candles(&mut ts);

        let len = RSI::default_args().len_opt().unwrap();
        let indicator_type = IndicatorType::RSI(len);

        for (i, candle) in ts.candles.iter().enumerate() {
            let indicator = candle.indicators.get(&indicator_type).unwrap();
            let rsi = indicator.as_rsi();
            if i < len {
                assert!(rsi.is_none());
            } else {
                assert!(rsi.is_some());
            }
        }

        let last_candle = ts.candles.last().unwrap();
        let last_sma = last_candle
            .indicators
            .get(&indicator_type)
            .unwrap()
            .as_rsi()
            .unwrap();
        assert!(last_sma.value - 48.14777970740 < 0.00001);
    }

    #[test]
    fn rsi_populate_last_candle() {
        let candles = Candle::dummy_data(150, "alternating", 100.0);
        let mut ts = TimeSeriesBuilder::new()
            .symbol("DUMMY".to_string())
            .interval(Interval::Day1)
            .candles(candles)
            .build();

        let _ = RSI::populate_candles(&mut ts);

        let candle = Candle::dummy_from_val(200.0);

        let _ = ts.add_candle(&candle);

        let len = RSI::default_args().len_opt().unwrap();
        let indicator_type = IndicatorType::RSI(len);

        for (i, candle) in ts.candles.iter().enumerate() {
            let indicator = candle.indicators.get(&indicator_type).unwrap();
            let rsi = indicator.as_rsi();
            if i < len {
                assert!(rsi.is_none());
            } else {
                assert!(rsi.is_some());
            }
        }

        let last_candle = ts.candles.last().unwrap();
        let last_rsi = last_candle
            .indicators
            .get(&indicator_type)
            .unwrap()
            .as_rsi()
            .unwrap();

        assert!(last_rsi.value - 70.6923842589078 < 0.0001);
    }
}
