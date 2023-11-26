use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutgoingMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    req_id: Option<String>,
    op: String,
    args: Vec<OutgoingMessageArg>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutgoingMessageArg {
    pub stream: String,
    pub interval: String,
    pub symbol: String,
}

impl OutgoingMessage {
    pub fn to_json(&self) -> String {
        let args: Vec<String> = self
            .args
            .iter()
            .map(|arg| format!("{}.{}.{}", arg.stream, arg.interval, arg.symbol))
            .collect();

        let message = json!({
            "op": &self.op,
            "args": args
        });

        // If req_id is Some, add it to the message
        let mut message_map = message.as_object().unwrap().clone();
        if let Some(req_id) = &self.req_id {
            message_map.insert("req_id".to_string(), json!(req_id));
        }

        serde_json::to_string(&message_map).unwrap()
    }

    pub fn ping(req_id: Option<String>) -> Self {
        Self {
            req_id,
            op: "ping".to_string(),
            args: vec![],
        }
    }

    pub fn new(op: &str, args: Vec<OutgoingMessageArg>) -> Self {
        Self {
            req_id: None,
            op: op.to_string(),
            args,
        }
    }
}
