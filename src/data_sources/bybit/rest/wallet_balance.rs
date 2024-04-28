use std::collections::HashMap;

use crate::{
    data_sources::bybit::rest::{
        api_responses::wallet_balance::WalletBalanceResponse,
        bybit_rest_api::BybitRestApi,
        utils::{bybit_key, bybit_url, generate_hmac_signature},
    },
    models::{net_version::NetVersion, wallet::Wallet},
    utils::string::params_to_query_str,
};
use anyhow::{Context, Result};
use reqwest::Client;

pub async fn get() -> Result<Wallet> {
    let server_time = BybitRestApi::get_server_time().await?;
    let mut params: HashMap<String, String> = HashMap::new();
    params.insert("accountType".to_string(), "UNIFIED".to_string());

    let param_str = params_to_query_str(&params);
    let base_url = bybit_url("/v5/account/wallet-balance", &NetVersion::Mainnet);
    let url = format!("{}?{}", base_url, param_str);
    let api_key = bybit_key()?;
    let recv_window = 5000;

    let signature = generate_hmac_signature(server_time, &api_key, recv_window, param_str)?;

    let client = Client::new();
    let res = client
        .get(url)
        .header("X-BAPI-SIGN", signature)
        .header("X-BAPI-API-KEY", api_key)
        .header("X-BAPI-SIGN-TYPE", "2")
        .header("X-BAPI-TIMESTAMP", server_time)
        .header("X-BAPI-RECV-WINDOW", recv_window)
        .send()
        .await?;

    let response: WalletBalanceResponse = match res.status() {
        reqwest::StatusCode::OK => res.json().await?,
        _ => panic!("Unable to fetch account balance"),
    };

    let wallet_balance = response
        .result
        .context("Unable to parse Wallet Balance Result")?
        .list
        .first()
        .cloned()
        .expect("Should return at least one account.");

    Ok(wallet_balance.to_wallet()?)
}
