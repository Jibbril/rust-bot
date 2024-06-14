use actix::Message;

#[derive(Debug, Clone)]
pub struct StopPayload;

impl Message for StopPayload {
    type Result = ();
}
