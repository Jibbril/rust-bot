use lazy_static::lazy_static;
use reqwest::Error;
use serde_json::Value;
use tokio::runtime::Runtime;
use std::env;

lazy_static! {
    static ref URL: String = "https://www.alphavantage.co/query?function=DIGITAL_CURRENCY_DAILY&symbol=BTC&market=CNY&apikey=".to_string();
    static ref ALPHA_KEY: String = env::var("ALPHA_VANTAGE_KEY")
        .unwrap_or("".to_string())
        .to_string(); 
    static ref ALPHA_URL: String = format!("{}{}", *URL, *ALPHA_KEY);
}


pub fn request_data(ticker: &str) -> Result<(), Box<dyn std::error::Error>> {
    //let mut rt = Runtime::new()?;

    println!("{}",*ALPHA_URL);

    Ok(())
}
