use crate::{
    indicators::{indicator::Indicator, indicator_type::IndicatorType},
    models::interval::Interval,
};
use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub fn new(
        timestamp: DateTime<Utc>,
        open: f64,
        close: f64,
        high: f64,
        low: f64,
        volume: f64,
    ) -> Candle {
        Candle {
            timestamp,
            open,
            close,
            high,
            low,
            volume,
            indicators: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn dummy_from_val(val: f64) -> Candle {
        let now = Utc::now();

        Candle {
            timestamp: now,
            open: val,
            close: val,
            high: val,
            low: val,
            volume: 1000.0,
            indicators: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn from_val(timestamp: DateTime<Utc>, val: f64, volume: f64) -> Candle {
        Candle {
            timestamp,
            open: val,
            close: val,
            high: val,
            low: val,
            volume,
            indicators: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn dummy_from_arr(nums: &[f64]) -> Vec<Candle> {
        let mut now = Utc::now();

        nums.iter()
            .map(|num| {
                now += Duration::days(1);
                let val = *num;
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

    #[allow(dead_code)]
    pub fn dummy_from_increments(nums: &[f64]) -> Vec<Candle> {
        let mut val: f64 = 1000.0;
        let mut now = Utc::now();

        nums.iter()
            .map(|num| {
                val = val.max(val + *num) as f64;
                now += Duration::days(1);
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

    pub fn dyn_dummy_from_prev(candle: &Candle, interval: Interval) -> Candle {
        let prev = candle.close;
        let mut rng = rand::thread_rng();
        let delta = prev / 100.0;
        let wick_delta = delta / 2.0;

        let increment = rng.gen_range(-delta..delta);

        let open = prev;
        let close = prev + increment;
        let high;
        let low;

        if open < close {
            high = open + wick_delta;
            low = close - wick_delta;
        } else {
            high = close + wick_delta;
            low = open - wick_delta;
        }

        Candle {
            timestamp: candle.timestamp + interval.to_duration(),
            open,
            close,
            high,
            low,
            volume: rng.gen_range(200..1500) as f64,
            indicators: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn dyn_dummy_from_increments(nums: &[f64]) -> Vec<Candle> {
        let mut now = Utc::now();
        let mut prev: f64 = 1000.0;
        let mut rng = rand::thread_rng();

        nums.iter()
            .map(|num| {
                let open = prev;
                let close = prev + *num;
                let high;
                let low;
                prev = close;

                if open < close {
                    high = open + 1.0;
                    low = close - 1.0;
                } else {
                    high = close + 1.0;
                    low = open - 1.0;
                }

                now += Duration::days(1);
                Candle {
                    timestamp: now,
                    open,
                    close,
                    high,
                    low,
                    volume: rng.gen_range(200..1500) as f64,
                    indicators: HashMap::new(),
                }
            })
            .collect()
    }

    pub fn dummy_data(n: usize, mode: &str, init_val: f64) -> Vec<Candle> {
        let mut val = init_val;

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

    pub fn clone_indicator(&self, key: &IndicatorType) -> Result<Indicator> {
        self.indicators
            .get(key)
            .context(format!("Unable to find indicator with type: {:#?}", key))
            .cloned()
    }
}
