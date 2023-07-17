use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum StrategyOrientation {
    Long,
    Short,
    Both,
}

impl Display for StrategyOrientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            Self::Long => write!(f, "Long"),
            Self::Short => write!(f, "Short"),
            Self::Both => write!(f, "Long and Short"),
        }
    }
}
