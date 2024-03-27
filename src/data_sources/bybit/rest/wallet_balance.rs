use std::collections::HashMap;

use anyhow::{Result, Context};
use reqwest::Client;
use crate::{
    data_sources::bybit::rest::{
        api_responses::wallet_balance::{
            WalletBalance,
            WalletBalanceResponse
        },
        utils::{
            bybit_url,
            bybit_key,
            generate_hmac_signature
        }
    }, 
    utils::string::params_to_query_str, 
    models::net_version::NetVersion
};

pub async fn get(server_time: u64) -> Result<WalletBalance> {
    let mut params: HashMap<String,String> = HashMap::new();
    params.insert("accountType".to_string(), "UNIFIED".to_string());

    let param_str = params_to_query_str(&params);
    let base_url = bybit_url("/v5/account/wallet-balance", &NetVersion::Mainnet);
    let url = format!("{}?{}", base_url, param_str);
    let api_key = bybit_key()?;
    let recv_window = 5000;

    let signature = generate_hmac_signature(
        server_time,
        &api_key,
        recv_window,
        param_str
    )?;

    let client = Client::new();
    let res = client.get(url)
        .header("X-BAPI-SIGN", signature)
        .header("X-BAPI-API-KEY", api_key)
        .header("X-BAPI-SIGN-TYPE", "2")
        .header("X-BAPI-TIMESTAMP", server_time)
        .header("X-BAPI-RECV-WINDOW", recv_window)
        .send()
        .await?;

    let response: WalletBalanceResponse = match res.status() {
        reqwest::StatusCode::OK => res.json().await?,
        _ => panic!("Unable to fetch account balance")
    };

    let account_info = response
        .result
        .context("Unable to parse Wallet Balance Result")?
        .list
        .first()
        .cloned()
        .expect("Should return at least one account.");

    Ok(account_info)
}
