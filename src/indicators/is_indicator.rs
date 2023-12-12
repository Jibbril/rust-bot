use crate::models::candle::Candle;
use super::indicator_args::IndicatorArgs;

pub trait IsIndicator {
    fn default_args() -> IndicatorArgs;
    fn calculate(segment: &[Candle]) -> Option<Self>
    where Self: Sized;
}
