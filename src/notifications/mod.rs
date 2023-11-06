mod email;
use self::email::notify_email;
use crate::models::{setups::setup::Setup, traits::trading_strategy::TradingStrategy};
use anyhow::Result;

pub async fn notify(setup: &Setup, strategy: &Box<dyn TradingStrategy>) -> Result<()> {
    notify_email(setup, strategy).await?;

    Ok(())
}
