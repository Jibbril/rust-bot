use anyhow::Result;
use reqwest::get;
use crate::{data_sources::bybit::rest::{
    api_responses::server_time::ServerTimeResponse,
    utils::bybit_url,
}, models::net_version::NetVersion};

pub async fn get_server_time(net: &NetVersion) -> Result<u64> {
    let url = bybit_url("/v5/market/time", net);

    let res = get(url).await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let resdata: ServerTimeResponse = res.json().await?;
            Ok(resdata.time)
        }
        _ => panic!("Unable to fetch server time."),
    }
}
