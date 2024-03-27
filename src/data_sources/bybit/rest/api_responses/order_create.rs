use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderCreateResponse {
    #[serde(rename = "retCode")]
    ret_code: u32,

    #[serde(rename = "retMsg")]
    ret_msg: String,

    pub result: Option<OrderCreateResult>,

    #[serde(rename = "retExtInfo")]
    ret_ext_info: Value,

    time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderCreateResult {
    #[serde(rename = "orderId")]
    pub order_id: String,

    #[serde(rename = "orderLinkId")]
    order_link_id: String,
}

