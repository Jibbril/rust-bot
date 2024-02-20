use crate::models::candle::Candle;
use actix::Message;

#[derive(Debug, Clone)]
pub struct AddCandlesPayload {
    pub candles: Vec<Candle>,
}

impl Message for AddCandlesPayload {
    type Result = ();
}
