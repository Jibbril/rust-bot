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
