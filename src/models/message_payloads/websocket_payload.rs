use crate::models::candle::Candle;
use actix::Message;

#[derive(Debug, Clone)]
pub struct WebsocketPayload {
    pub ok: bool,
    pub message: Option<String>,
    pub candle: Option<Candle>,
}

impl Message for WebsocketPayload {
    type Result = ();
}
