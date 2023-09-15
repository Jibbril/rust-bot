
#[derive(Clone,Debug)]
pub struct Subscription {
    pub channel: String,
    pub exchange: String,
    pub base: String,
    pub quote: String,
}

impl Subscription {
    pub fn new(channel: &str, exchange: &str, base: &str, quote: &str) -> Self {
        Self {
            channel: channel.to_string(),
            exchange: exchange.to_string(),
            base: base.to_string(),
            quote: quote.to_string(),
        }
    }
}