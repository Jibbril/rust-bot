#[actix::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rust_bot::run_single_indicator().await?;
    // rust_bot::run_strategy().await?;
    // rust_bot::run_historical().await?;

    Ok(())
}
