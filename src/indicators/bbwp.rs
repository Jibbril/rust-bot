use super::sma::SMA;

/// Bollinger Band Width Percentile
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct BBWP {
    #[allow(dead_code)] // TODO: Remove once used
    pub length: usize,
    pub lookback: usize,
    basis: SMA,
}
