mod alpha_vantage;

use crate::utils::{generic_result::GenericResult, timeseries::TimeSeries};
use alpha_vantage::get;

// Available data sources
pub enum DataSource {
    AlphaVantage,
}

pub async fn request_data(datasource: DataSource, symbol: &str) -> GenericResult<()> {
    let data: TimeSeries = match datasource {
        DataSource::AlphaVantage => get(symbol).await?,
    };

    println!("Data:{:#?}", data);

    Ok(())
}
