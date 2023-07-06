use chrono::{DateTime, Utc, NaiveDate, NaiveDateTime, NaiveTime};
use super::generic_result::GenericResult;

#[derive(Debug,Clone)]
pub enum Interval {
    // Hour1,
    // Hour4,
    // Hour12,
    Daily,
    // Weekly,
}

#[derive(Debug,Clone)]
pub struct TimeSeries {
    pub ticker: String,
    pub interval: Interval,
    pub candles: Vec<Candle>
}

#[derive(Debug,Clone)]
pub struct Candle {
    pub timestamp: DateTime<Utc>,
    pub open: f32,
    pub close: f32,
    pub high: f32,
    pub low: f32,
    pub volume: f32
}

pub fn str_date_to_datetime(s: &str) -> GenericResult<DateTime<Utc>> {
    let time = NaiveTime::from_hms_opt(0,0,0).unwrap();
    let date = NaiveDate::parse_from_str(s, "%Y-%m-%d");
    
    match date {
        Ok(date) => {
            let datetime = NaiveDateTime::new(date, time);
            Ok(DateTime::from_utc(datetime, Utc))
        },
        Err(e) => Err(Box::new(e))
    }
}