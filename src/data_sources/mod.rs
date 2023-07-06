mod alpha_vantage;
use alpha_vantage::get;

use crate::utils::generic_result::GenericResult;

pub async fn request_data(symbol: &str) -> GenericResult<()> {
    let data = get(symbol).await?;

    println!("Data:{:#?}", data);

    Ok(())
}
