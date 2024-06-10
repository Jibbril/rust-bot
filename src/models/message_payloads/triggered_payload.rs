use actix::Message;

#[derive(Debug, Clone)]
pub struct TriggeredPayload;

impl Message for TriggeredPayload {
    type Result = ();
}
