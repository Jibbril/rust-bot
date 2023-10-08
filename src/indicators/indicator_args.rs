use anyhow::{Result, anyhow};


#[derive(Debug, Clone, Copy)]
pub enum IndicatorArgs {
    LengthArg(usize),
    BollingerBandArgs(usize, f64), // length, n-standard deviations
    BBWPArgs(usize, usize, usize), // bbwp-length, lookback, sma-length
}

const ERR: &str = "Invalid indicator arguments.";

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
            _ => return Err(anyhow!(ERR)),
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
            _ => return Err(anyhow!(ERR)),
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
            _ => return Err(anyhow!(ERR)),
        }
    }
}
