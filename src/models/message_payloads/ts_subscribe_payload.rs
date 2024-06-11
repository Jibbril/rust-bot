use crate::models::message_payloads::candle_added_payload::CandleAddedPayload;
use actix::{Message, Recipient};

#[derive(Debug, Clone)]
pub struct TSSubscribePayload {
    pub observer: Recipient<CandleAddedPayload>,
}

impl Message for TSSubscribePayload {
    type Result = ();
}
