use crate::models::interval::Interval;
use anyhow::{anyhow, Result};

#[allow(dead_code)]
pub fn interval_to_str(interval: &Interval) -> Result<String> {
    let interval = match interval {
        Interval::Minute1 => "1",
        Interval::Minute5 => "5",
        Interval::Minute15 => "15",
        Interval::Minute30 => "30",
        Interval::Hour1 => "60",
        Interval::Hour4 => "240",
        Interval::Day1 => "D",
        Interval::Week1 => "W",
        // Interval::Month1 => "1M",
        _ => return Err(anyhow!("Bybit does not support this interval.")),
    };

    Ok(interval.to_string())
}

#[allow(dead_code)]
pub fn str_to_interval(interval_str: &str) -> Result<Interval> {
    match interval_str {
        "1" => Ok(Interval::Minute1),
        "5" => Ok(Interval::Minute5),
        "15" => Ok(Interval::Minute15),
        "30" => Ok(Interval::Minute30),
        "60" => Ok(Interval::Hour1),
        "240" => Ok(Interval::Hour4),
        "D" => Ok(Interval::Day1),
        "W" => Ok(Interval::Week1),
        // "1M" => Ok(Interval::Month1),
        _ => Err(anyhow!("Unsupported interval string.")),
    }
}
