use actix::Message;

use crate::models::candle::Candle;

#[derive(Debug, Clone)]
pub struct WebsocketPayload {
    pub ok: bool,
    pub message: Option<String>,
    pub candle: Option<Candle>,
}

impl Message for WebsocketPayload {
    type Result = ();
}