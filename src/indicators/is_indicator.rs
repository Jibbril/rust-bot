use crate::models::{candle::Candle, calculation_mode::CalculationMode};
use super::indicator_args::IndicatorArgs;

pub trait IsIndicator {
    fn default_args() -> IndicatorArgs;
    fn calculate(segment: &[Candle]) -> Option<Self>
    where Self: Sized;
    fn calculate_by_mode(segment: &[Candle], mode: CalculationMode) -> Option<Self>
    where Self: Sized;
}
