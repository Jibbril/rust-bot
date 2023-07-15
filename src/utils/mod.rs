use chrono::{DateTime, Utc, NaiveTime, NaiveDate, NaiveDateTime};

use self::generic_result::GenericResult;

pub mod calculation_mode;
pub mod generic_result;
pub mod timeseries;

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
