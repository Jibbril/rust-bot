use serde::Serialize;

use crate::models::{
    calculation_mode::CalculationMode, candle::Candle, generic_result::GenericResult,
    timeseries::TimeSeries,
};

use super::{ indicator::Indicator, indicator_type::IndicatorType, populates_candles::PopulatesCandles, indicator_args::IndicatorArgs};

#[derive(Debug, Copy, Clone, Serialize, PartialEq, PartialOrd)]
pub struct RSI {
    pub value: f64,
    #[allow(dead_code)] // TODO: Remove once used
    pub length: usize,
    pub avg_gain: f64,
    pub avg_loss: f64,
}

impl PopulatesCandles for RSI {
    fn populate_candles_default(ts: &mut TimeSeries) -> GenericResult<()> {
        let args = IndicatorArgs::LengthArg(14);
        Self::populate_candles(ts, args)
    }

    fn populate_candles(ts: &mut TimeSeries, args: IndicatorArgs) -> GenericResult<()> {
        let length = args.extract_length_arg_res()?;
        let mut rsi: Option<RSI> = None;
        let new_rsis: Vec<Option<RSI>> = (0..ts.candles.len())
            .map(|i| {
                rsi = Self::calculate_rolling(i, &ts.candles, &rsi);
                rsi
            })
            .collect();

        let indicator_type = IndicatorType::RSI(length);

        for (i, candle) in ts.candles.iter_mut().enumerate() {
            let new_rsi = Indicator::RSI(new_rsis[i]);

            candle.indicators.insert(indicator_type, new_rsi);
        }

        Ok(())
    }
}

impl RSI {
    // Default implementation using closing values and for calculations.
    pub fn calculate_rolling(
        i: usize,
        candles: &Vec<Candle>,
        prev_rsi: &Option<RSI>,
    ) -> Option<RSI> {
        Self::calculate_rolling_with_opts(14, i, candles, CalculationMode::Close, prev_rsi)
    }

    fn calculate_rolling_with_opts(
        length: usize,
        i: usize,
        candles: &Vec<Candle>,
        mode: CalculationMode,
        prev_rsi: &Option<RSI>,
    ) -> Option<RSI> {
        if i < length - 1 || i >= candles.len() || candles.len() < length {
            None
        } else if let Some(prev_rsi) = prev_rsi {
            let current = candles[i].price_by_mode(&mode);
            let previous = candles[i - 1].price_by_mode(&mode);

            let f_length = length as f64;
            let mut gains = prev_rsi.avg_gain * (f_length - 1.0);
            let mut losses = prev_rsi.avg_loss * (f_length - 1.0);

            let change = current - previous;
            if change >= 0.0 {
                gains += change;
            } else {
                losses += -change;
            }

            let rs = if losses != 0.0 { gains / losses } else { 0.0 };

            let rsi = 100.0 - (100.0 / (1.0 + rs));

            Some(RSI {
                length,
                value: rsi,
                avg_gain: gains / f_length,
                avg_loss: losses / f_length,
            })
        } else {
            Self::calculate(i, candles)
        }
    }

    // Default implementation using closing values for calculations.
    pub fn calculate(i: usize, candles: &Vec<Candle>) -> Option<RSI> {
        Self::calculate_with_opts(14, i, candles, CalculationMode::Close)
    }

    fn calculate_with_opts(
        length: usize,
        i: usize,
        candles: &Vec<Candle>,
        mode: CalculationMode,
    ) -> Option<RSI> {
        if i < length - 1 || length > candles.len() {
            None
        } else {
            let f_length = length as f64;
            let start = i + 1 - length;
            let end = i + 1;
            let segment = &candles[start..end];

            let (gains, losses) = Self::get_outcomes(segment, mode);

            let rs = if losses != 0.0 { gains / losses } else { 0.0 };

            Some(RSI {
                length,
                value: 100.0 - 100.0 / (1.0 + rs),
                avg_gain: gains / f_length,
                avg_loss: losses / f_length,
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

        (gains, losses)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        indicators::rsi::RSI,
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

        let rsis: Vec<Option<RSI>> = (0..n)
            .map(|i| {
                rsi = RSI::calculate_rolling(i, &candles, &rsi);
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
