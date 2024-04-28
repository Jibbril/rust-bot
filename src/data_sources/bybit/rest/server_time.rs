use crate::{
    data_sources::bybit::rest::{api_responses::server_time::ServerTimeResponse, utils::bybit_url},
    models::net_version::NetVersion,
};
use anyhow::Result;
use reqwest::get;

pub async fn get_server_time() -> Result<u64> {
    let url = bybit_url("/v5/market/time", &NetVersion::Mainnet);

    let res = get(url).await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let resdata: ServerTimeResponse = res.json().await?;
            Ok(resdata.time)
        }
        _ => panic!("Unable to fetch server time."),
    }
}
