use anyhow::Result;

use super::{
    indicator::Indicator, indicator_args::IndicatorArgs, indicator_type::IndicatorType,
    populates_candles::PopulatesCandles,
};
use crate::models::{candle::Candle, timeseries::TimeSeries};

#[derive(Debug, Copy, Clone)]
pub struct DynamicPivot {
    pub len: usize,
    pub high: f64,
    pub low: f64,
}

impl PopulatesCandles for DynamicPivot {
    fn populate_candles(ts: &mut TimeSeries) -> Result<()> {
        let args = IndicatorArgs::LengthArg(15);
        Self::populate_candles_args(ts, args)
    }

    fn populate_candles_args(ts: &mut TimeSeries, args: IndicatorArgs) -> Result<()> {
        let len = args.extract_len_res()?;
        let mut new_pivots: Vec<Option<DynamicPivot>> = (0..len).map(|_| None).collect();

        let mut pivots: Option<DynamicPivot> = Some(DynamicPivot {
            len,
            high: ts.candles[len].high,
            low: ts.candles[len].low,
        });

        // Push initial pivots
        new_pivots.push(pivots);

        for i in len + 1..ts.candles.len() {
            pivots = Self::calculate_rolling(len, i, &ts.candles, &pivots);
            new_pivots.push(pivots);
        }

        let indicator_type = IndicatorType::DynamicPivot(len);

        for (i, candle) in ts.candles.iter_mut().enumerate() {
            let new_pivots = Indicator::DynamicPivot(new_pivots[i]);

            candle.indicators.insert(indicator_type, new_pivots);
        }

        ts.indicators.insert(indicator_type);

        Ok(())
    }
}

impl DynamicPivot {
    // Default implementation using closing values for calculations.
    pub fn calculate_rolling(
        len: usize,
        i: usize,
        candles: &Vec<Candle>,
        prev: &Option<DynamicPivot>,
    ) -> Option<DynamicPivot> {
        Self::calculate_rolling_with_opts(len, i, candles, prev)
    }

    fn calculate_rolling_with_opts(
        len: usize,
        i: usize,
        candles: &Vec<Candle>,
        prev: &Option<DynamicPivot>,
    ) -> Option<DynamicPivot> {
        let arr_len = candles.len();
        if !Self::calculation_ok(i, len, arr_len) {
            None
        } else if let Some(prev_pivot) = prev {
            let is_high = Self::is_pivot(candles, i, len, true);
            let is_low = Self::is_pivot(candles, i, len, false);

            Self::build_pivot(len, &prev_pivot, &candles[i], is_high, is_low)
        } else {
            Self::calculate(len, i, candles)
        }
    }

    // Default implementation using closing values for calculations.
    pub fn calculate(len: usize, i: usize, candles: &Vec<Candle>) -> Option<DynamicPivot> {
        Self::calculate_with_opts(len, i, candles)
    }

    fn calculate_with_opts(len: usize, i: usize, candles: &Vec<Candle>) -> Option<DynamicPivot> {
        let arr_len = candles.len();
        if !Self::calculation_ok(i, len, arr_len) {
            return None;
        }

        let is_high = Self::is_pivot(candles, i, len, true);
        let is_low = Self::is_pivot(candles, i, len, false);

        if let Some(prev_pivots) = Self::get_prev_pivots(len, i, candles) {
            Self::build_pivot(len, &prev_pivots, &candles[i], is_high, is_low)
        } else {
            None
        }
    }

    fn is_pivot(candles: &[Candle], i: usize, len: usize, high_check: bool) -> bool {
        let segment = &candles[i + 1 - len..i + len];

        if high_check {
            !segment.iter().any(|c| c.high > candles[i].high)
        } else {
            !segment.iter().any(|c| c.low < candles[i].low)
        }
    }

    fn build_pivot(
        len: usize,
        prev: &DynamicPivot,
        candle: &Candle,
        is_high: bool,
        is_low: bool,
    ) -> Option<DynamicPivot> {
        let mut high = prev.high;
        let mut low = prev.low;

        if is_high {
            high = candle.high
        }

        if is_low {
            low = candle.low
        }

        Some(DynamicPivot { len, high, low })
    }

    pub fn get_prev_pivots(len: usize, i: usize, candles: &[Candle]) -> Option<DynamicPivot> {
        if i == 0 {
            return None;
        }

        let mut j = i;

        loop {
            let prev_pivots = candles[i - 1].get_indicator(&IndicatorType::DynamicPivot(len));
            if let Ok(i) = prev_pivots {
                return i.as_dynamic_pivots();
            }

            j -= 1;
            if j == 0 {
                break;
            }
        }

        None
    }

    fn calculation_ok(i: usize, len: usize, arr_len: usize) -> bool {
        i < arr_len && len <= arr_len && i >= len - 1 && i + len < arr_len
    }
}
