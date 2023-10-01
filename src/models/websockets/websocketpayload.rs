use crate::models::candle::Candle;

#[derive(Debug, Clone)]
pub struct WebsocketPayload {
    pub ok: bool,
    pub message: Option<String>,
    pub candle: Option<Candle>, 
}

