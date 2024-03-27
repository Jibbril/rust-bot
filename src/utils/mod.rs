pub mod data;
pub mod math;
pub mod string;
pub mod constants;

use crate::models::setups::setup::Setup;
use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use std::{
    fs::{create_dir_all, File},
    path::Path,
};

pub fn str_date_to_datetime(s: &str) -> Result<DateTime<Utc>> {
    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let date = NaiveDate::parse_from_str(s, "%Y-%m-%d");

    match date {
        Ok(date) => {
            let datetime = NaiveDateTime::new(date, time);
            Ok(DateTime::from_utc(datetime, Utc))
        }
        Err(e) => Err(anyhow!(e)),
    }
}

pub fn millis_to_datetime(timestamp: u64) -> Result<DateTime<Utc>> {
    Utc.timestamp_opt((timestamp / 1000) as i64, 0)
        .single()
        .context(format!("Invalid timestamp: {}", timestamp))
}

pub fn secs_to_datetime(timestamp: u64) -> Result<DateTime<Utc>> {
    millis_to_datetime(timestamp * 1000)
}

#[allow(dead_code)]
pub fn len_or_one<T>(arr: &[T]) -> usize {
    if !arr.is_empty() {
        arr.len()
    } else {
        1
    }
}

#[allow(dead_code)]
pub fn f_len_or_one<T>(arr: &[T]) -> f64 {
    len_or_one(arr) as f64
}

pub fn save_setups(setups: &[Setup], name: &str) -> Result<()> {
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
