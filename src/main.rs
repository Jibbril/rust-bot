mod data_sources;
mod utils;

use dotenv::dotenv;
use data_sources::request_data;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    dotenv().ok();
    request_data("BTC").await?;

    Ok(())
}
