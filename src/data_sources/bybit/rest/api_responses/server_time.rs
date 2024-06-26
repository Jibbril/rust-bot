use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerTimeResponse {
    #[serde(rename = "retCode")]
    ret_code: i32,

    #[serde(rename = "retMsg")]
    ret_msg: String,

    result: Option<ServerTimeResult>,

    #[serde(rename = "retExtInfo")]
    ret_ext_info: Value,

    pub time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerTimeResult {
    #[serde(rename = "timeSecond")]
    time_second: String,

    #[serde(rename = "timeNano")]
    time_nano: String,
}
