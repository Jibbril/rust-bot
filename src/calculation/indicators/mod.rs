mod sma;

pub enum CalculationMode {
    Close,
    Open,
    High,
    Low,
}

#[derive(Debug,Clone)]
pub enum IndicatorType {
    SMA(usize),
}

#[derive(Debug,Clone)]
pub enum Indicator {
    Numeric(f32),
    // Logical(bool)
}

