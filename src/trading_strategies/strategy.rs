use super::rsi_basic::RsiBasic;
use crate::models::{generic_result::GenericResult, timeseries::TimeSeries, setup::{FindsSetups, Setup, FindsReverseSetups}};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Strategy {
    RsiBasic(RsiBasic),
}

impl Display for Strategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::RsiBasic(s) => write!(f, "{}", s),
        }
    }
}

impl FindsSetups for Strategy {
    fn find_setups(&self, ts: &TimeSeries) -> GenericResult<Vec<Setup>> {
        match self {
            Self::RsiBasic(rsi) => rsi.find_setups(ts),
        }
    }
}

impl FindsReverseSetups for Strategy {
    fn find_reverse_setups(&self, ts: &TimeSeries) -> GenericResult<Vec<Setup>> {
        match self {
            Self::RsiBasic(rsi) => rsi.find_reverse_setups(ts),
        }
    }
}
