use serde::{self, Deserialize, Deserializer, de::Error};
use serde_json::{Value, from_value};

#[allow(dead_code)]
#[derive(Debug)]
pub enum CryptoCompareWSMessage {
    StreamerWelcome(StreamerWelcome),
    Heartbeat(Heartbeat),
}

impl<'de> Deserialize<'de> for CryptoCompareWSMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json: Value = Deserialize::deserialize(deserializer)?;
        match json["TYPE"].as_str() {
            Some("20") => {
                let message = from_value(json).map_err(Error::custom)?;
                Ok(CryptoCompareWSMessage::StreamerWelcome(message))
            }
            Some("999") => {
                let message = from_value(json).map_err(Error::custom)?;
                Ok(CryptoCompareWSMessage::Heartbeat(message))
            }
            _ => Err(Error::custom("Unknown TYPE")),
        }
    }
}


#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct StreamerWelcome {
    #[serde(rename = "CLIENT_ID")]
    client_id: u64,
    #[serde(rename = "DATA_FORMAT")]
    data_format: String,
    #[serde(rename = "MESSAGE")]
    message: String,
    // ... other fields
    #[serde(rename = "RATELIMIT_MAX_DAY")]
    ratelimit_max_day: u32,
    #[serde(rename = "RATELIMIT_REMAINING_DAY")]
    ratelimit_remaining_day: u32,
    #[serde(rename = "SERVER_NAME")]
    server_name: String,
    #[serde(rename = "SERVER_TIME_MS")]
    server_time_ms: u64,
    #[serde(rename = "SERVER_UPTIME_SECONDS")]
    server_uptime_seconds: u64,
    #[serde(rename = "SOCKETS_ACTIVE")]
    sockets_active: u32,
    #[serde(rename = "SOCKETS_REMAINING")]
    sockets_remaining: u32,
    #[serde(rename = "SOCKET_ID")]
    socket_id: String,
    #[serde(rename = "TYPE")]
    type_: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Heartbeat {
    #[serde(rename = "MESSAGE")]
    message: String,
    #[serde(rename = "TIMEMS")]
    time_ms: u64,
    #[serde(rename = "TYPE")]
    type_: String,
}
