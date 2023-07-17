mod email;
use self::email::notify_email;
use crate::{
    models::generic_result::GenericResult,
    trading_strategies::{setup::Setup, strategy::Strategy},
};

pub async fn notify(setup: &Setup, strategy: &Strategy) -> GenericResult<()> {
    notify_email(setup, strategy).await?;

    Ok(())
}
