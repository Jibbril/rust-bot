use crate::{
    indicators::{Indicator, IndicatorType},
    models::{
        calculation_mode::{price_by_calc_mode, CalculationMode},
        candle::Candle,
        generic_result::GenericResult,
    },
};
use super::PopulatesCandles;

#[derive(Debug, Copy, Clone)]
pub struct DynamicPivot {
    pub length: usize,
    pub high: f64,
    pub low: f64,
}

impl PopulatesCandles for DynamicPivot {
    fn populate_candles(candles: &mut Vec<Candle>, length: usize) -> GenericResult<()> {
        let mut new_pivots: Vec<Option<DynamicPivot>> = (0..length).map(|_| None).collect();

        let mut pivots: Option<DynamicPivot> = Some(DynamicPivot {
            length,
            high: candles[length].high,
            low: candles[length].low,
        });

        // Push initial pivots
        new_pivots.push(pivots);

        for i in length + 1..candles.len() {
            pivots = Self::calculate_rolling(length, i, candles, &pivots);
            new_pivots.push(pivots);
        }

        let indicator_type = IndicatorType::DynamicPivot(length);

        for (i, candle) in candles.iter_mut().enumerate() {
            let new_pivots = Indicator::DynamicPivot(new_pivots[i]);

            candle.indicators.insert(indicator_type, new_pivots);
        }

        Ok(())
    }
}

impl DynamicPivot {
    // Default implementation using closing values for calculations.
    pub fn calculate_rolling(
        length: usize,
        i: usize,
        candles: &Vec<Candle>,
        prev: &Option<DynamicPivot>,
    ) -> Option<DynamicPivot> {
        Self::calculate_rolling_with_opts(length, i, candles, prev)
    }

    fn calculate_rolling_with_opts(
        length: usize,
        i: usize,
        candles: &Vec<Candle>,
        prev: &Option<DynamicPivot>,
    ) -> Option<DynamicPivot> {
        let arr_length = candles.len();
        if !Self::calculation_ok(i, length, arr_length) {
            None
        } else if let Some(prev_pivot) = prev {
            let is_high = Self::is_pivot(candles, i, length, true);
            let is_low = Self::is_pivot(candles, i, length, false);

            Self::build_pivot(length, &prev_pivot, &candles[i], is_high, is_low)
        } else {
            Self::calculate(length, i, candles)
        }
    }

    // Default implementation using closing values for calculations.
    pub fn calculate(length: usize, i: usize, candles: &Vec<Candle>) -> Option<DynamicPivot> {
        Self::calculate_with_opts(length, i, candles)
    }

    fn calculate_with_opts(length: usize, i: usize, candles: &Vec<Candle>) -> Option<DynamicPivot> {
        let arr_length = candles.len();
        if !Self::calculation_ok(i, length, arr_length) {
            return None;
        }

        let is_high = Self::is_pivot(candles, i, length, true);
        let is_low = Self::is_pivot(candles, i, length, false);

        if let Some(prev_pivots) = Self::get_prev_pivots(length, i, candles) {
            Self::build_pivot(length, &prev_pivots, &candles[i], is_high, is_low)
        } else {
            None
        }
    }

    fn is_pivot(candles: &[Candle], i: usize, length: usize, high_check: bool) -> bool {
        let segment = &candles[i + 1 - length..i + length];

        if high_check {
            !segment.iter().any(|c| c.high > candles[i].high)
        } else {
            !segment.iter().any(|c| c.low < candles[i].low)
        }
    }

    fn build_pivot(
        length: usize,
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

        Some(DynamicPivot { length, high, low })
    }

    pub fn get_prev_pivots(length: usize, i: usize, candles: &[Candle]) -> Option<DynamicPivot> {
        if i == 0 {
            return None;
        }

        let mut j = i;

        loop {
            let prev_pivots = candles[i - 1].get_indicator(&IndicatorType::DynamicPivot(length));
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

    fn calculation_ok(i: usize, length: usize, arr_length: usize) -> bool {
        i < arr_length && length <= arr_length && i >= length - 1 && i + length < arr_length
    }
}
