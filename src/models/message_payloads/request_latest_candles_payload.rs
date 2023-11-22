use actix::Message;
use crate::models::candle::Candle;

#[derive(Debug, Clone)]
pub struct RequestLatestCandlesPayload {
    pub n: usize
}

impl Message for RequestLatestCandlesPayload {
    type Result = Vec<Candle>;
}
