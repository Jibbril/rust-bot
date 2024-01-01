use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Interval {
    Minute1,
    Minute5,
    Minute15,
    Minute30,
    Hour1,
    Hour4,
    Hour12,
    Day1,
    Day5,
    Week1,
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            Interval::Minute1 => write!(f, "1 Minute"),
            Interval::Minute5 => write!(f, "5 Minute"),
            Interval::Minute15 => write!(f, "15 Minute"),
            Interval::Minute30 => write!(f, "30 Minute"),
            Interval::Hour1 => write!(f, "Hourly"),
            Interval::Hour4 => write!(f, "4 Hour"),
            Interval::Hour12 => write!(f, "12 Hour"),
            Interval::Day1 => write!(f, "Daily"),
            Interval::Day5 => write!(f, "5 Day"),
            Interval::Week1 => write!(f, "Weekly"),
        }
    }
}

impl Interval {
    pub fn to_duration(&self) -> Duration {
        match self {
            Interval::Minute1 => Duration::minutes(1),
            Interval::Minute5 => Duration::minutes(5),
            Interval::Minute15 => Duration::minutes(15),
            Interval::Minute30 => Duration::minutes(30),
            Interval::Hour1 => Duration::hours(1),
            Interval::Hour4 => Duration::hours(4),
            Interval::Hour12 => Duration::hours(12),
            Interval::Day1 => Duration::days(1),
            Interval::Day5 => Duration::days(5),
            Interval::Week1 => Duration::weeks(1),
        }
    }

    /// Returns the acceptable difference in duration to still consider candles
    /// subsequent in a timeseries.
    pub fn max_diff(&self) -> Duration {
        match self {
            Interval::Minute1 => Duration::seconds(1),
            Interval::Minute5 => Duration::seconds(5),
            Interval::Minute15 => Duration::seconds(5),
            Interval::Minute30 => Duration::seconds(5),
            Interval::Hour1 => Duration::minutes(1),
            Interval::Hour4 => Duration::minutes(1),
            Interval::Hour12 => Duration::minutes(1),
            Interval::Day1 => Duration::hours(1),
            Interval::Day5 => Duration::hours(1),
            Interval::Week1 => Duration::hours(1),
        }
    }
}
