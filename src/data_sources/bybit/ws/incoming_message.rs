use crate::{models::candle::Candle, utils::millis_to_datetime};
use anyhow::Result;
use serde::{de::Error, Deserialize, Deserializer};
use serde_json::{from_value, Value};
use std::collections::HashMap;

#[derive(Debug, Clone)]
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
            return Err(Error::custom(
                json["ret_msg"].as_str().unwrap_or("Unknown error."),
            ));
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
    pub success: bool,
    pub ret_msg: String,
    pub conn_id: String,
    pub op: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Subscribe {
    success: bool,
    ret_msg: String,
    conn_id: String,
    req_id: Option<String>,
    op: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct KlineResponse {
    #[serde(rename = "type")]
    type_: String,
    topic: String,
    ts: u64,
    data: Vec<Kline>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Kline {
    pub start: u64,
    pub end: u64,
    pub interval: String,
    pub open: String,
    pub close: String,
    pub high: String,
    pub low: String,
    pub volume: String,
    pub turnover: String,
    pub confirm: bool,
    pub timestamp: u64,
}

impl KlineResponse {
    pub fn get_kline(&self) -> Result<Kline> {
        let kline = &self.data[0];
        Ok(kline.clone())
    }
}

impl Kline {
    pub fn to_candle(&self) -> Result<Candle> {
        // End time for kline response is always 1 second before the
        // start time for the next candle.
        let timestamp = self.end - 59000;

        Ok(Candle {
            timestamp: millis_to_datetime(timestamp)?,
            open: self.open.parse::<f64>()?,
            close: self.close.parse::<f64>()?,
            high: self.high.parse::<f64>()?,
            low: self.low.parse::<f64>()?,
            volume: self.volume.parse::<f64>()?,
            indicators: HashMap::new(),
        })
    }
}
