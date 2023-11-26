use anyhow::{Context, Result};
use serde::Serialize;
use crate::models::{calculation_mode::CalculationMode, candle::Candle, timeseries::TimeSeries};
use super::{
    indicator::Indicator, indicator_args::IndicatorArgs, indicator_type::IndicatorType,
    is_indicator::IsIndicator, populates_candles::PopulatesCandles,
};

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
        let len = args.extract_len_res()?;
        let mut rsi: Option<RSI> = None;
        let new_rsis: Vec<Option<RSI>> = (0..ts.candles.len())
            .map(|i| {
                rsi = Self::calculate_rolling(len, i, &ts.candles, &rsi);
                rsi
            })
            .collect();

        let indicator_type = IndicatorType::RSI(len);

        for (i, candle) in ts.candles.iter_mut().enumerate() {
            let new_rsi = Indicator::RSI(new_rsis[i]);
            candle.indicators.insert(indicator_type, new_rsi);
        }

        ts.indicators.insert(indicator_type);

        Ok(())
    }

    fn populate_last_candle(ts: &mut TimeSeries) -> Result<()> {
        Self::populate_last_candle_args(ts, Self::default_args())
    }

    fn populate_last_candle_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let len = args.extract_len_res()?;
        let indicator_type = IndicatorType::RSI(len);

        let previous_rsi =
            Indicator::get_second_last(ts, &indicator_type).and_then(|rsi| rsi.as_rsi());

        let new_rsi =
            Self::calculate_rolling(len, ts.candles.len() - 1, &ts.candles, &previous_rsi);
        let new_rsi = Indicator::RSI(new_rsi);
        
        let new_candle = ts.candles.last_mut().context("Failed to get last candle")?;
        new_candle.indicators.insert(indicator_type, new_rsi);

        Ok(())
    }
}

impl IsIndicator for RSI {
    fn default_args() -> IndicatorArgs {
        IndicatorArgs::LengthArg(14)
    }
}

impl RSI {
    // Default implementation using closing values and for calculations.
    pub fn calculate_rolling(
        len: usize,
        i: usize,
        candles: &Vec<Candle>,
        prev_rsi: &Option<RSI>,
    ) -> Option<RSI> {
        Self::calculate_rolling_with_opts(len, i, candles, CalculationMode::Close, prev_rsi)
    }

    fn calculate_rolling_with_opts(
        len: usize,
        i: usize,
        candles: &Vec<Candle>,
        mode: CalculationMode,
        prev_rsi: &Option<RSI>,
    ) -> Option<RSI> {
        if i < len - 1 || i >= candles.len() || candles.len() < len {
            None
        } else if let Some(prev_rsi) = prev_rsi {
            let current = candles[i].price_by_mode(&mode);
            let previous = candles[i - 1].price_by_mode(&mode);

            let f_len = len as f64;
            let mut gains = prev_rsi.avg_gain * (f_len - 1.0);
            let mut losses = prev_rsi.avg_loss * (f_len - 1.0);

            let change = current - previous;
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

            let rsi = if rs.is_finite() {
                100.0 - (100.0 / (1.0 + rs))
            } else {
                100.0
            };

            Some(RSI {
                len,
                value: rsi,
                avg_gain: gains / f_len,
                avg_loss: losses / f_len,
            })
        } else {
            Self::calculate_with_opts(len, i, candles, mode)
        }
    }

    // Default implementation using closing values for calculations.
    #[allow(dead_code)]
    pub fn calculate(i: usize, candles: &Vec<Candle>) -> Option<RSI> {
        let len = Self::default_args().extract_len_opt()?;
        Self::calculate_with_opts(len, i, candles, CalculationMode::Close)
    }

    fn calculate_with_opts(
        len: usize,
        i: usize,
        candles: &Vec<Candle>,
        mode: CalculationMode,
    ) -> Option<RSI> {
        if i < len - 1 || len > candles.len() {
            None
        } else {
            let start = i + 1 - len;
            let end = i + 1;
            let segment = &candles[start..end];

            let (avg_gain, avg_loss) = Self::get_outcomes(segment, mode);

            let rs = if avg_loss != 0.0 {
                avg_gain / avg_loss
            } else {
                f64::INFINITY
            };

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
    }

    fn get_outcomes(segment: &[Candle], mode: CalculationMode) -> (f64, f64) {
        let mut gains = 0.0;
        let mut losses = 0.0;

        for i in 1..segment.len() {
            let current = segment[i].price_by_mode(&mode);
            let previous = segment[i - 1].price_by_mode(&mode);

            let change = current - previous;
            if change > 0.0 {
                gains += change;
            } else {
                losses += -change;
            }
        }

        let f_len = segment.len() as f64;

        (gains / f_len, losses / f_len)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        indicators::{is_indicator::IsIndicator, rsi::RSI},
        models::{calculation_mode::CalculationMode, candle::Candle},
    };

    #[test]
    fn test_rsi_calculation() {
        let candles = Candle::dummy_data(14, "alternating", 100.0);
        let rsi = RSI::calculate_with_opts(14, 13, &candles, CalculationMode::Close);

        assert!(rsi.is_some());
        let rsi = rsi.unwrap();
        assert!(rsi.value >= 0.0 && rsi.value <= 100.0)
    }

    #[test]
    fn test_rsi_calculation_not_enough_data() {
        let candles = Candle::dummy_data(3, "", 100.0);
        let rsi = RSI::calculate_with_opts(14, 13, &candles, CalculationMode::Close);

        assert!(rsi.is_none());
    }

    #[test]
    fn no_candles() {
        let candles: Vec<Candle> = Vec::new();
        let rsi = RSI::calculate_with_opts(14, 13, &candles, CalculationMode::Close);

        assert!(rsi.is_none());
    }

    #[test]
    fn rolling_rsi_test() {
        let n = 20;
        let candles = Candle::dummy_data(n, "alternating", 100.0);
        let mut rsi = None;
        let len = RSI::default_args().extract_len_opt().unwrap();

        let rsis: Vec<Option<RSI>> = (0..n)
            .map(|i| {
                rsi = RSI::calculate_rolling(len, i, &candles, &rsi);
                rsi
            })
            .collect();

        for (i, rsi) in rsis.iter().enumerate() {
            if i < 13 {
                assert!(rsi.is_none())
            } else {
                assert!(rsi.is_some())
            }
        }

        // assert that last value is reasonably close
        assert!((rsis[n - 1].unwrap().value - 46.86).abs() < 0.1)
    }
}
