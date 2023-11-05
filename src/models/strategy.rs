use anyhow::Result;

use super::{
    setups::{
        finds_setups::{FindsReverseSetups, FindsSetups},
        setup::Setup,
    },
    traits::has_max_length::HasMaxLength,
};
use crate::{
    models::timeseries::TimeSeries,
    trading_strategies::{rsi_basic::RsiBasic, silver_cross::SilverCross},
};
use std::fmt::{Display, Formatter};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Strategy {
    RsiBasic(RsiBasic),
    SilverCross(SilverCross),
}

impl Display for Strategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RsiBasic(s) => write!(f, "{}", s),
            Self::SilverCross(s) => write!(f, "{}", s),
        }
    }
}

impl FindsSetups for Strategy {
    fn find_setups(&self, ts: &TimeSeries) -> Result<Vec<Setup>> {
        match self {
            Self::RsiBasic(rsi) => rsi.find_setups(ts),
            Self::SilverCross(sc) => sc.find_setups(ts),
        }
    }
}

impl FindsReverseSetups for Strategy {
    fn find_reverse_setups(&self, ts: &TimeSeries) -> Result<Vec<Setup>> {
        match self {
            Self::RsiBasic(rsi) => rsi.find_reverse_setups(ts),
            _ => Ok(Vec::new()),
        }
    }
}

impl HasMaxLength for Strategy {
    fn max_length(&self) -> usize {
        match self {
            Self::RsiBasic(rsi) => rsi.max_length(),
            Self::SilverCross(sc) => sc.max_length(),
        }
    }
}
