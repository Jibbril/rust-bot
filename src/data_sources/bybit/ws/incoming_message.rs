use serde::{Deserialize, Deserializer, de::Error};
use serde_json::{Value, from_value};

#[derive(Debug,Clone)]
pub enum IncomingMessage {
    Pong(Pong),
}

impl<'de> Deserialize<'de> for IncomingMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json: Value = Deserialize::deserialize(deserializer)?;
        match json["ret_msg"].as_str() {
            Some("pong") => {
                let message = from_value(json).map_err(Error::custom)?;
                Ok(IncomingMessage::Pong(message))
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