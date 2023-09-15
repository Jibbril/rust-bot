pub mod math;

use crate::models::{generic_result::GenericResult, setup::Setup};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use std::{
    fs::{create_dir_all, File},
    path::Path,
};

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

pub fn millis_to_datetime(timestamp: u64) -> GenericResult<DateTime<Utc>> {
    Utc.timestamp_opt((timestamp / 1000) as i64, 0)
        .single()
        .ok_or_else(|| format!("Invalid timestamp: {}", timestamp).into())
}

pub fn secs_to_datetime(timestamp: u64) -> GenericResult<DateTime<Utc>> {
    millis_to_datetime(timestamp * 1000)
}

pub fn len_or_one<T>(arr: &[T]) -> usize {
    if !arr.is_empty() {
        arr.len()
    } else {
        1
    }
}

pub fn f_len_or_one<T>(arr: &[T]) -> f64 {
    len_or_one(arr) as f64
}

pub fn save_setups(setups: &[Setup], name: &str) -> GenericResult<()> {
    let folder_path = "data/temp/setups";
    let folder_path = Path::new(&folder_path);

    create_dir_all(&folder_path)?;

    let file = File::create(folder_path.join(name))?;

    let mut writer = csv::Writer::from_writer(file);

    for setup in setups {
        writer.serialize(setup.to_csv_row())?;
    }

    writer.flush()?;

    Ok(())
}
