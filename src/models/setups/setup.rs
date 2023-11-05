use serde::{Deserialize, Serialize};

use crate::{
    models::{candle::Candle, interval::Interval, strategy_orientation::StrategyOrientation},
    resolution_strategies::{atr_resolution::AtrResolution, ResolutionStrategy},
};

use super::csv_setup_row::CsvSetupRow;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setup {
    pub ticker: String,
    pub candle: Candle,
    pub interval: Interval,
    pub orientation: StrategyOrientation,
    pub stop_loss_resolution: ResolutionStrategy,
    pub take_profit_resolution: ResolutionStrategy,
    pub stop_loss: f64,
    pub take_profit: f64,
}

impl Setup {
    #[allow(dead_code)] // TODO: Remove once used
    pub fn dummy() -> Setup {
        let candle = Candle::dummy_data(1, "", 100.0).pop().unwrap();
        let resolution = ResolutionStrategy::ATR(AtrResolution::new(14, 1.0, 1.0));
        Setup {
            ticker: "DUMMY".to_string(),
            candle,
            interval: Interval::Day1,
            orientation: StrategyOrientation::Long,
            stop_loss_resolution: resolution.clone(),
            take_profit_resolution: resolution,
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
            stop_loss_resolution: self.stop_loss_resolution.to_string(),
            take_profit_resolution: self.take_profit_resolution.to_string(),
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
