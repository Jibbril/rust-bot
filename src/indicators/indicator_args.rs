use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Copy)]
pub enum IndicatorArgs {
    LengthArg(usize),
    BollingerBandArgs(usize, f64), // length, n-standard deviations
    BBWPArgs(usize, usize, usize), // bbwp-length, lookback, sma-length
    LengthLookbackArgs(usize,usize), // Length, lookback
}

const ERR_MSG: &str = "Invalid indicator arguments.";

impl IndicatorArgs {
    #[allow(dead_code)]
    pub fn extract_len_opt(&self) -> Option<usize> {
        match self {
            IndicatorArgs::LengthArg(n) => Some(*n),
            _ => return None,
        }
    }

    pub fn extract_len_res(&self) -> Result<usize> {
        match self {
            IndicatorArgs::LengthArg(n) => Ok(*n),
            _ => return Err(anyhow!(ERR_MSG)),
        }
    }

    pub fn extract_bb_opt(&self) -> Option<(usize, f64)> {
        match self {
            IndicatorArgs::BollingerBandArgs(n, m) => Some((*n, *m)),
            _ => return None,
        }
    }

    pub fn extract_bb_res(&self) -> Result<(usize, f64)> {
        match self {
            IndicatorArgs::BollingerBandArgs(n, m) => Ok((*n, *m)),
            _ => return Err(anyhow!(ERR_MSG)),
        }
    }

    #[allow(dead_code)]
    pub fn extract_bbwp_opt(&self) -> Option<(usize, usize, usize)> {
        match self {
            IndicatorArgs::BBWPArgs(a, b, c) => Some((*a, *b, *c)),
            _ => return None,
        }
    }

    pub fn extract_bbwp_res(&self) -> Result<(usize, usize, usize)> {
        match self {
            IndicatorArgs::BBWPArgs(a, b, c) => Ok((*a, *b, *c)),
            _ => return Err(anyhow!(ERR_MSG)),
        }
    }

    pub fn extract_len_lookback_opt(&self) -> Option<(usize, usize)> {
        match self {
            IndicatorArgs::LengthLookbackArgs(a, b) => Some((*a, *b)),
            _ => return None,
        }
    }

    pub fn extract_len_lookback_res(&self) -> Result<(usize, usize)> {
        match self {
            IndicatorArgs::LengthLookbackArgs(a, b) => Ok((*a, *b)),
            _ => return Err(anyhow!(ERR_MSG)),
        }
    }
}
