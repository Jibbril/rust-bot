use crate::models::{timeseries::TimeSeries, generic_result::GenericResult};

pub trait PopulatesCandles {
    fn populate_candles(ts: &mut TimeSeries, args: IndicatorArgs) -> GenericResult<()>;
    fn populate_candles_default(ts: &mut TimeSeries) -> GenericResult<()>;
}

#[derive(Debug,Clone, Copy)]
pub enum IndicatorArgs {
    LengthArg(usize),
    BollingerBandArgs(usize,f64), // length, n-standard deviations
    BBWPArgs(usize,usize,usize), // bbwp-length, lookback, sma-length
}

const ERR: &str = "Invalid indicator arguments.";

impl IndicatorArgs {
    #[allow(dead_code)]
    pub fn extract_length_arg_opt(&self) -> Option<usize> {
        match self {
            IndicatorArgs::LengthArg(n) => Some(*n),
            _ => return None
        }
    }

    pub fn extract_length_arg_res(&self) -> GenericResult<usize> {
        match self {
            IndicatorArgs::LengthArg(n) => Ok(*n),
            _ => return Err(ERR.into())
        }
    }
    
    pub fn extract_bb_args_opt(&self) -> Option<(usize, f64)> {
        match self {
            IndicatorArgs::BollingerBandArgs(n, m) => Some((*n,*m)),
            _ => return None
        }
    }

    pub fn extract_bb_args_res(&self) -> GenericResult<(usize, f64)> {
        match self {
            IndicatorArgs::BollingerBandArgs(n, m) => Ok((*n,*m)),
            _ => return Err(ERR.into())
        }
    }

    #[allow(dead_code)]
    pub fn extract_bbwp_args_opt(&self) -> Option<(usize,usize, usize)> {
        match self {
            IndicatorArgs::BBWPArgs(a,b,c) => Some((*a,*b,*c)),
            _ => return None
        }
    }

    pub fn extract_bbwp_args_res(&self) -> GenericResult<(usize,usize, usize)> {
        match self {
            IndicatorArgs::BBWPArgs(a,b,c) => Ok((*a,*b,*c)),
            _ => return Err(ERR.into())
        }
    }
}