use crate::indicators::indicator_type::IndicatorType;

pub trait RequiresIndicators {
    /// Required indicators for succesful use of resolution strategy
    fn required_indicators(&self) -> Vec<IndicatorType>;
}
