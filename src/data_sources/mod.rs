mod alpha_vantage;
mod local;

use crate::utils::{
    generic_result::GenericResult,
    timeseries::{Interval, TimeSeries},
};

// Available data sources
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataSource {
    AlphaVantage,
    Local(Box<DataSource>),
}

pub async fn request_data(
    source: &DataSource,
    symbol: &str,
    interval: Interval,
    save_local: bool,
) -> GenericResult<TimeSeries> {
    let ts = data_by_source(source, symbol, interval).await?;

    if save_local {
        match source {
            DataSource::Local(_) => (),
            _ => local::write(&ts, &source).await?,
        }
    }

    Ok(ts)
}

async fn data_by_source(
    source: &DataSource,
    symbol: &str,
    interval: Interval,
) -> GenericResult<TimeSeries> {
    let data: TimeSeries = match source {
        DataSource::AlphaVantage => alpha_vantage::get(symbol, &interval).await?,
        DataSource::Local(source) => local::read(&source, &symbol, &interval).await?,
    };

    Ok(data)
}
