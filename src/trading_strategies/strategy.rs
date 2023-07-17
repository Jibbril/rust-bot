use super::{
    rsi_basic::RsiBasic,
    setup::{FindsSetups, Setup},
};
use crate::models::{generic_result::GenericResult, timeseries::TimeSeries};
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
