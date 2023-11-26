use actix::Message;
use crate::models::{interval::Interval, candle::Candle};

#[allow(dead_code)]
pub struct LatestCandleResponse {
    pub symbol: String,
    pub interval: Interval,
    pub candles: Vec<Candle>,
}

impl Message for LatestCandleResponse {
    type Result = ();
}

#[allow(dead_code)]
impl LatestCandleResponse {
    pub fn new(symbol: String, interval: Interval, candles: Vec<Candle>) -> Self {
        LatestCandleResponse {
            symbol,
            interval,
            candles,
        }
    }
}