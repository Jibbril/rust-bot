use super::{calculation_mode::CalculationMode, generic_result::GenericResult};
use crate::indicators::{indicator::Indicator, indicator_type::IndicatorType};
use chrono::{DateTime, Duration, Utc};
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
    pub fn dummy_from_arr(nums:&[f64]) -> Vec<Candle> {
        let mut val: f64 = 1000.0;
        let mut now = Utc::now();
        
        nums.iter().map(|num| {
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
        }).collect()
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

    pub fn price_by_mode(&self, mode: &CalculationMode) -> f64 {
        match mode {
            CalculationMode::Close => self.close,
            CalculationMode::Open => self.open,
            CalculationMode::High => self.high,
            CalculationMode::Low => self.low,
        }
    }

    pub fn get_indicator(&self, key: &IndicatorType) -> GenericResult<Indicator> {
        self.indicators
            .get(key)
            .ok_or_else(|| format!("Unable to find indicator with type: {:#?}", key).into())
            .and_then(|indicator| Ok(indicator.clone()))
    }
}
