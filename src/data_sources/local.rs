use csv::Reader;

use crate::models::{
    candle::Candle, generic_result::GenericResult, interval::Interval, timeseries::TimeSeries,
};

use super::DataSource;
use std::{
    collections::HashSet,
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
        indicators: HashSet::new(),
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
        DataSource::Bitfinex => "bitfinex",
        DataSource::CoinMarketCap => "coinmarketcap",
        DataSource::CryptoCompare(_) => "cryptocompare",
        DataSource::Local(_) => "local",
    };

    let interval = match interval {
        Interval::Minute5 => "minute-5",
        Interval::Minute15 => "minute-15",
        Interval::Minute30 => "minute-30",
        Interval::Hour1 => "hour-1",
        Interval::Hour4 => "hour-4",
        Interval::Hour12 => "hour-12",
        Interval::Day1 => "day-1",
        Interval::Day5 => "day-5",
        Interval::Week1 => "week-1",
    };

    format!("data/{}/{}/{}", source, ticker, interval)
}
