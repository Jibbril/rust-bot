use dotenv::dotenv;

#[actix::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    // rust_bot::run_setup_finder().await?;
    // rust_bot::run_manual_setups().await?;
    rust_bot::run_single_indicator().await?;
    // rust_bot::run_strategy().await?;
    // rust_bot::run_historical().await?;

    Ok(())
}
