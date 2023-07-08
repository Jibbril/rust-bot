mod calculation;
mod data_sources;
mod utils;

use data_sources::{request_data, DataSource};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    request_data(DataSource::AlphaVantage, "BTC").await?;

    Ok(())
}
