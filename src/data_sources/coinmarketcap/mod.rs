use crate::models::{generic_result::GenericResult, timeseries::TimeSeries};


pub async fn get() -> GenericResult<TimeSeries>{
    Ok(TimeSeries::dummy())
}