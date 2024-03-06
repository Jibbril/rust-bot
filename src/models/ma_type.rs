use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum MAType {
    SMA,
    EMA,
    VWMA,
}
