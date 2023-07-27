use crate::{
    models::{
        generic_result::GenericResult,
        setup::{FindsReverseSetups, FindsSetups, Setup},
        timeseries::TimeSeries,
    },
    trading_strategies::{rsi_basic::RsiBasic, silver_cross::SilverCross},
};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Strategy {
    RsiBasic(RsiBasic),
    SilverCross(SilverCross),
}

impl Display for Strategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::RsiBasic(s) => write!(f, "{}", s),
            Self::SilverCross(s) => write!(f, "{}", s),
        }
    }
}

impl FindsSetups for Strategy {
    fn find_setups(&self, ts: &TimeSeries) -> GenericResult<Vec<Setup>> {
        match self {
            Self::RsiBasic(rsi) => rsi.find_setups(ts),
            Self::SilverCross(sc) => sc.find_setups(ts),
        }
    }
}

impl FindsReverseSetups for Strategy {
    fn find_reverse_setups(&self, ts: &TimeSeries) -> GenericResult<Vec<Setup>> {
        match self {
            Self::RsiBasic(rsi) => rsi.find_reverse_setups(ts),
            _ => Ok(Vec::new()),
        }
    }
}
