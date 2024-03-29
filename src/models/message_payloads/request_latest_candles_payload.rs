use crate::models::message_payloads::latest_candles_payload::LatestCandleResponse;
use actix::Message;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct RequestLatestCandlesPayload {
    pub n: usize,
}

impl Message for RequestLatestCandlesPayload {
    type Result = Result<LatestCandleResponse>;
}
