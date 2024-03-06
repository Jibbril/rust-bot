use super::indicator_args::IndicatorArgs;
use crate::models::candle::Candle;

pub trait IsIndicator {
    fn default_args() -> IndicatorArgs;
    fn calculate(segment: &[Candle]) -> Option<Self>
    where
        Self: Sized;
    fn calculate_args(segment: &[Candle], args: &IndicatorArgs) -> Option<Self>
    where
        Self: Sized;
}
