use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        candle::Candle, generic_result::GenericResult, interval::Interval,
        strategy_orientation::StrategyOrientation, timeseries::TimeSeries,
    },
    resolution_strategies::{atr_resolution::AtrResolution, ResolutionStrategy},
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setup {
    pub ticker: String,
    pub candle: Candle,
    pub interval: Interval,
    pub orientation: StrategyOrientation,
    pub resolution_strategy: ResolutionStrategy,
    pub stop_loss: f64,
    pub take_profit: f64,
}

impl Setup {
    #[allow(dead_code)] // TODO: Remove once used
    pub fn dummy() -> Setup {
        let candle = Candle::dummy_data(1, "", 100.0).pop().unwrap();
        Setup {
            ticker: "DUMMY".to_string(),
            candle,
            interval: Interval::Day1,
            orientation: StrategyOrientation::Long,
            resolution_strategy: ResolutionStrategy::ATR(AtrResolution::new(14, 1.0, 1.0)),
            stop_loss: 0.0,
            take_profit: 0.0,
        }
    }

    pub fn to_csv_row(&self) -> CsvSetupRow {
        CsvSetupRow {
            ticker: self.ticker.clone(),
            timestamp: self.candle.timestamp,
            interval: self.interval.clone(),
            orientation: self.orientation,
            resolution_strategy: self.resolution_strategy.to_string(),
            stop_loss: self.stop_loss,
            take_profit: self.take_profit,
            open: self.candle.open,
            close: self.candle.close,
            high: self.candle.high,
            low: self.candle.low,
            volume: self.candle.volume,
        }
    }
}

pub trait FindsSetups {
    fn find_setups(&self, ts: &TimeSeries) -> GenericResult<Vec<Setup>>;
}

pub trait FindsReverseSetups {
    fn find_reverse_setups(&self, ts: &TimeSeries) -> GenericResult<Vec<Setup>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvSetupRow {
    pub ticker: String,
    pub timestamp: DateTime<Utc>,
    pub interval: Interval,
    pub orientation: StrategyOrientation,
    pub resolution_strategy: String,
    pub stop_loss: f64,
    pub take_profit: f64,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
}
