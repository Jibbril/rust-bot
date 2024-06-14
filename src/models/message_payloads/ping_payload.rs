use actix::Message;

#[derive(Debug, Clone)]
pub struct PingPayload;

impl Message for PingPayload {
    type Result = ();
}
