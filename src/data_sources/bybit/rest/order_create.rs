use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Map, Value, to_string};
use crate::{
    utils::{
        math::{round, floor}, 
        constants::BASE_CURRENCY,
    },
    data_sources::bybit::rest::{api_responses::{wallet_balance::WalletBalance, order_create::OrderCreateResponse}, server_time::get_server_time, utils::{bybit_key, generate_hmac_signature, bybit_url}}, models::net_version::NetVersion,
};

const ORDER_MAX_DECIMALS: i64 = 6;

pub async fn market_buy(quantity: f64, net: &NetVersion) -> Result<()> {
    let symbol = "BTCUSDT";
    let rounded_quantity = round(quantity, 2);

    let mut params = Map::new();
    params.insert("category".to_string(), json!("spot"));
    params.insert("symbol".to_string(), json!(symbol));
    params.insert("side".to_string(), json!("Buy"));
    params.insert("orderType".to_string(), json!("Market"));
    params.insert("marketUnit".to_string(), json!("quoteCoin"));
    params.insert("qty".to_string(), json!(rounded_quantity.to_string()));

    println!("params: {:#?}",params);

    Ok(post_market_order(params,net).await?)
}

pub async fn market_sell_all(account_info: &WalletBalance, net: &NetVersion) -> Result<()> {
    for coin in account_info.coin.iter() {
        // Skip selling of base currency
        if coin.coin == BASE_CURRENCY { continue; };

        let usd_value: f64 = coin.usd_value.parse()?;

        // Ignore small amounts
        if usd_value < 1.0 { continue; };

        let amount: f64 = coin.wallet_balance.parse()?;
        let amount = floor(amount, ORDER_MAX_DECIMALS);

        // Ignore extremely small quantities of the traded currency
        if amount == 0.0 { continue; };

        let symbol = format!("{}{}", coin.coin, BASE_CURRENCY);

        let mut params = Map::new();
        params.insert("category".to_string(), json!("spot"));
        params.insert("symbol".to_string(), json!(symbol));
        params.insert("side".to_string(), json!("Sell"));
        params.insert("orderType".to_string(), json!("Market"));
        params.insert("qty".to_string(), json!(amount.to_string()));
        params.insert("marketUnit".to_string(), json!("baseCoin"));

        post_market_order(params, net).await?;
    }
    
    Ok(())
}


async fn post_market_order(params: Map<String,Value>, net: &NetVersion) -> Result<()> {
    let client = Client::new();
    let timestamp = get_server_time(net).await?;
    let recv_window = 5000;
    let api_key = &bybit_key()?;

    let signature = generate_hmac_signature(
        timestamp, 
        &api_key, 
        recv_window, 
        to_string(&params)?
    )?;

    let res = client.post(bybit_url("/v5/order/create", net))
        .json(&params)
        .header("X-BAPI-SIGN", signature)
        .header("X-BAPI-API-KEY", api_key)
        .header("X-BAPI-SIGN-TYPE", "2")
        .header("X-BAPI-TIMESTAMP", timestamp)
        .header("X-BAPI-RECV-WINDOW", recv_window)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    // println!("Response Text: {:#?}",res.text().await?);

    let response: OrderCreateResponse = match res.status() {
        reqwest::StatusCode::OK => res.json().await?,
        _ => panic!("Unable to perform market buy")
    };

    println!("Create Response: {:#?}", response);

    Ok(())
}
