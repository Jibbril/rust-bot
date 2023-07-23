pub mod math;
use crate::models::generic_result::GenericResult;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};

pub fn str_date_to_datetime(s: &str) -> GenericResult<DateTime<Utc>> {
    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let date = NaiveDate::parse_from_str(s, "%Y-%m-%d");

    match date {
        Ok(date) => {
            let datetime = NaiveDateTime::new(date, time);
            Ok(DateTime::from_utc(datetime, Utc))
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub fn length_or_one<T>(arr: &[T]) -> usize {
    if !arr.is_empty() {
        arr.len()
    } else {
        1
    }
}

pub fn f_length_or_one<T>(arr: &[T]) -> f64 {
    length_or_one(arr) as f64
}
