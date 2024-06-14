use crate::{
    data_sources::bybit::rest::{
        api_responses::order_create::OrderCreateResponse,
        server_time::get_server_time,
        utils::{bybit_key, bybit_url, generate_hmac_signature},
    },
    models::{net_version::NetVersion, wallet::Wallet},
    utils::{
        constants::BASE_CURRENCY,
        math::{floor, round},
    },
};
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde_json::{json, to_string, Map, Value};

const ORDER_MAX_DECIMALS: i64 = 6;

pub async fn market_buy(symbol: &str, quantity: f64) -> Result<()> {
    let rounded_quantity = round(quantity, 2);

    let mut params = Map::new();
    params.insert("category".to_string(), json!("spot"));
    params.insert("symbol".to_string(), json!(symbol));
    params.insert("side".to_string(), json!("Buy"));
    params.insert("orderType".to_string(), json!("Market"));
    params.insert("marketUnit".to_string(), json!("quoteCoin"));
    params.insert("qty".to_string(), json!(rounded_quantity.to_string()));
    println!("buy params: {:#?}", params);

    Ok(post_market_order(params, &NetVersion::Mainnet).await?)
}

pub async fn market_sell_all(wallet: &Wallet) -> Result<()> {
    for coin in wallet.coins.values() {
        // Skip selling of base currency
        if coin.symbol == BASE_CURRENCY {
            continue;
        };

        // Ignore small amounts
        if coin.usd_value < 1.0 {
            continue;
        };

        let quantity: f64 = coin.quantity;
        let quantity = floor(quantity, ORDER_MAX_DECIMALS);

        // Ignore extremely small quantities of the traded currency
        if quantity == 0.0 {
            continue;
        };

        let symbol = format!("{}{}", coin.symbol, BASE_CURRENCY);

        let mut params = Map::new();
        params.insert("category".to_string(), json!("spot"));
        params.insert("symbol".to_string(), json!(symbol));
        params.insert("side".to_string(), json!("Sell"));
        params.insert("orderType".to_string(), json!("Market"));
        params.insert("qty".to_string(), json!(quantity.to_string()));
        params.insert("marketUnit".to_string(), json!("baseCoin"));

        post_market_order(params, &NetVersion::Mainnet).await?;
    }

    Ok(())
}

pub async fn market_sell(symbol: &str, quantity: f64) -> Result<()> {
    let qty = floor(quantity, ORDER_MAX_DECIMALS);

    let mut params = Map::new();
    params.insert("category".to_string(), json!("spot"));
    params.insert("symbol".to_string(), json!(symbol));
    params.insert("side".to_string(), json!("Sell"));
    params.insert("orderType".to_string(), json!("Market"));
    params.insert("qty".to_string(), json!(qty.to_string()));
    params.insert("marketUnit".to_string(), json!("baseCoin"));
    println!("sell params: {:#?}", params);

    post_market_order(params, &NetVersion::Mainnet).await?;

    Ok(())
}

async fn post_market_order(params: Map<String, Value>, net: &NetVersion) -> Result<()> {
    let client = Client::new();
    let timestamp = get_server_time().await?;
    let recv_window = 5000;
    let api_key = &bybit_key()?;

    let signature = generate_hmac_signature(timestamp, &api_key, recv_window, to_string(&params)?)?;

    let res = client
        .post(bybit_url("/v5/order/create", net))
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
        _ => panic!("Unable to perform market buy"),
    };

    if response.ret_code != 0 {
        return Err(anyhow!(format!(
            "Unable to post market order, error: {}",
            response.ret_msg
        )));
    }

    // println!("Create Response: {:#?}", response);

    Ok(())
}
