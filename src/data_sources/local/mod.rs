use super::DataSource;
use crate::utils::{
    generic_result::GenericResult,
    timeseries::{Interval, TimeSeries},
};
use std::{
    fs::{create_dir_all, File},
    path::Path,
};

pub async fn read(source: &DataSource, symbol: &str) -> GenericResult<TimeSeries> {
    Ok(TimeSeries {
        ticker: symbol.to_string(),
        interval: Interval::Daily,
        candles: Vec::new(),
    })
}

pub async fn write(ts: &TimeSeries, source: &DataSource) -> GenericResult<()> {
    let path = construct_path(ts, source);
    let path = Path::new(&path);

    create_dir_all(&path)?;

    let file = File::create(path.join("data.csv"))?;

    let mut writer = csv::Writer::from_writer(file);

    for candle in ts.candles.iter() {
        writer.serialize(candle)?;
    }

    writer.flush()?;

    Ok(())
}

fn construct_path(ts: &TimeSeries, source: &DataSource) -> String {
    let source = match source {
        DataSource::AlphaVantage => "alpha_vantage",
        DataSource::Local(_) => "local",
    };

    let interval = match ts.interval {
        Interval::Daily => "daily",
    };

    format!("data/{}/{}/{}", source, ts.ticker, interval)
}
