mod sma;
mod rsi;

use sma::SMA;
use rsi::RSI;

pub enum CalculationMode {
    Close,
    Open,
    High,
    Low,
}

#[derive(Debug,Clone)]
pub enum IndicatorType {
    SMA(usize),
    RSI(usize),
}

#[derive(Debug,Clone)]
pub enum Indicator {
    SMA(SMA),
    RSI(RSI),
}