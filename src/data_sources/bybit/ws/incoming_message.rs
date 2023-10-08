use serde::{Deserialize, Deserializer, de::Error};
use serde_json::{Value, from_value};

#[derive(Debug,Clone)]
pub enum IncomingMessage {
    Pong(Pong),
    Subscribe(Subscribe),
    Kline(KlineResponse),
}

impl<'de> Deserialize<'de> for IncomingMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json: Value = Deserialize::deserialize(deserializer)?;

        if let Some("400") = json["ret_code"].as_str() {
            return Err(Error::custom(json["ret_msg"].as_str().unwrap_or("Unknown error.")));
        }

        if let Some("snapshot") = json["type"].as_str() {
            // TODO: Support other types of snapshots
            let message = from_value(json).map_err(Error::custom)?;
            return Ok(IncomingMessage::Kline(message));
        }

        match json["ret_msg"].as_str() {
            Some("pong") => {
                let message = from_value(json).map_err(Error::custom)?;
                Ok(IncomingMessage::Pong(message))
            }
            Some("subscribe") => {
                let message = from_value(json).map_err(Error::custom)?;
                Ok(IncomingMessage::Subscribe(message))
            }
            _ => Err(Error::custom("Unknown operation.")),
        }
    }
}

// ========================================================================
// ======================== Incoming message types ========================
// ========================================================================
#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Pong {
    success: bool,
    ret_msg: String,
    conn_id: String,
    op: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Subscribe {
    success: bool,
    ret_msg: String,
    conn_id: String,
    req_id: Option<String>,
    op: String
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct KlineResponse {
    #[serde(rename = "type")]
    type_: String,
    topic: String,
    ts: u64,
    data: Vec<Kline>
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Kline {
    start: u64,
    end: u64,
    interval: String,
    open: String,
    close: String,
    high: String,
    low: String,
    volume: String,
    turnover: String,
    confirm: bool,
    timestamp: u64
}
