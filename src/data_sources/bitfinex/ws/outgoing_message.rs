#[derive(Debug)]
pub struct OutgoingMessage {
    event: String,
    channel: String,
    key: String,
}

impl OutgoingMessage {
    pub fn new(event: &str, channel: &str, key: &str) -> Self {
        Self {
            event: event.to_string(),
            channel: channel.to_string(),
            key: key.to_string(),
        }
    }
}

impl ToString for OutgoingMessage {
    fn to_string(&self) -> String {
        format!("{{\"event\":\"{}\",\"channel\":\"{}\",\"key\":\"{}\"}}", self.event, self.channel, self.key)
    }
}