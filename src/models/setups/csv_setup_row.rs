use crate::models::{interval::Interval, strategy_orientation::StrategyOrientation};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvSetupRow {
    pub symbol: String,
    pub timestamp: DateTime<Utc>,
    pub interval: Interval,
    pub orientation: StrategyOrientation,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
}
