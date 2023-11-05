mod email;
use anyhow::Result;
use crate::models::{setups::setup::Setup, strategy::Strategy};
use self::email::notify_email;

pub async fn notify(setup: &Setup, strategy: &Strategy) -> Result<()> {
    notify_email(setup, strategy).await?;

    Ok(())
}
