mod data_sources;
mod utils;
mod calculation;

use dotenv::dotenv;
use data_sources::{DataSource,request_data};

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    dotenv().ok();

    request_data(DataSource::AlphaVantage,"BTC").await?;

    Ok(())
}
