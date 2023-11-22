use crate::models::candle::Candle;
use actix::Message;

#[derive(Debug, Clone)]
pub struct RequestLatestCandlesPayload {
    pub n: usize,
}

impl Message for RequestLatestCandlesPayload {
    type Result = Vec<Candle>;
}
