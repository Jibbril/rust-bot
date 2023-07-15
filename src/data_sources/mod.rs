use crate::models::{generic_result::GenericResult, interval::Interval, timeseries::TimeSeries};

mod alpha_vantage;
mod local;

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
    let ts: TimeSeries = match source {
        DataSource::AlphaVantage => alpha_vantage::get(symbol, &interval).await?,
        DataSource::Local(source) => local::read(&source, &symbol, &interval).await?,
    };

    if save_local {
        match source {
            DataSource::Local(_) => (),
            _ => local::write(&ts, &source).await?,
        }
    }

    Ok(ts)
}
