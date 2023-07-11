use crate::utils::{generic_result::GenericResult, timeseries::{TimeSeries, Interval}};

use super::DataSource;

pub async fn read(source: DataSource, symbol: &str) -> GenericResult<TimeSeries> {
    Ok(TimeSeries { 
        ticker: symbol.to_string(), 
        interval: Interval::Daily, 
        candles: Vec::new() 
    })
}

pub async fn write(ts: &TimeSeries) -> GenericResult<()>{

    Ok(())
}