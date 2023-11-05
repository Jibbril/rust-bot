use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::models::{interval::Interval, strategy_orientation::StrategyOrientation};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvSetupRow {
    pub ticker: String,
    pub timestamp: DateTime<Utc>,
    pub interval: Interval,
    pub orientation: StrategyOrientation,
    pub stop_loss_resolution: String,
    pub take_profit_resolution: String,
    pub stop_loss: f64,
    pub take_profit: f64,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
}