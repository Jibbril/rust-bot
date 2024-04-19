use crate::models::ma_type::MAType;
use anyhow::{anyhow, Result};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum IndicatorArgs {
    LengthArg(usize),
    BollingerBandArgs(usize, f64),    // length, n-standard deviations
    BBWPArgs(usize, usize, usize),    // bbwp-length, lookback, sma-length
    LengthLookbackArgs(usize, usize), // Length, lookback
    PMARArgs(usize, MAType),          // Length, moving average type
    PMARPArgs(usize, usize, MAType),  // Length, lookback, moving average type
    StochasticArgs(usize, usize, usize), // K length, K smoothing, D Smoothing
}

const ERR_MSG: &str = "Invalid indicator arguments.";

impl IndicatorArgs {
    #[allow(dead_code)]
    pub fn len_opt(&self) -> Option<usize> {
        match self {
            IndicatorArgs::LengthArg(n) => Some(*n),
            _ => return None,
        }
    }

    pub fn len_res(&self) -> Result<usize> {
        match self {
            IndicatorArgs::LengthArg(n) => Ok(*n),
            _ => return Err(anyhow!(ERR_MSG)),
        }
    }

    pub fn bb_opt(&self) -> Option<(usize, f64)> {
        match self {
            IndicatorArgs::BollingerBandArgs(n, m) => Some((*n, *m)),
            _ => return None,
        }
    }

    pub fn bb_res(&self) -> Result<(usize, f64)> {
        match self {
            IndicatorArgs::BollingerBandArgs(n, m) => Ok((*n, *m)),
            _ => return Err(anyhow!(ERR_MSG)),
        }
    }

    #[allow(dead_code)]
    pub fn bbwp_opt(&self) -> Option<(usize, usize, usize)> {
        match self {
            IndicatorArgs::BBWPArgs(a, b, c) => Some((*a, *b, *c)),
            _ => return None,
        }
    }

    pub fn bbwp_res(&self) -> Result<(usize, usize, usize)> {
        match self {
            IndicatorArgs::BBWPArgs(a, b, c) => Ok((*a, *b, *c)),
            _ => return Err(anyhow!(ERR_MSG)),
        }
    }

    #[allow(dead_code)]
    pub fn len_lookback_opt(&self) -> Option<(usize, usize)> {
        match self {
            IndicatorArgs::LengthLookbackArgs(a, b) => Some((*a, *b)),
            _ => return None,
        }
    }

    #[allow(dead_code)]
    pub fn len_lookback_res(&self) -> Result<(usize, usize)> {
        match self {
            IndicatorArgs::LengthLookbackArgs(a, b) => Ok((*a, *b)),
            _ => return Err(anyhow!(ERR_MSG)),
        }
    }

    #[allow(dead_code)]
    pub fn pmar_opt(&self) -> Option<(usize, MAType)> {
        match self {
            IndicatorArgs::PMARArgs(a, b) => Some((*a, *b)),
            _ => return None,
        }
    }

    #[allow(dead_code)]
    pub fn pmar_res(&self) -> Result<(usize, MAType)> {
        match self {
            IndicatorArgs::PMARArgs(a, b) => Ok((*a, *b)),
            _ => return Err(anyhow!(ERR_MSG)),
        }
    }

    #[allow(dead_code)]
    pub fn pmarp_opt(&self) -> Option<(usize, usize, MAType)> {
        match self {
            IndicatorArgs::PMARPArgs(a, b, c) => Some((*a, *b, *c)),
            _ => return None,
        }
    }

    #[allow(dead_code)]
    pub fn pmarp_res(&self) -> Result<(usize, usize, MAType)> {
        match self {
            IndicatorArgs::PMARPArgs(a, b, c) => Ok((*a, *b, *c)),
            _ => return Err(anyhow!(ERR_MSG)),
        }
    }

    #[allow(dead_code)]
    pub fn stochastic_res(&self) -> Result<(usize, usize, usize)> {
        match self {
            IndicatorArgs::StochasticArgs(a, b, c) => Ok((*a, *b, *c)),
            _ => return Err(anyhow!(ERR_MSG)),
        }
    }

    #[allow(dead_code)]
    pub fn stochastic_opt(&self) -> Option<(usize, usize, usize)> {
        match self {
            IndicatorArgs::StochasticArgs(a, b, c) => Some((*a, *b, *c)),
            _ => return None,
        }
    }
}
