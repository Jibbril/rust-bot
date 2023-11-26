use super::indicator_args::IndicatorArgs;

pub trait IsIndicator {
    fn default_args() -> IndicatorArgs;
}
