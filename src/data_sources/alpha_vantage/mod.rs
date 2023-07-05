use std::env;
use reqwest;
use crate::utils::generic_result::GenericResult;
use crate::utils::timeseries::{TimeSeries,Candle};

pub async fn get(symbol: &str) -> GenericResult<TimeSeries> {
    let function = "DIGITAL_CURRENCY_DAILY";
    let market =  "USD";
    let url = construct_url(function, symbol, market);

    let response = reqwest::get(url).await?;

    match response.status() {
        reqwest::StatusCode::OK => convert_data(response).await,
        _ => Err("Request failed.".into())
    }
}

async fn convert_data(_res: reqwest::Response) -> GenericResult<TimeSeries> {
    // let response = res.text().await?;
    
    Ok(TimeSeries {
        ticker: "BTC".to_string(),
        candles: vec![
            Candle {
                open: 10,
                close: 11,
                high: 12,
                low: 9,
                volume: 24
            }
        ]
    })
}

fn construct_url(
    function: &str, 
    symbol: &str, 
    market: &str
) -> String {
    let key = env::var("ALPHA_VANTAGE_KEY");

    if let Ok(key) = key {
        format!(
            "https://www.alphavantage.co/query?function={}&symbol={}&market={}&apikey={}",
            function,
            symbol,
            market,
            key
        )
    } else {
        panic!("Unable to read Alpha Vantage API key.");
    }
}