#[actix::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rust_bot::run().await?;

    Ok(())
}
