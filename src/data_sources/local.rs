use anyhow::Result;
use csv::Reader;

use crate::models::{candle::Candle, interval::Interval, timeseries::TimeSeries};

use std::{
    fs::{create_dir_all, File},
    path::Path,
};

use super::datasource::DataSource;

const FILE_NAME: &str = "data.csv";

pub async fn read(source: &DataSource, symbol: &str, interval: &Interval) -> Result<TimeSeries> {
    let path = construct_path(interval, symbol, source);
    let path = Path::new(&path).join(FILE_NAME);
    let file = File::open(&path)?;

    let mut reader = Reader::from_reader(file);
    let mut candles = Vec::new();

    for result in reader.deserialize() {
        let candle: Candle = result?;
        candles.push(candle);
    }

    Ok(TimeSeries::new(
        symbol.to_string(),
        interval.clone(),
        candles,
    ))
}

#[allow(dead_code)]
pub async fn write(ts: &TimeSeries, source: &DataSource) -> Result<()> {
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
        DataSource::Bybit => "bybit",
        DataSource::CoinMarketCap => "coinmarketcap",
        DataSource::CryptoCompare(_) => "cryptocompare",
    };

    let interval = match interval {
        Interval::Minute1 => "minute-1",
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
