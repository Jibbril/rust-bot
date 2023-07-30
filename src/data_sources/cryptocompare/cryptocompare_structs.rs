use serde::Deserialize;

#[allow(dead_code)] 
#[derive(Deserialize, Debug)]
pub struct CryptoCompareApiResponse {
    #[serde(rename = "Response")]
    response: String,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "HasWarning")]
    has_warning: bool,
    #[serde(rename = "Type")]
    type_: i32,
    #[serde(rename = "RateLimit")]
    rate_limit: serde_json::Value,
    #[serde(rename = "Data")]
    data: Data,
}

#[allow(dead_code)] 
#[derive(Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "Aggregated")]
    aggregated: bool,
    #[serde(rename = "TimeFrom")]
    time_from: u64,
    #[serde(rename = "TimeTo")]
    time_to: u64,
    #[serde(rename = "Data")]
    data: Vec<DataEntry>,
}

#[allow(dead_code)] 
#[derive(Deserialize, Debug)]
pub struct DataEntry {
    #[serde(rename = "time")]
    time: u64,
    #[serde(rename = "high")]
    high: f64,
    #[serde(rename = "low")]
    low: f64,
    #[serde(rename = "open")]
    open: f64,
    #[serde(rename = "volumefrom")]
    volume_from: f64,
    #[serde(rename = "volumeto")]
    volume_to: f64,
    #[serde(rename = "close")]
    close: f64,
    #[serde(rename = "conversionType")]
    conversion_type: String,
    #[serde(rename = "conversionSymbol")]
    conversion_symbol: String,
}

