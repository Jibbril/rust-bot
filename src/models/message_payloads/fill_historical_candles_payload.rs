use actix::Message;
use chrono::{DateTime, Utc};
use crate::models::interval::Interval;

#[derive(Debug, Clone)]
pub struct FillHistoricalCandlesPayload {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub symbol: String,
    pub interval: Interval,
}

impl Message for FillHistoricalCandlesPayload {
    type Result = ();
}
