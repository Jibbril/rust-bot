use actix::Message;
use crate::models::candle::Candle;

#[derive(Debug, Clone)]
pub struct CandleAddedPayload {
    pub candle: Vec<Candle>,
}

impl Message for CandleAddedPayload {
    type Result = ();
}