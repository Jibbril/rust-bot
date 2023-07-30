use csv::Reader;

use crate::models::{
    candle::Candle, generic_result::GenericResult, interval::Interval, timeseries::TimeSeries,
};

use super::DataSource;
use std::{
    fs::{create_dir_all, File},
    path::Path,
};

const FILE_NAME: &str = "data.csv";

pub async fn read(
    source: &DataSource,
    symbol: &str,
    interval: &Interval,
) -> GenericResult<TimeSeries> {
    let path = construct_path(interval, symbol, source);
    let path = Path::new(&path).join(FILE_NAME);
    let file = File::open(&path)?;

    let mut reader = Reader::from_reader(file);

    let mut candles = Vec::new();

    for result in reader.deserialize() {
        let candle: Candle = result?;
        candles.push(candle);
    }

    Ok(TimeSeries {
        ticker: symbol.to_string(),
        interval: interval.clone(),
        candles,
    })
}

pub async fn write(ts: &TimeSeries, source: &DataSource) -> GenericResult<()> {
    let path = construct_path(&ts.interval, &ts.ticker, source);
    let path = Path::new(&path);

    create_dir_all(&path)?;

    let file = File::create(path.join(FILE_NAME))?;

    let mut writer = csv::Writer::from_writer(file);

    for candle in ts.candles.iter() {
        writer.serialize(candle)?;
    }

    writer.flush()?;

    Ok(())
}

fn construct_path(interval: &Interval, ticker: &str, source: &DataSource) -> String {
    let source = match source {
        DataSource::AlphaVantage => "alphavantage",
        DataSource::CoinMarketCap => "coinmarketcap",
        DataSource::CryptoCompare => "cryptocompare",
        DataSource::Local(_) => "local",
    };

    let interval = match interval {
        Interval::Daily => "daily",
    };

    format!("data/{}/{}/{}", source, ticker, interval)
}
