use serde::{Serialize, Deserialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct OutgoingMessage {
    req_id: Option<String>,
    op: String,
    args: Vec<OutgoingMessageArg>,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct OutgoingMessageArg {
    stream: String, 
    interval: String,
    symbol: String,
}

impl OutgoingMessage {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn ping() -> Self {
        Self {
            req_id: None,
            op: "ping".to_string(),
            args: vec![],
        }
    }

    pub fn new(op: String, args: Vec<OutgoingMessageArg>) -> Self {
        Self {
            req_id: None,
            op,
            args,
        }
    }
}