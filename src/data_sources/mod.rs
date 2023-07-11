mod alpha_vantage;
mod local;

use crate::utils::{generic_result::GenericResult, timeseries::{TimeSeries, Interval}};

// Available data sources
pub enum DataSource {
    AlphaVantage,
    Local(Box<DataSource>),
}

pub async fn request_data(source: DataSource, symbol: &str, interval: Interval, save_local: bool) -> GenericResult<TimeSeries> {
    let ts = data_by_source(source, symbol,interval).await?;

    if save_local {
        local::write(&ts).await;
    }

    Ok(ts)
}


async fn data_by_source(source: DataSource, symbol: &str, interval: Interval) -> GenericResult<TimeSeries> {
    let data: TimeSeries = match source {
        DataSource::AlphaVantage => {
            let ts = alpha_vantage::get(symbol, interval).await?;
            ts
        },
        DataSource::Local(source) => {
            local::read(*source,symbol).await?
        },
    };

    Ok(data)
}