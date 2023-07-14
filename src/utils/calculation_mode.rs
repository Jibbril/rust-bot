use crate::utils::timeseries::Candle;

pub enum CalculationMode {
    Close,
    #[allow(dead_code)] // TODO: Remove once used
    Open,
    #[allow(dead_code)] // TODO: Remove once used
    High,
    #[allow(dead_code)] // TODO: Remove once used
    Low,
}

pub fn price_by_calc_mode(candle: &Candle, mode: &CalculationMode) -> f64 {
    match mode {
        CalculationMode::Close => candle.close,
        CalculationMode::Open => candle.open,
        CalculationMode::High => candle.high,
        CalculationMode::Low => candle.low,
    }
}
