use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Interval {
    // Hour1,
    // Hour4,
    // Hour12,
    Daily,
    // Weekly,
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            Interval::Daily => write!(f, "Daily"),
        }
    }
}
