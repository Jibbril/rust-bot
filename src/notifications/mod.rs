mod email;
use self::email::notify_email;
use crate::models::{setups::setup::Setup, strategy::Strategy};
use anyhow::Result;

pub async fn notify(setup: &Setup, strategy: &Strategy) -> Result<()> {
    notify_email(setup, strategy).await?;

    Ok(())
}
