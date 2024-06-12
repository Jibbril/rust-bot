use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderCreateResponse {
    #[serde(rename = "retCode")]
    pub ret_code: u32,

    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    pub result: Option<OrderCreateResult>,

    #[serde(rename = "retExtInfo")]
    ret_ext_info: Value,

    time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderCreateResult {
    #[serde(rename = "orderId")]
    pub order_id: Option<String>,

    #[serde(rename = "orderLinkId")]
    order_link_id: Option<String>,
}
