use actix::Message;
use anyhow::Result;

use super::latest_candles_payload::LatestCandleResponse;

#[derive(Debug, Clone)]
pub struct RequestLatestCandlesPayload {
    pub n: usize,
}

impl Message for RequestLatestCandlesPayload {
    type Result = Result<LatestCandleResponse>;
}
