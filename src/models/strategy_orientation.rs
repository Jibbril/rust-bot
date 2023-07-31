use std::fmt::{Display, Formatter, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StrategyOrientation {
    Long,
    Short,
}

impl Display for StrategyOrientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            Self::Long => write!(f, "Long"),
            Self::Short => write!(f, "Short"),
        }
    }
}
