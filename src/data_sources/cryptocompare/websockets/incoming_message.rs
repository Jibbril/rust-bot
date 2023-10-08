use serde::{self, de::Error, Deserialize, Deserializer};
use serde_json::{from_value, Value};

#[allow(dead_code)]
#[derive(Debug)]
pub enum IncomingMessage {
    StreamerWelcome(StreamerWelcome),
    Heartbeat(Heartbeat),
    AggregateIndex(AggregateIndex),
    SubscribeComplete(SubscribeComplete),
    LoadComplete(LoadComplete),
    UnsubscribeComplete(UnsubscribeComplete),
    UnsubscribeAllComplete(UnsubscribeAllComplete),
}

impl<'de> Deserialize<'de> for IncomingMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json: Value = Deserialize::deserialize(deserializer)?;
        match json["TYPE"].as_str() {
            Some("3") => {
                let message = from_value(json).map_err(Error::custom)?;
                Ok(IncomingMessage::LoadComplete(message))
            }
            Some("5") => {
                let message = from_value(json).map_err(Error::custom)?;
                Ok(IncomingMessage::AggregateIndex(message))
            }
            Some("16") => {
                let message = from_value(json).map_err(Error::custom)?;
                Ok(IncomingMessage::SubscribeComplete(message))
            }
            Some("17") => {
                let message = from_value(json).map_err(Error::custom)?;
                Ok(IncomingMessage::UnsubscribeComplete(message))
            }
            Some("18") => {
                let message = from_value(json).map_err(Error::custom)?;
                Ok(IncomingMessage::UnsubscribeAllComplete(message))
            }
            Some("20") => {
                let message = from_value(json).map_err(Error::custom)?;
                Ok(IncomingMessage::StreamerWelcome(message))
            }
            Some("999") => {
                let message = from_value(json).map_err(Error::custom)?;
                Ok(IncomingMessage::Heartbeat(message))
            }
            Some("500") => match json["MESSAGE"].as_str() {
                Some("INVALID_JSON") => Err(Error::custom("Invalid JSON")),
                Some("SUBSCRIPTION_UNRECOGNIZED") => {
                    Err(Error::custom("Subscription unrecognized"))
                }
                _ => Err(Error::custom("Unknown ERROR")),
            },
            _ => Err(Error::custom("Unknown TYPE")),
        }
    }
}

// ========================================================================
// ======================== Incoming message types ========================
// ========================================================================
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

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct AggregateIndex {
    #[serde(rename = "CIRCULATINGSUPPLY")]
    circulating_supply: Option<f64>,
    #[serde(rename = "CIRCULATINGSUPPLYMKTCAP")]
    circulating_supply_mkt_cap: Option<f64>,
    #[serde(rename = "CURRENTSUPPLY")]
    current_supply: Option<f64>,
    #[serde(rename = "CURRENTSUPPLYMKTCAP")]
    current_supply_mkt_cap: Option<f64>,
    #[serde(rename = "FLAGS")]
    flags: Option<i64>,
    #[serde(rename = "FROMSYMBOL")]
    from_symbol: Option<String>,
    #[serde(rename = "HIGH24HOUR")]
    high_24_hour: Option<f64>,
    #[serde(rename = "HIGHDAY")]
    high_day: Option<f64>,
    #[serde(rename = "HIGHHOUR")]
    high_hour: Option<f64>,
    #[serde(rename = "LASTMARKET")]
    last_market: Option<String>,
    #[serde(rename = "LASTTRADEID")]
    last_trade_id: Option<String>,
    #[serde(rename = "LASTUPDATE")]
    last_update: Option<i64>,
    #[serde(rename = "LASTVOLUME")]
    last_volume: Option<f64>,
    #[serde(rename = "LASTVOLUMETO")]
    last_volume_to: Option<f64>,
    #[serde(rename = "LOW24HOUR")]
    low_24_hour: Option<f64>,
    #[serde(rename = "LOWDAY")]
    low_day: Option<f64>,
    #[serde(rename = "LOWHOUR")]
    low_hour: Option<f64>,
    #[serde(rename = "MARKET")]
    market: Option<String>,
    #[serde(rename = "MAXSUPPLY")]
    max_supply: Option<f64>,
    #[serde(rename = "MAXSUPPLYMKTCAP")]
    max_supply_mkt_cap: Option<f64>,
    #[serde(rename = "MEDIAN")]
    median: Option<f64>,
    #[serde(rename = "MKTCAPPENALTY")]
    mkt_cap_penalty: Option<f64>,
    #[serde(rename = "OPEN24HOUR")]
    open_24_hour: Option<f64>,
    #[serde(rename = "OPENDAY")]
    open_day: Option<f64>,
    #[serde(rename = "OPENHOUR")]
    open_hour: Option<f64>,
    #[serde(rename = "PRICE")]
    price: Option<f64>,
    #[serde(rename = "TOPTIERVOLUME24HOUR")]
    top_tier_volume_24_hour: Option<f64>,
    #[serde(rename = "TOPTIERVOLUME24HOURTO")]
    top_tier_volume_24_hour_to: Option<f64>,
    #[serde(rename = "TOSYMBOL")]
    to_symbol: Option<String>,
    #[serde(rename = "TYPE")]
    type_: Option<String>,
    #[serde(rename = "VOLUME24HOUR")]
    volume_24_hour: Option<f64>,
    #[serde(rename = "VOLUME24HOURTO")]
    volume_24_hour_to: Option<f64>,
    #[serde(rename = "VOLUMEDAY")]
    volume_day: Option<f64>,
    #[serde(rename = "VOLUMEDAYTO")]
    volume_day_to: Option<f64>,
    #[serde(rename = "VOLUMEHOUR")]
    volume_hour: Option<f64>,
    #[serde(rename = "VOLUMEHOURTO")]
    volume_hour_to: Option<f64>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct SubscribeComplete {
    #[serde(rename = "MESSAGE")]
    message: String,
    #[serde(rename = "SUB")]
    sub: String,
    #[serde(rename = "TYPE")]
    type_: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct LoadComplete {
    #[serde(rename = "INFO")]
    info: String,
    #[serde(rename = "MESSAGE")]
    message: String,
    #[serde(rename = "TYPE")]
    type_: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct UnsubscribeComplete {
    #[serde(rename = "MESSAGE")]
    message: String,
    #[serde(rename = "SUB")]
    sub: String,
    #[serde(rename = "TYPE")]
    type_: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct UnsubscribeAllComplete {
    #[serde(rename = "INFO")]
    info: String,
    #[serde(rename = "INFO_OBJ")]
    info_obj: InfoObj,
    #[serde(rename = "MESSAGE")]
    message: String,
    #[serde(rename = "TYPE")]
    type_: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct InfoObj {
    #[serde(rename = "invalid")]
    invalid: i64,
    #[serde(rename = "valid")]
    valid: i64,
}
