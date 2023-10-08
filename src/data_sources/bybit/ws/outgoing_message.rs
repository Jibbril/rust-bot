use serde::{Serialize, Deserialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct OutgoingMessage {
    req_id: Option<String>,
    op: String,
    args: Vec<OutgoingMessageArg>,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct OutgoingMessageArg {
    pub stream: String, 
    pub interval: String,
    pub symbol: String,
}

impl OutgoingMessage {
    pub fn to_json(&self) -> String {
        let mut str = "{".to_string();

        if let Some(req_id) = &self.req_id {
            str +="\"req_id\": \"";
            str += req_id;
            str += "\",";
        }

        str +="\"op\": \"";
        str += &self.op;
        str += "\"";

        if self.args.len() > 0 {

            let args = self.args.iter()
                .map(|arg| {
                    format!("\"{}.{}.{}\"", arg.stream, arg.interval, arg.symbol)
                })
                .collect::<Vec<String>>();

            let args = args.join(",");

            str += ", \"args\": [";
            str += &args;
            str += "]";
        }

        str += "}";
        str
    }

    pub fn ping() -> Self {
        Self {
            req_id: None,
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