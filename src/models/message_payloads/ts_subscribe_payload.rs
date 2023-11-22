use actix::{Message, Addr};
use crate::models::setups::setup_finder::SetupFinder;

#[derive(Debug, Clone)]
pub struct TSSubscribePayload {
    pub observer: Addr<SetupFinder>
}

impl Message for TSSubscribePayload {
    type Result = ();
}
