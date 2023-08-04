use crate::{models::{generic_result::GenericResult, candle::Candle, calculation_mode::{CalculationMode, price_by_calc_mode}}, indicators::{IndicatorType, Indicator}};

use super::PopulatesCandles;



#[derive(Debug, Copy, Clone)]
pub struct DynamicPivot {
    // TODO: Replace with length
    pub bars_left: usize,
    pub bars_right: usize,
    pub high: f64,
    pub low: f64,
}

impl PopulatesCandles for DynamicPivot {
    fn populate_candles(candles: &mut Vec<Candle>, length: usize) -> GenericResult<()> {
        let mut pivots: Option<DynamicPivot> = None; 
        let new_pivots: Vec<Option<DynamicPivot>> = (0..candles.len())
            .map(|i| {
                pivots = Self::calculate_rolling(length,i,candles,&pivots);
                pivots
            })
            .collect();

        let indicator_type = IndicatorType::DynamicPivot(length);

        for (i,candle) in candles.iter_mut().enumerate() {
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
        if !Self::calculation_ok(i,length, arr_length) {
            None
        } else if let Some(prev_pivot) = prev {
            let is_high = candles[i].high > prev_pivot.high;
            let is_low = candles[i].low < prev_pivot.low;

            if let Some(prev) = Self::get_prev_pivots(length, i, candles) {
                Self::build_pivot(length, &prev, &candles[i], is_high, is_low)
            } else {
                None
            }
        } else {
            Self::calculate(length, i, candles)
        }
    }

    // Default implementation using closing values for calculations.
    pub fn calculate(length: usize, i: usize, candles: &Vec<Candle>) -> Option<DynamicPivot> {
        Self::calculate_with_opts(length, i, candles)
    }

    fn calculate_with_opts(
        length: usize,
        i: usize,
        candles: &Vec<Candle>,
    ) -> Option<DynamicPivot> {
        let arr_length = candles.len();
        if !Self::calculation_ok(i, length, arr_length) {
            return None
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
        let segment = &candles[i+1-length..i+length];

        if high_check {
            !segment.iter().any(|c| c.high > candles[i].high)
        } else {
            !segment.iter().any(|c| c.low < candles[i].low)
        }
    }


    fn build_pivot(length: usize, prev: &DynamicPivot, candle: &Candle, is_high: bool, is_low: bool) -> Option<DynamicPivot> {
        let mut high = prev.high;
        let mut low = prev.low;

        if is_high {
            high = candle.high
        } 
        
        if is_low {
            low = candle.low
        }

        Some(DynamicPivot {
            bars_left: length,
            bars_right: length,
            high,
            low
        })
    }

    fn get_prev_pivots(length: usize,i: usize, candles: &[Candle]) -> Option<DynamicPivot> {
        let prev_pivots = candles[i-1]
            .get_indicator(&IndicatorType::DynamicPivot(length));

        match prev_pivots {
            Ok(i) => i.as_dynamic_pivots(),
            _ => return None
        }
    }

    fn calculation_ok(i: usize, length: usize, arr_length: usize)  -> bool {
        i < arr_length 
        && length <= arr_length 
        && i >= length - 1 
        && i + length < arr_length
    }
}