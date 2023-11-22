use crate::models::candle::Candle;
use actix::Message;

#[derive(Debug, Clone)]
pub struct CandleAddedPayload {
    pub candle: Candle,
}

impl Message for CandleAddedPayload {
    type Result = ();
}
