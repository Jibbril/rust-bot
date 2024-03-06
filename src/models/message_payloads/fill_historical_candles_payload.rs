use crate::models::interval::Interval;
use actix::Message;

#[derive(Debug, Clone)]
pub struct FillHistoricalCandlesPayload {
    pub from: i64,
    pub to: i64,
    pub symbol: String,
    pub interval: Interval,
}

impl Message for FillHistoricalCandlesPayload {
    type Result = ();
}
