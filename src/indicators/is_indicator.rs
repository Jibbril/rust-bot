use super::indicator_args::IndicatorArgs;
use crate::models::candle::Candle;

pub trait IsIndicator {
    fn default_args() -> IndicatorArgs;
    fn calculate(segment: &[Candle]) -> Option<Self>
    where
        Self: Sized;
}
