use super::setup::Setup;
use crate::models::timeseries::TimeSeries;
use anyhow::Result;

pub trait FindsSetups {
    fn find_setups(&self, ts: &TimeSeries) -> Result<Vec<Setup>>;
}

pub trait FindsReverseSetups {
    fn find_reverse_setups(&self, ts: &TimeSeries) -> Result<Vec<Setup>>;
}
