use crate::models::setups::setup_finder::SetupFinder;
use actix::{Addr, Message};

#[derive(Debug, Clone)]
pub struct TSSubscribePayload {
    pub observer: Addr<SetupFinder>,
}

impl Message for TSSubscribePayload {
    type Result = ();
}
