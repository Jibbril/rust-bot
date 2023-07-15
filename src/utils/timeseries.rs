use crate::indicators::{Indicator, IndicatorType};

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Interval {
    // Hour1,
    // Hour4,
    // Hour12,
    Daily,
    // Weekly,
}

#[derive(Debug, Clone)]
pub struct TimeSeries {
    pub ticker: String,
    pub interval: Interval,
    pub candles: Vec<Candle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,

    #[serde(skip_serializing, skip_deserializing)]
    pub indicators: HashMap<IndicatorType, Indicator>,
}

impl Candle {
    #[allow(dead_code)] // TODO: Remove once used
    pub fn dummy_data(n: usize, mode: &str) -> Vec<Candle> {
        let mut val = 100.0;

        let mut now = Utc::now();

        (0..n)
            .map(|i| {
                now += Duration::days(1);
                val += match mode {
                    "positive" => 10.0,
                    "negative" => -10.0,
                    _ => {
                        if i % 2 == 0 {
                            10.0
                        } else {
                            -10.0
                        }
                    }
                };

                Candle {
                    timestamp: now,
                    open: val,
                    close: val,
                    high: val,
                    low: val,
                    volume: 1000.0,
                    indicators: HashMap::new(),
                }
            })
            .collect()
    }
}
