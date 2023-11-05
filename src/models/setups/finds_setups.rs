use anyhow::Result;
use crate::models::timeseries::TimeSeries;
use super::setup::Setup;

pub trait FindsSetups {
    fn find_setups(&self, ts: &TimeSeries) -> Result<Vec<Setup>>;
}

pub trait FindsReverseSetups {
    fn find_reverse_setups(&self, ts: &TimeSeries) -> Result<Vec<Setup>>;
}
