use serde::{Deserialize, Serialize};

use super::csv_setup_row::CsvSetupRow;
use crate::{
    models::{candle::Candle, interval::Interval, strategy_orientation::StrategyOrientation},
    resolution_strategies::{atr_resolution::AtrResolution, ResolutionStrategy},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setup {
    pub candle: Candle,
    pub orientation: StrategyOrientation,
    pub ticker: String,
    pub interval: Interval,
    pub resolution_strategy: Option<ResolutionStrategy>,
    pub stop_loss: Option<f64>,
    pub take_profit: Option<f64>,
}

impl Setup {
    #[allow(dead_code)] // TODO: Remove once used
    pub fn dummy() -> Setup {
        let candle = Candle::dummy_data(1, "", 100.0).pop().unwrap();
        let resolution_strategy = ResolutionStrategy::ATR(AtrResolution::new(14, 1.0, 1.0));
        Setup {
            ticker: "DUMMY".to_string(),
            candle,
            interval: Interval::Day1,
            orientation: StrategyOrientation::Long,
            resolution_strategy: Some(resolution_strategy),
            stop_loss: None,
            take_profit: None,
        }
    }

    pub fn to_csv_row(&self) -> CsvSetupRow {
        let resolution = match self.resolution_strategy {
            Some(ref strategy) => strategy.to_string(),
            None => "-".to_string(),
        };

        CsvSetupRow {
            ticker: self.ticker.clone(),
            timestamp: self.candle.timestamp,
            interval: self.interval.clone(),
            orientation: self.orientation,
            stop_loss_resolution: resolution.clone(),
            take_profit_resolution: resolution,
            stop_loss: self.stop_loss.unwrap_or(-1.0),
            take_profit: self.take_profit.unwrap_or(-1.0),
            open: self.candle.open,
            close: self.candle.close,
            high: self.candle.high,
            low: self.candle.low,
            volume: self.candle.volume,
        }
    }
}
