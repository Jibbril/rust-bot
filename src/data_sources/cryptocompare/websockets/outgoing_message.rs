use super::{action::Action, subscription::Subscription};

#[derive(Debug)]
pub struct OutgoingMessage {
    action: Action,
    subs: Vec<Subscription>,
}

impl OutgoingMessage {
    pub fn new(action: Action, subs: Vec<Subscription>) -> Self {
        Self {
            action,
            subs,
        }
    }
}

impl ToString for OutgoingMessage {
    fn to_string(&self) -> String {
        let action = self.action.to_string(); 
        let subs: Vec<String> = self.subs.iter()
            .map(|s| {
                format!("\"{}~{}~{}~{}\"", s.channel, s.exchange, s.base, s.quote)
            })
            .collect();
        let subs = subs.join(","); 

        format!("{{\"action\":\"{}\",\"subs\":[{}]}}", action, subs)
    }
}




