mod email;
use anyhow::Result;

use self::email::notify_email;
use crate::models::{setup::Setup, strategy::Strategy};

pub async fn notify(setup: &Setup, strategy: &Strategy) -> Result<()> {
    notify_email(setup, strategy).await?;

    Ok(())
}
