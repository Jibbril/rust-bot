use serde::Serialize;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize)]
pub enum IndicatorType {
    SMA(usize),
    RSI(usize),
    ATR(usize),
    BollingerBands(usize),
    BBW(usize),
    BBWP(usize,usize), // length, lookback
    DynamicPivot(usize),
}