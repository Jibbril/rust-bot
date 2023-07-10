#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = rust_bot::run().await;

    Ok(())
}
